
# === Builder Stage ===

FROM rust:1.80 AS builder

# Install curl, gnupg, ca-certificates needed for NodeSource setup
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    gnupg \
    ca-certificates \
    && mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
       | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_22.x nodistro main" \
       | tee /etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN rustup update stable && rustup default stable

WORKDIR /app
COPY . .

# Build frontend
RUN cd frontend && npm install && npm run build

# Build backend
RUN cd backend && cargo build --release

# === Runtime Stage ===

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    libssl3 \
    ca-certificates \
    nginx \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled backend binary
COPY --from=builder /app/backend/target/release/backend /usr/local/bin/backend

# Copy built frontend assets
COPY --from=builder /app/frontend/dist /app/frontend/dist

# Copy nginx and supervisord configs
COPY docker/nginx.conf /etc/nginx/nginx.conf
COPY docker/supervisord.conf /etc/supervisor/conf.d/qviewng.conf

# Copy backend runtime config and release env
COPY backend/config /app/config
COPY .env.release /app/.env.release

# Create log directory for log4rs
RUN mkdir -p /var/log/qviewng

EXPOSE 80

CMD ["/usr/bin/supervisord", "-n", "-c", "/etc/supervisor/supervisord.conf"]
