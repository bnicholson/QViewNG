// Utilities to export a table (all rows, not just the visible page) to CSV, XLSX, or PDF.
// CSV is fully native. XLSX/PDF dynamically import optional libraries; run:
//   npm install xlsx jspdf jspdf-autotable
// (CSV export works without them.)

export interface ExportPayload {
  filename: string; // base name, without extension
  columns: string[];
  rows: (string | number)[][];
}

function triggerDownload(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

function csvCell(value: string | number): string {
  const s = value === null || value === undefined ? "" : String(value);
  return /[",\n\r]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s;
}

export function exportCsv(p: ExportPayload): void {
  const lines = [
    p.columns.map(csvCell).join(","),
    ...p.rows.map((row) => row.map(csvCell).join(",")),
  ];
  const blob = new Blob(["﻿" + lines.join("\r\n")], { type: "text/csv;charset=utf-8;" });
  triggerDownload(blob, `${p.filename}.csv`);
}

export async function exportXlsx(p: ExportPayload): Promise<void> {
  // @ts-ignore optional dependency — run `npm install xlsx`
  const XLSX = await import("xlsx");
  const worksheet = XLSX.utils.aoa_to_sheet([p.columns, ...p.rows]);
  const workbook = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(workbook, worksheet, "Data");
  XLSX.writeFile(workbook, `${p.filename}.xlsx`);
}

export async function exportPdf(p: ExportPayload): Promise<void> {
  // @ts-ignore optional dependency — run `npm install jspdf`
  const { jsPDF } = await import("jspdf");
  // @ts-ignore optional dependency — run `npm install jspdf-autotable`
  const autoTable = (await import("jspdf-autotable")).default;
  const doc = new jsPDF({ orientation: p.columns.length > 8 ? "landscape" : "portrait" });
  autoTable(doc, {
    head: [p.columns],
    body: p.rows.map((row) => row.map((c) => (c === null || c === undefined ? "" : String(c)))),
    styles: { fontSize: 8 },
    headStyles: { fillColor: [37, 99, 235] },
  });
  doc.save(`${p.filename}.pdf`);
}

export type ExportFormat = "csv" | "xlsx" | "pdf";

export async function exportTable(format: ExportFormat, p: ExportPayload): Promise<void> {
  if (format === "csv") return exportCsv(p);
  if (format === "xlsx") return exportXlsx(p);
  return exportPdf(p);
}
