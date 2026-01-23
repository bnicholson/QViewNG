
-- modification to previous migration:
ALTER TABLE equipment
    ADD COLUMN equipmentsetid BIGINT NOT NULL REFERENCES equipmentsets(id);

CREATE TABLE equipmentregistrations (
    id BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*
    equipmentid BIGINT NOT NULL REFERENCES equipment(id),
    tournamentid UUID NOT NULL REFERENCES tournaments(tid),
    roomid UUID REFERENCES rooms(roomid),
    -- imagined enum options for 'status' column: 
    -- 'Received from Owner', 'Prepared for Assignment', 'Assigned to Room', 'Deployed to Room', 
    -- 'On Standby', 'Returned from Room', 'Needs Repair', 'Returned to Owner'
    status VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(equipmentid,tournamentid) 
)
