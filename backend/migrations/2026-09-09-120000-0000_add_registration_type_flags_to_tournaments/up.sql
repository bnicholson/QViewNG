-- Which registration types a tournament offers. Each gates its tab/button in the registration
-- section of the UI. Default to true so existing tournaments keep all three enabled.
ALTER TABLE tournaments ADD COLUMN use_team_registration BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE tournaments ADD COLUMN use_gear_registration BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE tournaments ADD COLUMN use_volunteer_registration BOOLEAN NOT NULL DEFAULT true;
