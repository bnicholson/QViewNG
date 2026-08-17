import { useEffect, useMemo, useState } from "react";
import Box from "@mui/material/Box";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select, { type SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import Alert from "@mui/material/Alert";
import { DataTableTemplate, type ColumnDef } from "./DataTableTemplate";
import { GameAPI } from "../features/GameAPI";
import { RoomAPI } from "../features/RoomAPI";
import { RoundAPI } from "../features/RoundAPI";
import { StatsGroupAPI, type StatsGroupTS, type TeamStatTS, type IndividualStatTS } from "../features/StatsGroupAPI";
import { DivisionAPI } from "../features/DivisionAPI";
import ImportGameEventsButton from "./ImportGameEventsButton";
import ExportTableButton from "./ExportTableButton";
import type { ExportPayload } from "../features/exportTable";

// One row per Game of the tournament (mirrors the Room Monitor columns).
interface GameSelectionRow {
  gid: string;
  division: string;
  room: string;
  round: string;
  question: string;
  done: string;   // "Yes" | "No" — placeholder until game readiness is wired to real data
  dataOk: string; // "Yes" | "No" — placeholder
  information: string;
}

// ─── Filter option definitions ────────────────────────────────────────────────

type DataView = "games" | "team" | "individual";

const DATA_OPTIONS: { value: DataView; label: string }[] = [
  { value: "games", label: "Games Selection" },
  { value: "team", label: "Team Stats" },
  { value: "individual", label: "Individual Stats" },
];

// ─── Content sections ─────────────────────────────────────────────────────────

function GamesSelectionSection({ tid, statsGroupId, refreshKey, onExportReady }: { tid: string; statsGroupId: string; refreshKey: number; onExportReady: (p: ExportPayload) => void }) {
  const [rows, setRows] = useState<GameSelectionRow[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    let cancelled = false;
    Promise.all([
      GameAPI.getByTournament(tid, 0, 500),
      RoomAPI.getByTournament(tid, 0, 500),
      RoundAPI.getByTournament(tid, 0, 500),
      DivisionAPI.getByTournament(tid, 0, 500),
      GameAPI.getStatuses(tid),
      statsGroupId ? StatsGroupAPI.getGames(statsGroupId, 0, 500) : Promise.resolve([]),
    ])
      .then(([gamesResult, rooms, rounds, divisions, statuses, groupGames]) => {
        if (cancelled) return;
        const roomNames = new Map(rooms.map((r) => [r.roomid, r.name]));
        const roundNames = new Map(rounds.map((r) => [r.roundid, r.name]));
        const divisionNames = new Map(divisions.map((d) => [d.did, d.dname]));
        const statusByGid = new Map(statuses.map((s) => [s.gid, s]));
        setRows(
          gamesResult.items.map((g) => {
            const status = statusByGid.get(g.gid);
            return {
              gid: g.gid,
              division: divisionNames.get(g.divisionid) ?? g.divisionid,
              room: roomNames.get(g.roomid) ?? g.roomid,
              round: roundNames.get(g.roundid) ?? g.roundid,
              question: "—",
              done: status?.done ? "Yes" : "No",
              dataOk: status?.data_ok ? "Yes" : "No",
              information: "",
            };
          })
        );
        // Pre-select the games that are already in the selected stats group.
        setSelected(new Set(groupGames.map((g) => g.gid)));
      })
      .catch(() => console.error("Failed to load games for stats group selection"));
    return () => {
      cancelled = true;
    };
  }, [tid, statsGroupId, refreshKey]);

  // Publish the full table (all rows) for export, reflecting live selection state.
  useEffect(() => {
    onExportReady({
      filename: "game-selection",
      columns: ["Selected", "Division", "Room", "Round", "Question", "Done", "DataOk", "Information"],
      rows: rows.map((r) => [
        selected.has(r.gid) ? "Yes" : "No",
        r.division,
        r.room,
        r.round,
        r.question,
        r.done,
        r.dataOk,
        r.information,
      ]),
    });
  }, [rows, selected, onExportReady]);

  const toggleRow = (gid: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(gid)) next.delete(gid);
      else next.add(gid);
      return next;
    });
  };

  // A game isn't ready to include if it isn't Done or its data isn't OK.
  const notReady = rows.filter((r) => r.done !== "Yes" || r.dataOk !== "Yes");

  const columns: ColumnDef<GameSelectionRow>[] = [
    {
      header: "Selected",
      render: (r) => (
        <input
          type="checkbox"
          checked={selected.has(r.gid)}
          onChange={() => toggleRow(r.gid)}
          style={{ cursor: "pointer" }}
        />
      ),
    },
    { header: "Division", render: (r) => r.division },
    { header: "Room", render: (r) => r.room },
    { header: "Round", render: (r) => r.round },
    { header: "Question", render: (r) => r.question },
    { header: "Done", render: (r) => r.done },
    { header: "DataOk", render: (r) => r.dataOk },
    { header: "Information", render: (r) => r.information },
  ];

  return (
    <Stack spacing={2}>
      {notReady.length > 0 && (
        <Alert severity="warning">
          {notReady.length} games are not ready to include either because they are not done or they are
          missing data. Resend data for Games missing data from the Room Monitor page before including them.
        </Alert>
      )}

      <DataTableTemplate<GameSelectionRow>
        entityLabel="Game"
        showCreateButton={false}
        showDeleteButton={false}
        dense
        columns={columns}
        rows={rows}
        totalCount={rows.length}
        getId={(r) => r.gid}
        onDelete={async () => {}}
        page={0}
        pageSize={rows.length || 1}
        onPageChange={() => {}}
        onPageSizeChange={() => {}}
      />
    </Stack>
  );
}

