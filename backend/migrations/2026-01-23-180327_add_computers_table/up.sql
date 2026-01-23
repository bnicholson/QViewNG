
CREATE TABLE computers (
    computerid BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*; a unique id for a computer
    brand VARCHAR(64) NOT NULL,
    operating_system VARCHAR(64) NOT NULL,
    quizmachine_version VARCHAR(32) NOT NULL,
    wifi_capabilities VARCHAR(64) NOT NULL,  -- enum options: 'Built-In', 'USB', & 'Other'
    login_username VARCHAR(64) NOT NULL,
    login_password VARCHAR(64) NOT NULL,
    has_vga_out_port BOOLEAN NOT NULL,
    has_dvi_out_port BOOLEAN NOT NULL,
    has_hdmi_out_port BOOLEAN NOT NULL,
    has_display_port_out BOOLEAN NOT NULL,
    has_usb_port BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)
