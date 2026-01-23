
ALTER TABLE questionsandanswers ADD COLUMN was_imported_from_ui BOOLEAN;
ALTER TABLE questionsandanswers ADD COLUMN is_quizstuff_authored_question BOOLEAN;

-- 'default' means is not explicitly assigned to an EquipmentSet the user created
-- Equipment not assigned to an EquipmentSet are automatically assigned to the 'default' one
ALTER TABLE equipmentsets ADD COLUMN is_default BOOLEAN;
