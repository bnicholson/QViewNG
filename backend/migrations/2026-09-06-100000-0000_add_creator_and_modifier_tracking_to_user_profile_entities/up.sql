-- Add creator ("Created By") and/or last-modifier ("Last Modified By") tracking to the entities
-- surfaced in the User-profile data tables that did not already track them. Each column is a
-- required FK to users(id).
--
-- Columns are added nullable, backfilled from the row's most sensible existing user, then
-- constrained NOT NULL with the FK added last, so this is safe against databases that already
-- hold rows (e.g. a persistent test DB).

-- teams: already track last_modified_user; add creator_id, attributed to the coach.
ALTER TABLE teams ADD COLUMN creator_id UUID;
UPDATE teams SET creator_id = coachid WHERE creator_id IS NULL;
ALTER TABLE teams ALTER COLUMN creator_id SET NOT NULL;
ALTER TABLE teams ADD CONSTRAINT teams_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES users(id);

-- games: already track last_modified_user; add creator_id, attributed to the quizmaster.
ALTER TABLE games ADD COLUMN creator_id UUID;
UPDATE games SET creator_id = quizmasterid WHERE creator_id IS NULL;
ALTER TABLE games ALTER COLUMN creator_id SET NOT NULL;
ALTER TABLE games ADD CONSTRAINT games_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES users(id);

-- rosters: already track the creator (created_by_userid); add last_modified_user, backfilled from it.
ALTER TABLE rosters ADD COLUMN last_modified_user UUID;
UPDATE rosters SET last_modified_user = created_by_userid WHERE last_modified_user IS NULL;
ALTER TABLE rosters ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE rosters ADD CONSTRAINT rosters_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- equipment (gear items): add both, attributed to the owner of the parent equipment set.
ALTER TABLE equipment ADD COLUMN creator_id UUID;
ALTER TABLE equipment ADD COLUMN last_modified_user UUID;
UPDATE equipment e SET creator_id = es.equipmentownerid
    FROM equipmentsets es WHERE e.equipmentsetid = es.id AND e.creator_id IS NULL;
UPDATE equipment e SET last_modified_user = es.equipmentownerid
    FROM equipmentsets es WHERE e.equipmentsetid = es.id AND e.last_modified_user IS NULL;
ALTER TABLE equipment ALTER COLUMN creator_id SET NOT NULL;
ALTER TABLE equipment ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE equipment ADD CONSTRAINT equipment_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES users(id);
ALTER TABLE equipment ADD CONSTRAINT equipment_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);
