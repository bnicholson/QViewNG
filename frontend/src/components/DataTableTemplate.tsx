import { useState, type ReactNode } from "react";

// ─── Shared Sub-components ───────────────────────────────────────────────────

export function BoolBadge({ value }: { value: boolean }) {
  return (
    <span
      style={{
        display: "inline-block",
        padding: "2px 10px",
        borderRadius: 12,
        fontSize: 12,
        fontWeight: 600,
        letterSpacing: "0.03em",
        background: value ? "#e6f4ea" : "#fce8e6",
        color: value ? "#1e7e34" : "#c0392b",
        border: `1px solid ${value ? "#a8d5b5" : "#f5c6cb"}`,
      }}
    >
      {value ? "Yes" : "No"}
    </span>
  );
}

function DeleteButton({ onDelete }: { onDelete: () => Promise<void> }) {
  const [confirming, setConfirming] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleClick = async (): Promise<void> => {
    if (!confirming) {
      setConfirming(true);
      setError(null);
      return;
    }
    setDeleting(true);
    try {
      await onDelete();
    } catch (err: any) {
      setError(err?.message ?? "Delete failed");
      setConfirming(false);
    } finally {
      setDeleting(false);
    }
  };

  return (
    <div style={{ display: "flex", gap: 6, alignItems: "center" }}>
      <button
        onClick={handleClick}
        disabled={deleting}
        style={{
          padding: "3px 10px",
          borderRadius: 5,
          border: `1px solid ${confirming ? "#c0392b" : "#e0e0e0"}`,
          background: confirming ? "#c0392b" : "transparent",
          color: confirming ? "#fff" : "#c0392b",
          fontSize: 12,
          fontWeight: 600,
          cursor: deleting ? "not-allowed" : "pointer",
          opacity: deleting ? 0.6 : 1,
          transition: "all .15s",
          whiteSpace: "nowrap",
        }}
      >
        {deleting ? "Deleting…" : confirming ? "Confirm" : "Delete"}
      </button>
      {confirming && !deleting && (
        <button
          onClick={() => { setConfirming(false); setError(null); }}
          style={{
            padding: "3px 8px",
            borderRadius: 5,
            border: "1px solid #e0e0e0",
            background: "transparent",
            color: "#555",
            fontSize: 12,
            cursor: "pointer",
          }}
        >
          Cancel
        </button>
      )}
      {error && (
        <span style={{ fontSize: 12, color: "#c0392b" }}>{error}</span>
      )}
    </div>
  );
}

// ─── DataTable ────────────────────────────────────────────────────────────────

export interface ColumnDef<T> {
  /** Header label shown in the <thead> */
  header: string;
  /** Renders the cell content for a given row */
  render: (row: T) => ReactNode;
}

export interface DataTableProps<T> {
  /** Singular entity name used in the toolbar ("Division", "Room", "Round") */
  entityLabel: string;
  /** href for the "Create <entityLabel>" button — use this OR onCreate, not both */
  createHref?: string;
  /** onClick handler for the "Create <entityLabel>" button — use this OR createHref, not both */
  onCreate?: () => void;
  /** Column definitions (header + cell renderer). Do NOT include the delete column — it is added automatically. */
  columns: ColumnDef<T>[];
  /** Current row data */
  rows: T[];
  /** Extracts a stable React key from each row */
  getId: (row: T) => string | number;
  /** Called with the row whose deletion was confirmed. Should call the API and update state; may throw on failure. */
  onDelete: (row: T) => Promise<void>;
}

