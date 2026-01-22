
CREATE TABLE equipmentsets (
    id BIGSERIAL PRIMARY KEY,  -- *BIGSERIAL intentional (human-readable)*; a unique id for an equipment set
    equipmentownerid UUID NOT NULL REFERENCES users(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
