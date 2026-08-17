import { useRef, useState } from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Typography from "@mui/material/Typography";
import Alert from "@mui/material/Alert";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import CircularProgress from "@mui/material/CircularProgress";
import { GameEventImportAPI, type ImportPreview, type GameRef, type GameImportError, type ImportableGame } from "../features/GameEventImportAPI";

function gameLabel(g: GameRef): string {
  return `${g.division} · Room ${g.room} · Round ${g.round}`;
}

function Section({ title, count, children }: { title: string; count: number; children: React.ReactNode }) {
  return (
    <Box sx={{ mt: 2 }}>
      <Typography sx={{ fontSize: 13, fontWeight: 700, color: "#374151", textTransform: "uppercase", letterSpacing: "0.04em" }}>
        {title} ({count})
      </Typography>
      <Box sx={{ mt: 0.5 }}>{children}</Box>
    </Box>
  );
}

export default function ImportGameEventsButton({ tid, onImported }: { tid: string; onImported?: () => void }) {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [open, setOpen] = useState(false);
  const [csvText, setCsvText] = useState("");
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const [loading, setLoading] = useState(false);   // preview in flight
  const [importing, setImporting] = useState(false); // commit in flight
  const [committed, setCommitted] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const openFilePicker = () => fileInputRef.current?.click();

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    // Reset the input so selecting the same file again re-triggers change.
    e.target.value = "";
    if (!file) return;

    setOpen(true);
    setPreview(null);
    setCommitted(false);
    setError(null);

    if (!file.name.toLowerCase().endsWith(".csv")) {
      setError("Only .csv files are accepted.");
      return;
    }

    setLoading(true);
    try {
      const text = await file.text();
      setCsvText(text);
      const p = await GameEventImportAPI.preview(tid, text);
      setPreview(p);
    } catch (err: any) {
      setError(err?.message ?? "Failed to read or analyze the file.");
    } finally {
      setLoading(false);
    }
  };

  const handleImport = async () => {
    setImporting(true);
    setError(null);
    try {
      const result = await GameEventImportAPI.commit(tid, csvText);
      setPreview(result);
      setCommitted(true);
    } catch (err: any) {
      setError(err?.message ?? "Import failed.");
    } finally {
      setImporting(false);
    }
  };

  const handleClose = () => {
    if (importing) return;
    setOpen(false);
    // Refresh the underlying table only when data was actually imported.
    if (committed) {
      setCommitted(false);
      onImported?.();
    }
  };

  const canImport = !!preview && !loading && !importing && !committed && preview.importable.length > 0;

  return (
    <>
      <Button variant="outlined" size="small" onClick={openFilePicker}>
        Import (SneakerNet)
      </Button>
      <input
        ref={fileInputRef}
        type="file"
        accept=".csv"
        hidden
        onChange={handleFileChange}
      />

      <Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
        <DialogTitle>Import Game Events</DialogTitle>
        <DialogContent dividers>
          {error && <Alert severity="error" sx={{ mb: 1 }}>{error}</Alert>}

          {loading && (
            <Stack direction="row" spacing={1.5} alignItems="center" sx={{ py: 2 }}>
              <CircularProgress size={20} />
              <Typography variant="body2" color="text.secondary">Analyzing file…</Typography>
            </Stack>
          )}

          {committed && (
            <Alert severity="success" sx={{ mb: 1 }}>
              Imported game events for {preview?.importable.length ?? 0} game
              {(preview?.importable.length ?? 0) === 1 ? "" : "s"}.
            </Alert>
          )}

          {preview && !loading && (
            <>
              <Section title={committed ? "Imported" : "Games to import"} count={preview.importable.length}>
                {preview.importable.length === 0 ? (
                  <Typography variant="body2" color="text.secondary">No games will be imported.</Typography>
                ) : (
                  preview.importable.map((g: ImportableGame) => (
                    <Typography key={g.gid} variant="body2">
                      {gameLabel(g)} — {g.event_count} events
                    </Typography>
                  ))
                )}
              </Section>

              {preview.errors.length > 0 && (
                <Section title="Errors" count={preview.errors.length}>
                  {preview.errors.map((e: GameImportError, i) => (
                    <Typography key={i} variant="body2" color="error">
                      {gameLabel(e)} — {e.message}
                    </Typography>
                  ))}
                </Section>
              )}

              {preview.games_not_found.length > 0 && (
                <Section title="Games not found" count={preview.games_not_found.length}>
                  {preview.games_not_found.map((g, i) => (
                    <Typography key={i} variant="body2" color="warning.main">
                      {gameLabel(g)} — no matching game in this tournament
                    </Typography>
                  ))}
                </Section>
              )}

              {preview.missing_games.length > 0 && (
                <Section title="Missing game events" count={preview.missing_games.length}>
                  {preview.missing_games.map((g, i) => (
                    <Typography key={i} variant="body2" color="text.secondary">
                      {gameLabel(g)} — exists in this room but has no events in the file
                    </Typography>
                  ))}
                </Section>
              )}
            </>
          )}
        </DialogContent>
        <DialogActions>
          {committed ? (
            <Button onClick={handleClose}>Close</Button>
          ) : (
            <>
              <Button onClick={handleClose} disabled={importing}>Cancel</Button>
              <Button variant="contained" onClick={handleImport} disabled={!canImport}>
                {importing ? <CircularProgress size={20} color="inherit" /> : "Import"}
              </Button>
            </>
          )}
        </DialogActions>
      </Dialog>
    </>
  );
}