function TeamStatsSection({ statsGroupId, onExportReady }: { statsGroupId: string; onExportReady: (p: ExportPayload) => void }) {
  const [rows, setRows] = useState<TeamStatTS[]>([]);

  useEffect(() => {
    if (!statsGroupId) {
      setRows([]);
      return;
    }
    let cancelled = false;
    StatsGroupAPI.getTeamStats(statsGroupId)
      .then((result) => {
        if (!cancelled) setRows(result);
      })
      .catch(() => {
        if (!cancelled) setRows([]);
      });
    return () => {
      cancelled = true;
    };
  }, [statsGroupId]);

  useEffect(() => {
    onExportReady({
      filename: "team-stats",
      columns: ["Place", "Name", "# Games", "Wins", "Losses", "Olympic Points", "Mod. Olympic Points", "Total Points", "Tie Breaker (manual)"],
      rows: rows.map((r) => [r.place, r.name, r.games, r.wins, r.losses, r.olympic_points, r.mod_olympic_points, r.total_points, r.tie_breaker || ""]),
    });
  }, [rows, onExportReady]);

  const columns: ColumnDef<TeamStatTS>[] = [
    { header: "Place", render: (r) => r.place },
    { header: "Name", render: (r) => <span style={{ whiteSpace: "nowrap" }}>{r.name}</span> },
    { header: "# Games", render: (r) => r.games },
    { header: "Wins", render: (r) => r.wins },
    { header: "Losses", render: (r) => r.losses },
    { header: "Olympic Points", render: (r) => r.olympic_points },
    { header: "Mod. Olympic Points", render: (r) => r.mod_olympic_points },
    { header: "Total Points", render: (r) => r.total_points },
    { header: "Tie Breaker (manual)", render: (r) => r.tie_breaker || "—" },
  ];

  return (
    <DataTableTemplate<TeamStatTS>
      entityLabel="Team"
      showCreateButton={false}
      showDeleteButton={false}
      dense
      columns={columns}
      rows={rows}
      totalCount={rows.length}
      getId={(r) => r.name}
      onDelete={async () => {}}
      page={0}
      pageSize={rows.length || 1}
      onPageChange={() => {}}
      onPageSizeChange={() => {}}
    />
  );
}

function IndividualStatsSection({ statsGroupId, onExportReady }: { statsGroupId: string; onExportReady: (p: ExportPayload) => void }) {
  const [rows, setRows] = useState<IndividualStatTS[]>([]);

  useEffect(() => {
    if (!statsGroupId) {
      setRows([]);
      return;
    }
    let cancelled = false;
    StatsGroupAPI.getIndividualStats(statsGroupId)
      .then((result) => {
        if (!cancelled) setRows(result);
      })
      .catch(() => {
        if (!cancelled) setRows([]);
      });
    return () => {
      cancelled = true;
    };
  }, [statsGroupId]);

  useEffect(() => {
    onExportReady({
      filename: "individual-stats",
      columns: [
        "Place", "Individual", "Team Name", "# Games", "Score", "Avg", "Correct", "Errors", "Bonus Pts", "Bonus Attempts",
        "Errs 16+/5+", "Generals", "Memory", "According", "Context", "Special",
      ],
      rows: rows.map((r) => [
        r.place, r.individual, r.team_name, r.games, r.score, r.avg.toFixed(1), r.correct, r.errors, r.bonus_pts, r.bonus_attempts,
        "", "", "", "", "", "",
      ]),
    });
  }, [rows, onExportReady]);

  const blank = () => "";
  const columns: ColumnDef<IndividualStatTS>[] = [
    { header: "Place", render: (r) => r.place },
    { header: "Individual", render: (r) => <span style={{ whiteSpace: "nowrap" }}>{r.individual}</span> },
    { header: "Team Name", render: (r) => <span style={{ whiteSpace: "nowrap" }}>{r.team_name}</span> },
    { header: "# Games", render: (r) => r.games },
    { header: "Score", render: (r) => r.score },
    { header: "Avg", render: (r) => r.avg.toFixed(1) },
    { header: "Correct", render: (r) => r.correct },
    { header: "Errors", render: (r) => r.errors },
    { header: "Bonus Pts", render: (r) => r.bonus_pts },
    { header: "Bonus Attempts", render: (r) => r.bonus_attempts },
    // Remaining detail columns are intentionally left blank for now.
    { header: "Errs 16+/5+", render: blank },
    { header: "Generals", render: blank },
    { header: "Memory", render: blank },
    { header: "According", render: blank },
    { header: "Context", render: blank },
    { header: "Special", render: blank },
  ];

  return (
    <DataTableTemplate<IndividualStatTS>
      entityLabel="Individual"
      showCreateButton={false}
      showDeleteButton={false}
      dense
      columns={columns}
      rows={rows}
      totalCount={rows.length}
      getId={(r) => `${r.team_name}-${r.individual}`}
      onDelete={async () => {}}
      page={0}
      pageSize={rows.length || 1}
      onPageChange={() => {}}
      onPageSizeChange={() => {}}
    />
  );
}

