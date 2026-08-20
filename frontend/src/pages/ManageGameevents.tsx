import { useState, useCallback, type ReactNode } from "react";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from "../components/DataTableTemplate";
import { GameAPI, type GameEventTS } from "../features/GameAPI";

function formatDateTime(iso: string): string {
  if (!iso) return "—";
  const d = new Date(iso);
  return isNaN(d.getTime()) ? iso : d.toLocaleString();
}

// Columns use the UI names provided; each maps to a field on the gameevents record.
const COLUMNS: ColumnDef<GameEventTS>[] = [
  { header: "Question #", render: (r) => r.question },
  { header: "Event #", render: (r) => r.eventnum },
  { header: "Event", render: (r) => r.event },
  { header: "Name", render: (r) => r.name },
  { header: "Team #", render: (r) => r.team },
  { header: "Seat #", render: (r) => r.quizzer },
  { header: "Client TS", render: (r) => formatDateTime(r.clientts) },
  { header: "Server TS", render: (r) => formatDateTime(r.serverts) },
  { header: "QM Reg. Key", render: (r) => r.qm_registration_key ?? "" },
];

export const ManageGameevents = () => {
  const [gameId, setGameId] = useState("");
  // The Game ID actually used for the current results (set on "View Gameevents").
  const [submittedGameId, setSubmittedGameId] = useState<string | null>(null);
  const [rows, setRows] = useState<GameEventTS[] | null>(null);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadPage = useCallback((gid: string, p: number, ps: number) => {
    setLoading(true);
    setError(null);
    GameAPI.getGameevents(gid, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        // The endpoint returns a bare array (no count); estimate total so paging works.
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setRows(result);
      })
      .catch((err: any) => {
        setRows(null);
        setError(err?.message ?? "Failed to load game events.");
      })
      .finally(() => setLoading(false));
  }, []);

  const handleView = () => {
    const trimmed = gameId.trim();
    if (!trimmed) {
      setError("Please enter a Game ID.");
      return;
    }
    setSubmittedGameId(trimmed);
    loadPage(trimmed, 0, pageSize);
  };

  const handlePageChange = useCallback((newPage: number) => {
    if (submittedGameId) loadPage(submittedGameId, newPage, pageSize);
  }, [submittedGameId, pageSize, loadPage]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (!submittedGameId) return;
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setRows((prev) => (prev ? prev.slice(0, newSize) : prev));
    } else {
      loadPage(submittedGameId, 0, newSize);
    }
  }, [submittedGameId, pageSize, page, loadPage]);

  return (
    <Stack spacing={3}>
      {/* ── Query controls ── */}
      {/* pt gives the outlined field's floating label room so a parent's overflow clip doesn't cut it off */}
      <Stack direction="row" spacing={2} alignItems="center" flexWrap="wrap" useFlexGap sx={{ pt: 1 }}>
        <TextField
          size="small"
          label="Game ID"
          value={gameId}
          onChange={(e) => setGameId(e.target.value)}
          onKeyDown={(e) => { if (e.key === "Enter") handleView(); }}
          sx={{ minWidth: 340 }}
        />
        <Button variant="contained" onClick={handleView} disabled={loading}>
          {loading ? "Loading…" : "View Gameevents"}
        </Button>
      </Stack>

      {error && (
        <Typography variant="body2" color="error">
          {error}
        </Typography>
      )}

      {/* Table renders only after a lookup has been performed. */}
      {rows !== null && (
        <DataTableTemplate<GameEventTS>
          entityLabel="Game Event"
          showCreateButton={false}
          showDeleteButton={false}
          columns={COLUMNS}
          rows={rows}
          totalCount={totalCount}
          getId={(r) => `${r.gid}-${r.question}-${r.eventnum}`}
          onDelete={async () => {}}
          page={page}
          pageSize={pageSize}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      )}
    </Stack>
  );
};
