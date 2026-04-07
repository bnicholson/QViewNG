CREATE TABLE create_tournament_applicants (
    id                    UUID          NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id               UUID          NOT NULL REFERENCES users(id),
    request_context       TEXT,
    status                VARCHAR(64)   NOT NULL,
    created_at            TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    modified_at           TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    last_modified_user_id UUID          NOT NULL REFERENCES users(id)
);
