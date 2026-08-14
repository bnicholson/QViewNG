import { useEffect, useMemo, useState, type ReactNode } from "react";
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
import { StatsGroupAPI, type StatsGroupTS, type TeamStatTS } from "../features/StatsGroupAPI";
import { DivisionAPI } from "../features/DivisionAPI";

// ─── Presentational table matching the QView DataTableTemplate style ──────────
// (No CRUD toolbar / pagination — these are read-only ranking tables.)

interface StatsColumn<T> {
  header: string;
  render: (row: T) => ReactNode;
}

function StatsTable<T>({
  columns,
  rows,
  getRowKey,
}: {
  columns: StatsColumn<T>[];
  rows: T[];
  getRowKey: (row: T, index: number) => string | number;
}) {
  return (
    <div style={{ overflowX: "auto", overflowY: "auto", maxHeight: "60vh", borderRadius: 10, border: "1px solid #e5e7eb" }}>
      <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 14 }}>
        <thead>
          <tr style={{ background: "#f9fafb", borderBottom: "1px solid #e5e7eb", position: "sticky", top: 0, zIndex: 1 }}>
            {columns.map((col) => (
              <th
                key={col.header}
                style={{
                  padding: "8px 14px",
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
          </tr>
        </thead>
        <tbody>
          {rows.length === 0 ? (
            <tr>
              <td colSpan={columns.length} style={{ padding: "32px 14px", textAlign: "center", color: "#9ca3af" }}>
                No data found.
              </td>
            </tr>
          ) : (
            rows.map((row, i) => (
              <tr
                key={getRowKey(row, i)}
                style={{
                  background: i % 2 === 0 ? "#fff" : "#fafafa",
                  borderBottom: "1px solid #f3f4f6",
                  transition: "background .1s",
                }}
                onMouseEnter={(e) => (e.currentTarget.style.background = "#f0f7ff")}
                onMouseLeave={(e) => (e.currentTarget.style.background = i % 2 === 0 ? "#fff" : "#fafafa")}
              >
                {columns.map((col) => (
                  <td key={col.header} style={{ padding: "8px 14px", color: "#374151", whiteSpace: "nowrap" }}>
                    {col.render(row)}
                  </td>
                ))}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}

// ─── Dummy data (placeholder until backend endpoints exist) ───────────────────

interface IndividualStatRow {
  place: number;
  individual: string;
  teamName: string;
  games: number;
  score: number;
  avg: number;
  correct: number;
  errors: number;
  bonusPts: number;
  bonusAttempts: number;
}

const DUMMY_INDIVIDUAL_STATS: IndividualStatRow[] = [
  { place: 1, individual: "Isla Mackenzie", teamName: "Grace Fellowship A", games: 8, score: 1240, avg: 155.0, correct: 96, errors: 8, bonusPts: 180, bonusAttempts: 22 },
  { place: 2, individual: "Noah Whitfield", teamName: "Cornerstone Blue", games: 8, score: 1180, avg: 147.5, correct: 91, errors: 11, bonusPts: 160, bonusAttempts: 20 },
  { place: 3, individual: "Clara Hoffmann", teamName: "New Life Red", games: 8, score: 1055, avg: 131.9, correct: 84, errors: 9, bonusPts: 140, bonusAttempts: 19 },
  { place: 4, individual: "Felix Almeida", teamName: "Trinity Eagles", games: 8, score: 990, avg: 123.8, correct: 78, errors: 13, bonusPts: 120, bonusAttempts: 17 },
  { place: 5, individual: "Samuel Okafor", teamName: "Living Water B", games: 8, score: 870, avg: 108.8, correct: 69, errors: 15, bonusPts: 100, bonusAttempts: 15 },
  { place: 6, individual: "Priya Nair", teamName: "Redeemer Gold", games: 8, score: 795, avg: 99.4, correct: 63, errors: 12, bonusPts: 90, bonusAttempts: 14 },
];

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

function GamesSelectionSection({ tid, statsGroupId }: { tid: string; statsGroupId: string }) {
  const [rows, setRows] = useState<GameSelectionRow[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    let cancelled = false;
    Promise.all([
      GameAPI.getByTournament(tid, 0, 500),
      RoomAPI.getByTournament(tid, 0, 500),
      RoundAPI.getByTournament(tid, 0, 500),
      DivisionAPI.getByTournament(tid, 0, 500),
      statsGroupId ? StatsGroupAPI.getGames(statsGroupId, 0, 500) : Promise.resolve([]),
    ])
      .then(([gamesResult, rooms, rounds, divisions, groupGames]) => {
        if (cancelled) return;
        const roomNames = new Map(rooms.map((r) => [r.roomid, r.name]));
        const roundNames = new Map(rounds.map((r) => [r.roundid, r.name]));
        const divisionNames = new Map(divisions.map((d) => [d.did, d.dname]));
        setRows(
          gamesResult.items.map((g) => ({
            gid: g.gid,
            division: divisionNames.get(g.divisionid) ?? g.divisionid,
            room: roomNames.get(g.roomid) ?? g.roomid,
            round: roundNames.get(g.roundid) ?? g.roundid,
            question: "—",
            done: "Yes",
            dataOk: "Yes",
            information: "",
          }))
        );
        // Pre-select the games that are already in the selected stats group.
        setSelected(new Set(groupGames.map((g) => g.gid)));
      })
      .catch(() => console.error("Failed to load games for stats group selection"));
    return () => {
      cancelled = true;
    };
  }, [tid, statsGroupId]);

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

function TeamStatsSection({ statsGroupId }: { statsGroupId: string }) {
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

  const columns: ColumnDef<TeamStatTS>[] = [
    { header: "Place", render: (r) => r.place },
    { header: "Name", render: (r) => r.name },
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

function IndividualStatsSection() {
  const blank = () => "";
  const columns: StatsColumn<IndividualStatRow>[] = [
    { header: "Place", render: (r) => r.place },
    { header: "Individual", render: (r) => r.individual },
    { header: "Team Name", render: (r) => r.teamName },
    { header: "# Games", render: (r) => r.games },
    { header: "Score", render: (r) => r.score },
    { header: "Avg", render: (r) => r.avg.toFixed(1) },
    { header: "Correct", render: (r) => r.correct },
    { header: "Errors", render: (r) => r.errors },
    { header: "Bonus Pts", render: (r) => r.bonusPts },
    { header: "Bonus Attempts", render: (r) => r.bonusAttempts },
    // Last 6 columns intentionally left without dummy data.
    { header: "Errs 16+/5+", render: blank },
    { header: "Generals", render: blank },
    { header: "Memory", render: blank },
    { header: "According", render: blank },
    { header: "Context", render: blank },
    { header: "Special", render: blank },
  ];
  return <StatsTable columns={columns} rows={DUMMY_INDIVIDUAL_STATS} getRowKey={(r) => r.place} />;
}

// ─── Main panel ───────────────────────────────────────────────────────────────

export default function StatsGroupsPanel({ tid }: { tid: string }) {
  const [statsGroups, setStatsGroups] = useState<StatsGroupTS[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<string>("");
  const [dataView, setDataView] = useState<DataView>("games");

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
        return <TeamStatsSection statsGroupId={selectedGroup} />;
      case "individual":
        return <IndividualStatsSection />;
      case "games":
      default:
        return <GamesSelectionSection tid={tid} statsGroupId={selectedGroup} />;
    }
  }, [dataView, tid, selectedGroup]);

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
        </Stack>
      </Paper>

      {/* ── Content ── */}
      <Box>{content}</Box>
    </Stack>
  );
}
