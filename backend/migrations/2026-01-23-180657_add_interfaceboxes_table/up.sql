
CREATE TABLE interfaceboxes (
    id BIGSERIAL PRIMARY KEY,               -- *BIGSERIAL intentional (human-readable)*
    type VARCHAR(64) NOT NULL,              -- enum options: 'Parallel' and 'QBox'
    serial_number VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
