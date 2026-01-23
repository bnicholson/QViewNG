
CREATE TABLE equipment (
    id BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*

    -- allow only one of these to have a value per record:
    computerid BIGINT,
    jumppadid BIGINT,
    interfaceboxid BIGINT,
    monitorid BIGINT,
    microphonerecorderid BIGINT,
    projectorid BIGINT,
    powerstripid BIGINT,
    extensioncordid BIGINT,

    misc_note TEXT DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Foreign keys (all nullable)
    CONSTRAINT fk_equipment_computer FOREIGN KEY (computerid) REFERENCES computers(computerid),
    CONSTRAINT fk_equipment_jumppad FOREIGN KEY (jumppadid) REFERENCES jumppads(jumppadid),
    CONSTRAINT fk_equipment_interfacebox FOREIGN KEY (interfaceboxid) REFERENCES interfaceboxes(id),
    CONSTRAINT fk_equipment_monitor FOREIGN KEY (monitorid) REFERENCES monitors(id),
    CONSTRAINT fk_equipment_microphonerecorder
        FOREIGN KEY (microphonerecorderid) REFERENCES microphonerecorders(id),
    CONSTRAINT fk_equipment_projector FOREIGN KEY (projectorid) REFERENCES projectors(id),
    CONSTRAINT fk_equipment_powerstrip FOREIGN KEY (powerstripid) REFERENCES powerstrips(id),
    CONSTRAINT fk_equipment_extensioncord FOREIGN KEY (extensioncordid) REFERENCES extensioncords(id),

    -- Enforce exactly one is non-null
    CONSTRAINT exactly_one_equipment_type CHECK (
        (computerid              IS NOT NULL)::integer +
        (jumppadid               IS NOT NULL)::integer +
        (interfaceboxid          IS NOT NULL)::integer +
        (monitorid               IS NOT NULL)::integer +
        (microphonerecorderid    IS NOT NULL)::integer +
        (projectorid             IS NOT NULL)::integer +
        (powerstripid            IS NOT NULL)::integer +
        (extensioncordid         IS NOT NULL)::integer
        = 1
    )
)
