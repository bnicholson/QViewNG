ALTER TABLE tournaments ADD COLUMN registration_open_date DATE;
ALTER TABLE tournaments ADD COLUMN registration_close_date DATE;
-- Preserve intent of existing "open" tournaments by mapping the window to the tournament dates.
UPDATE tournaments
   SET registration_open_date = fromdate,
       registration_close_date = todate
 WHERE registration_is_open = TRUE;
ALTER TABLE tournaments DROP COLUMN registration_is_open;