export function DataTableTemplate<T>({
  entityLabel,
  createHref,
  onCreate,
  columns,
  rows,
  getId,
  onDelete,
}: DataTableProps<T>) {
  // +1 for the delete column
  const colSpan = columns.length + 1;

  return (
    <div style={{ fontFamily: "inherit" }}>
      {/* ── Toolbar ── */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginBottom: 16,
        }}
      >
        <div>
          <h2 style={{ margin: 0, fontSize: 20, fontWeight: 600, letterSpacing: "-0.01em" }}>
            {entityLabel}s
          </h2>
          <p style={{ margin: "2px 0 0", fontSize: 13, color: "#666" }}>
            {rows.length} {entityLabel.toLowerCase()}
            {rows.length !== 1 ? "s" : ""}
          </p>
        </div>

        {onCreate ? (
          <button
            onClick={onCreate}
            style={{
              display: "inline-flex",
              alignItems: "center",
              gap: 6,
              padding: "8px 16px",
              borderRadius: 7,
              background: "#2563eb",
              color: "#fff",
              border: "none",
              cursor: "pointer",
              fontSize: 14,
              fontWeight: 600,
              boxShadow: "0 1px 3px rgba(37,99,235,.25)",
              transition: "background .15s",
            }}
            onMouseEnter={(e) => (e.currentTarget.style.background = "#1d4ed8")}
            onMouseLeave={(e) => (e.currentTarget.style.background = "#2563eb")}
          >
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <path d="M7 1v12M1 7h12" stroke="#fff" strokeWidth="2" strokeLinecap="round" />
            </svg>
            Create {entityLabel}
          </button>
        ) : (
          <a
            href={createHref}
            style={{
              display: "inline-flex",
              alignItems: "center",
              gap: 6,
              padding: "8px 16px",
              borderRadius: 7,
              background: "#2563eb",
              color: "#fff",
              textDecoration: "none",
              fontSize: 14,
              fontWeight: 600,
              boxShadow: "0 1px 3px rgba(37,99,235,.25)",
              transition: "background .15s",
            }}
            onMouseEnter={(e) => (e.currentTarget.style.background = "#1d4ed8")}
            onMouseLeave={(e) => (e.currentTarget.style.background = "#2563eb")}
          >
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <path d="M7 1v12M1 7h12" stroke="#fff" strokeWidth="2" strokeLinecap="round" />
            </svg>
            Create {entityLabel}
          </a>
        )}
      </div>

      {/* ── Table ── */}
      <div style={{ overflowX: "auto", borderRadius: 10, border: "1px solid #e5e7eb" }}>
        <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 14 }}>
          <thead>
            <tr style={{ background: "#f9fafb", borderBottom: "1px solid #e5e7eb" }}>
              {columns.map((col) => (
                <th
                  key={col.header}
                  style={{
                    padding: "10px 14px",
                    textAlign: "center",
                    fontWeight: 600,
                    fontSize: 12,
                    color: "#6b7280",
                    letterSpacing: "0.05em",
                    textTransform: "uppercase",
                    whiteSpace: "nowrap",
                  }}
                >
                  {col.header}
                </th>
              ))}
              {/* Empty header for the delete column */}
              <th style={{ padding: "10px 14px" }} />
            </tr>
          </thead>
          <tbody>
            {rows.length === 0 ? (
              <tr>
                <td
                  colSpan={colSpan}
                  style={{ padding: "32px 14px", textAlign: "center", color: "#9ca3af" }}
                >
                  No {entityLabel.toLowerCase()}s found.
                </td>
              </tr>
            ) : (
              rows.map((row, i) => (
                <tr
                  key={getId(row)}
                  style={{
                    background: i % 2 === 0 ? "#fff" : "#fafafa",
                    borderBottom: "1px solid #f3f4f6",
                    transition: "background .1s",
                  }}
                  onMouseEnter={(e) => (e.currentTarget.style.background = "#f0f7ff")}
                  onMouseLeave={(e) =>
                    (e.currentTarget.style.background = i % 2 === 0 ? "#fff" : "#fafafa")
                  }
                >
                  {columns.map((col) => (
                    <td key={col.header} style={{ padding: "12px 14px", color: "#374151" }}>
                      {col.render(row)}
                    </td>
                  ))}
                  <td style={{ padding: "12px 14px", whiteSpace: "nowrap" }}>
                    <DeleteButton onDelete={() => onDelete(row)} />
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
