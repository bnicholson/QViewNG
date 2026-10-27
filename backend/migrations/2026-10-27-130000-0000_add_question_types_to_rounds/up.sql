-- The question type of each of a round's 20 questions, for stats generation.
--
-- Stored as exactly 20 single-letter question type codes, one per question in question order:
--   G General, A According-to, Q Quote, R Reference, V Verse, X Context, S Situation,
--   I In what Book and Chapter
-- M (Memory Verse) is inferred from one each of R, V and Q and is never stored.
-- An empty string means "no question types saved for this round". Storage is all-or-nothing,
-- which the CHECK enforces: a round has either no types or a type for every question.
-- Existing rows are backfilled with '' (no types), which satisfies the CHECK.
--
--   e.g.  'GARGGVGAGGQGAGXGSAGG'
ALTER TABLE rounds
    ADD COLUMN question_types TEXT NOT NULL DEFAULT ''
        CONSTRAINT rounds_question_types_format
        CHECK (question_types = '' OR question_types ~ '^[GAQRVXSI]{20}$');

-- When true (the default), saving question types checks that the round has the standard count of
-- each type. The user can turn it off for a round to save any mix (e.g. only G questions).
ALTER TABLE rounds
    ADD COLUMN using_standard_question_types BOOLEAN NOT NULL DEFAULT TRUE;

-- Importing question types from a question set file. Granted to the roles that already manage
-- rounds: tournament_manager, tournament_admin and super_user.
INSERT INTO permissions (name, resource, action) VALUES
    ('round:upload_question_types', 'round', 'upload_question_types')
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name IN ('super_user', 'tournament_manager', 'tournament_admin')
  AND p.name = 'round:upload_question_types'
ON CONFLICT DO NOTHING;
