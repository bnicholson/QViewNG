
-- modifying previous migration:
ALTER TABLE monitors DROP COLUMN misc_notes;  -- will be included in 'Equipment' table soon enough

CREATE TABLE microphonerecorders (
    id BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*
    type VARCHAR(64) NOT NULL,       -- enum options: 'External' and 'Built-In'
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)
