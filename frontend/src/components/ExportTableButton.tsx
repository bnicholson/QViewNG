import { useState } from "react";
import Button from "@mui/material/Button";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import { exportTable, type ExportFormat, type ExportPayload } from "../features/exportTable";

export default function ExportTableButton({
  getPayload,
  disabled,
}: {
  getPayload: () => ExportPayload | null;
  disabled?: boolean;
}) {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  const openMenu = (e: React.MouseEvent<HTMLButtonElement>) => setAnchorEl(e.currentTarget);
  const closeMenu = () => setAnchorEl(null);

  const run = async (format: ExportFormat) => {
    closeMenu();
    const payload = getPayload();
    if (!payload || payload.rows.length === 0) return;
    try {
      await exportTable(format, payload);
    } catch (err) {
      console.error("Export failed", err);
      const hint =
        format === "csv"
          ? ""
          : "\n\nExcel/PDF export requires the export libraries. Run:\n  npm install xlsx jspdf jspdf-autotable";
      alert(`Export failed.${hint}`);
    }
  };

  return (
    <>
      <Button variant="outlined" size="small" onClick={openMenu} disabled={disabled}>
        Export
      </Button>
      <Menu anchorEl={anchorEl} open={Boolean(anchorEl)} onClose={closeMenu}>
        <MenuItem onClick={() => run("csv")}>CSV (.csv)</MenuItem>
        <MenuItem onClick={() => run("xlsx")}>Excel (.xlsx)</MenuItem>
        <MenuItem onClick={() => run("pdf")}>PDF (.pdf)</MenuItem>
      </Menu>
    </>
  );
}
