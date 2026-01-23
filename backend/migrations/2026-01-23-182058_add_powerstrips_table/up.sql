
CREATE TABLE powerstrips (
    id BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*
    make VARCHAR(64) NOT NULL,
    model VARCHAR(64) NOT NULL,
    color VARCHAR(64) NOT NULL,
    num_of_plugs INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)
