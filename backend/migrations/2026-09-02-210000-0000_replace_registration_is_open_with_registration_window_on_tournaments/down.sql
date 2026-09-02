ALTER TABLE tournaments ADD COLUMN registration_is_open BOOLEAN NOT NULL DEFAULT FALSE;
-- Reconstruct the boolean from whether today falls within the (inclusive) registration window.
UPDATE tournaments
   SET registration_is_open = TRUE
 WHERE registration_open_date IS NOT NULL
   AND registration_close_date IS NOT NULL
   AND CURRENT_DATE BETWEEN registration_open_date AND registration_close_date;
ALTER TABLE tournaments DROP COLUMN registration_open_date;
ALTER TABLE tournaments DROP COLUMN registration_close_date;
