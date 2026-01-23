
CREATE TABLE monitors (
    id BIGSERIAL PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*
    size VARCHAR(64) NOT NULL,       -- enum options: '15-inch', '17-inch', '19-inch', '>19-inch'
    brand VARCHAR(64) NOT NULL,
    has_vga_out_port BOOLEAN NOT NULL,
    has_dvi_out_port BOOLEAN NOT NULL,
    has_hdmi_out_port BOOLEAN NOT NULL,
    has_display_port_out BOOLEAN NOT NULL,
    misc_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)