// ─── Main panel ───────────────────────────────────────────────────────────────

export default function StatsGroupsPanel({ tid }: { tid: string }) {
  const [statsGroups, setStatsGroups] = useState<StatsGroupTS[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<string>("");
  const [dataView, setDataView] = useState<DataView>("games");
  // Bumped after a game-events import closes, to re-fetch the Game Selection table.
  const [gamesRefreshKey, setGamesRefreshKey] = useState(0);
  // Full-table export payload for the active view (published by the active section).
  const [exportPayload, setExportPayload] = useState<ExportPayload | null>(null);

  // Clear stale export data when switching views (the new section republishes on load).
  useEffect(() => {
    setExportPayload(null);
  }, [dataView]);

  useEffect(() => {
    let cancelled = false;
    StatsGroupAPI.getByTournament(tid, 0, 200)
      .then((result) => {
        if (cancelled) return;
        setStatsGroups(result);
        // Default to the first stats group.
        if (result.length > 0) setSelectedGroup(result[0].sgid);
      })
      .catch(() => console.error("Failed to load stats groups for tournament"));
    return () => {
      cancelled = true;
    };
  }, [tid]);

  const handleGroupChange = (e: SelectChangeEvent) => setSelectedGroup(e.target.value);
  const handleDataChange = (e: SelectChangeEvent) => setDataView(e.target.value as DataView);

  const content = useMemo(() => {
    switch (dataView) {
      case "team":
        return <TeamStatsSection statsGroupId={selectedGroup} onExportReady={setExportPayload} />;
      case "individual":
        return <IndividualStatsSection statsGroupId={selectedGroup} onExportReady={setExportPayload} />;
      case "games":
      default:
        return <GamesSelectionSection tid={tid} statsGroupId={selectedGroup} refreshKey={gamesRefreshKey} onExportReady={setExportPayload} />;
    }
  }, [dataView, tid, selectedGroup, gamesRefreshKey]);

  return (
    <Stack spacing={1.5}>
      {/* ── Filter card ── */}
      <Paper
        variant="outlined"
        sx={{ p: 2, borderRadius: "10px", borderColor: "#e5e7eb" }}
      >
        <Stack direction="row" spacing={2} flexWrap="wrap" useFlexGap alignItems="center">
          <FormControl size="small" sx={{ minWidth: 220 }}>
            <InputLabel id="stats-groups-group-label">Divisions / Groups</InputLabel>
            <Select
              labelId="stats-groups-group-label"
              id="stats-groups-group-select"
              value={selectedGroup}
              label="Divisions / Groups"
              onChange={handleGroupChange}
              displayEmpty
            >
              {statsGroups.length === 0 && (
                <MenuItem value="" disabled>
                  No stats groups
                </MenuItem>
              )}
              {statsGroups.map((sg) => (
                <MenuItem key={sg.sgid} value={sg.sgid}>
                  {sg.name}{sg.division_id ? " (Division)" : " (Group)"}
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <FormControl size="small" sx={{ minWidth: 200 }}>
            <InputLabel id="stats-groups-data-label">Data</InputLabel>
            <Select
              labelId="stats-groups-data-label"
              id="stats-groups-data-select"
              value={dataView}
              label="Data"
              onChange={handleDataChange}
            >
              {DATA_OPTIONS.map((opt) => (
                <MenuItem key={opt.value} value={opt.value}>
                  {opt.label}
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          {dataView === "games" && (
            <ImportGameEventsButton tid={tid} onImported={() => setGamesRefreshKey((k) => k + 1)} />
          )}

          <ExportTableButton
            getPayload={() => exportPayload}
            disabled={!exportPayload || exportPayload.rows.length === 0}
          />
        </Stack>
      </Paper>

      {/* ── Content ── */}
      <Box>{content}</Box>
    </Stack>
  );
}
