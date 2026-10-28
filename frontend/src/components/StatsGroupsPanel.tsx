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
import { StatsGroupAPI, type StatsGroupTS, type TeamStatTS, type IndividualStatTS } from "../features/StatsGroupAPI";
import { DivisionAPI } from "../features/DivisionAPI";
import { useAuth } from "../hooks/useAuth";
import { Link } from "react-router-dom";
import ImportGameEventsButton from "./ImportGameEventsButton";
import RankRuleDialog from "./RankRuleDialog";
import TieBreakerRulesDialog from "./TieBreakerRulesDialog";
import SetQuestionTypesButton from "./SetQuestionTypesButton";
import ListAltIcon from "@mui/icons-material/ListAlt";
import Tooltip from "@mui/material/Tooltip";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import ExportTableButton from "./ExportTableButton";
import { exportFilename, type ExportPayload } from "../features/exportTable";
import QuestionTypeGrid from "./QuestionTypeGrid";
import { questionTypeColumns, questionTypeExportColumns, questionTypeExportValues } from "../utils/questionTypeStats";
import OutsAndFoulsGrid from "./OutsAndFoulsGrid";
import { outsAndFoulsColumns, outsAndFoulsExportColumns, outsAndFoulsExportValues } from "../utils/outsAndFoulsStats";

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
  const { accessToken } = useAuth();
  const [rows, setRows] = useState<GameSelectionRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<Set<string>>(new Set());

  // Display order: Selected (checked first), then Division, Room, Round.
  const sortedRows = useMemo(() => {
    const selectedRank = (r: GameSelectionRow) => (selected.has(r.gid) ? 0 : 1);
    return [...rows].sort(
      (a, b) =>
        selectedRank(a) - selectedRank(b) ||
        a.division.localeCompare(b.division, undefined, { numeric: true }) ||
        a.room.localeCompare(b.room, undefined, { numeric: true }) ||
        a.round.localeCompare(b.round, undefined, { numeric: true })
    );
  }, [rows, selected]);

  useEffect(() => {
    let cancelled = false;
    Promise.all([
      // Enriched rows carry each game's division (games no longer store it directly).
      GameAPI.getRowsByTournament(tid, 0, 500),
      RoomAPI.getByTournament(tid, 0, 500),
      RoundAPI.getByTournament(tid, 0, 500),
      DivisionAPI.getByTournament(tid, 0, 500),
      GameAPI.getStatuses(tid),
      statsGroupId ? StatsGroupAPI.getGames(statsGroupId, 0, 500, accessToken) : Promise.resolve([]),
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
            // Data-incomplete alert: past regulation (question >= 20) but data isn't OK.
            const dataAlert = status?.next_question != null && status.next_question >= 20 && !status.data_ok;
            return {
              gid: g.gid,
              division: divisionNames.get(g.divisionid) ?? g.divisionid,
              room: roomNames.get(g.roomid) ?? g.roomid,
              round: roundNames.get(g.roundid) ?? g.roundid,
              question: status?.next_question != null ? String(status.next_question) : "—",
              done: status?.done ? "Yes" : "No",
              dataOk: status?.data_ok ? "Yes" : "No",
              // Shown in the Information column (and export) only under the red-row condition.
              information: dataAlert ? "Data Incomplete. Resend Advised." : "",
            };
          })
        );
        // Pre-select the games that are already in the selected stats group.
        setSelected(new Set(groupGames.map((g) => g.gid)));
      })
      .catch(() => console.error("Failed to load games for stats group selection"))
      .finally(() => { if (!cancelled) setLoading(false); });
    return () => {
      cancelled = true;
    };
  }, [tid, statsGroupId, refreshKey, accessToken]);

  // Publish the full table (all rows) for export, reflecting live selection state.
  useEffect(() => {
    onExportReady({
      filename: "game-selection",
      columns: ["Selected", "Division", "Room", "Round", "Question", "Done", "DataOk", "Information"],
      rows: sortedRows.map((r) => [
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
  }, [sortedRows, selected, onExportReady]);

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

  // A game past regulation (question >= 20) that still has bad data is an alert — its row
  // is shown with a red background.
  const isDataAlert = (r: GameSelectionRow): boolean => Number(r.question) >= 20 && r.dataOk === "No";

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
    {
      header: "DataOk",
      render: (r) => {
        // When a game has started (question >= 1) but its data isn't OK, link "No" to the
        // Room Monitor so the user can issue a resend there.
        const started = Number(r.question) >= 1;
        if (r.dataOk === "No" && started) {
          return (
            <Link
              to={`/tournament/${tid}/room-monitor`}
              // White on the red alert rows, blue otherwise, so the link stays readable.
              style={{ color: isDataAlert(r) ? "#ffffff" : "#2563eb", fontWeight: 600, textDecoration: "underline" }}
              title="Data is missing — open the Room Monitor to resend"
            >
              No
            </Link>
          );
        }
        return r.dataOk;
      },
    },
    {
      // Only "Data Incomplete. Resend Advised.", shown under the same condition as the red row.
      header: "Information",
      render: (r) => (r.information ? <div>{r.information}</div> : null),
    },
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
        loading={loading}
        entityLabel="Game"
        showCreateButton={false}
        showDeleteButton={false}
        dense
        columns={columns}
        rows={sortedRows}
        totalCount={sortedRows.length}
        getId={(r) => r.gid}
        onDelete={async () => {}}
        page={0}
        pageSize={sortedRows.length || 1}
        onPageChange={() => {}}
        onPageSizeChange={() => {}}
        getRowStyle={(r) => (isDataAlert(r) ? { background: "#c0392b", color: "#ffffff" } : undefined)}
      />
    </Stack>
  );
}

/** A stats row's expanded panel: its tables side by side on desktop (wrapping if they don't fit), stacked on mobile. */
function ExpandedStats({ children }: { children: ReactNode }) {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: { xs: "column", md: "row" },
        flexWrap: { md: "wrap" },
        alignItems: "flex-start",
        gap: 1,
      }}
    >
      {children}
    </Box>
  );
}

function TeamStatsSection({ statsGroupId, onExportReady, rulesRefreshKey }: {
  statsGroupId: string;
  onExportReady: (p: ExportPayload) => void;
  /** Bumped when the tie-breaking rules are saved, to refetch the re-sorted standings. */
  rulesRefreshKey: number;
}) {
  const { accessToken } = useAuth();
  const [rows, setRows] = useState<TeamStatTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [rankRuleTeam, setRankRuleTeam] = useState<TeamStatTS | null>(null);

  useEffect(() => {
    if (!statsGroupId) {
      setRows([]);
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    StatsGroupAPI.getTeamStats(statsGroupId, accessToken)
      .then((result) => {
        if (!cancelled) setRows(result);
      })
      .catch(() => {
        if (!cancelled) setRows([]);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [statsGroupId, accessToken, rulesRefreshKey]);

  useEffect(() => {
    onExportReady({
      filename: "team-stats",
      columns: [
        "Place", "Name", "# Games", "Wins", "Losses", "Olympic Points", "Mod. Olympic Points", "Total Points", "Total Avg Pts",
        ...outsAndFoulsExportColumns("team"), ...questionTypeExportColumns, "Rank Rules",
      ],
      rows: rows.map((r) => [
        r.place, r.name, r.games, r.wins, r.losses, r.olympic_points, r.mod_olympic_points,
        r.total_points, r.avg_points.toFixed(1), ...outsAndFoulsExportValues(r.outs_and_fouls, "team"),
        ...questionTypeExportValues(r.question_types), rankRuleSummary(r),
      ]),
    });
  }, [rows, onExportReady]);

  const columns: ColumnDef<TeamStatTS>[] = [
    { header: "Place", render: (r) => r.place },
    { header: "Name", render: (r) => <span style={{ whiteSpace: "nowrap" }}>{r.name}</span> },
    { header: "# Games", render: (r) => r.games },
    { header: "Wins", render: (r) => r.wins },
    { header: "Losses", render: (r) => r.losses },
    { header: "Olympic\nPoints", render: (r) => r.olympic_points },
    { header: "Mod.\nOlympic\nPoints", render: (r) => r.mod_olympic_points },
    { header: "Total\nPoints", render: (r) => r.total_points },
    { header: "Total\nAvg.\nPoints", render: (r) => r.avg_points.toFixed(1) },
    ...outsAndFoulsColumns<TeamStatTS>((r) => r.outs_and_fouls, "team"),
    ...questionTypeColumns<TeamStatTS>((r) => r.question_types),
    {
      header: "Rank\nRules",
      render: (r) => (
        <IconButton
          size="small"
          aria-label={`Why ${r.name} is ranked here`}
          title={`Why ${r.name} is ranked here`}
          onClick={(e) => {
            e.stopPropagation(); // don't also expand/collapse the row
            setRankRuleTeam(r);
          }}
        >
          <InfoOutlinedIcon fontSize="small" />
        </IconButton>
      ),
    },
  ];

  return (
    <>
    <DataTableTemplate<TeamStatTS>
      loading={loading}
      entityLabel="Team"
      showCreateButton={false}
      showDeleteButton={false}
      dense
      columns={columns}
      rows={rows}
      totalCount={rows.length}
      getId={(r) => r.name}
      renderExpanded={(r) => (
        <ExpandedStats>
          <QuestionTypeGrid stats={r.question_types} />
          <OutsAndFoulsGrid stats={r.outs_and_fouls} scope="team" />
        </ExpandedStats>
      )}
      onDelete={async () => {}}
      page={0}
      pageSize={rows.length || 1}
      onPageChange={() => {}}
      onPageSizeChange={() => {}}
    />
    <RankRuleDialog team={rankRuleTeam} onClose={() => setRankRuleTeam(null)} />
    </>
  );
}

/** One-line form of a row's rank rules, for the CSV/export where a dialog isn't available. */
function rankRuleSummary(r: TeamStatTS): string {
  const parts: string[] = [];
  if (r.rank_rule_above) {
    parts.push(`below ${r.rank_rule_above.other_team} by rule ${r.rank_rule_above.number} (${r.rank_rule_above.label}: ${r.rank_rule_above.detail})`);
  }
  if (r.rank_rule_below) {
    parts.push(`above ${r.rank_rule_below.other_team} by rule ${r.rank_rule_below.number} (${r.rank_rule_below.label}: ${r.rank_rule_below.detail})`);
  }
  return parts.join("; ");
}

function IndividualStatsSection({ statsGroupId, onExportReady }: { statsGroupId: string; onExportReady: (p: ExportPayload) => void }) {
  const { accessToken } = useAuth();
  const [rows, setRows] = useState<IndividualStatTS[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!statsGroupId) {
      setRows([]);
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    StatsGroupAPI.getIndividualStats(statsGroupId, accessToken)
      .then((result) => {
        if (!cancelled) setRows(result);
      })
      .catch(() => {
        if (!cancelled) setRows([]);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [statsGroupId, accessToken]);

  useEffect(() => {
    onExportReady({
      filename: "individual-stats",
      columns: [
        "Place", "Individual", "Team Name", "# Games", "Score", "Avg", "Correct", "Errors", "Bonus Pts", "Bonus Attempts",
        "Errs 16+/5+", ...outsAndFoulsExportColumns("individual"), ...questionTypeExportColumns,
      ],
      rows: rows.map((r) => [
        r.place, r.individual, r.team_name, r.games, r.score, r.avg.toFixed(1), r.correct, r.errors, r.bonus_pts, r.bonus_attempts,
        "", ...outsAndFoulsExportValues(r.outs_and_fouls, "individual"), ...questionTypeExportValues(r.question_types),
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
    // Intentionally left blank for now.
    { header: "Errs 16+/5+", render: blank },
    ...outsAndFoulsColumns<IndividualStatTS>((r) => r.outs_and_fouls, "individual"),
    ...questionTypeColumns<IndividualStatTS>((r) => r.question_types),
  ];

  return (
    <DataTableTemplate<IndividualStatTS>
      loading={loading}
      entityLabel="Individual"
      showCreateButton={false}
      showDeleteButton={false}
      dense
      columns={columns}
      rows={rows}
      totalCount={rows.length}
      getId={(r) => `${r.team_name}-${r.individual}`}
      renderExpanded={(r) => (
        <ExpandedStats>
          <QuestionTypeGrid stats={r.question_types} />
          <OutsAndFoulsGrid stats={r.outs_and_fouls} scope="individual" />
        </ExpandedStats>
      )}
      onDelete={async () => {}}
      page={0}
      pageSize={rows.length || 1}
      onPageChange={() => {}}
      onPageSizeChange={() => {}}
    />
  );
}

// ─── Main panel ───────────────────────────────────────────────────────────────

/**
 * `canImport` gates the game-event import, which rewrites games' recorded events and so needs
 * `game:update` — the same permission the import endpoints check.
 */
export default function StatsGroupsPanel({ tid, tournamentName, canImport = false, canEditTieBreakers = false, canSetQuestionTypes = false }: {
  tid: string;
  /** Leads the export file names. */
  tournamentName?: string;
  canImport?: boolean;
  /** Whether the "Tie-breaking Rules" button is offered (needs `statsgroup:update`). */
  canEditTieBreakers?: boolean;
  /** Whether the "Set Question Types" button is offered (needs `round:upload_question_types`). */
  canSetQuestionTypes?: boolean;
}) {
  const { accessToken } = useAuth();
  const [statsGroups, setStatsGroups] = useState<StatsGroupTS[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<string>("");
  const [dataView, setDataView] = useState<DataView>("games");
  // Bumped after a game-events import closes, to re-fetch the Game Selection table.
  const [gamesRefreshKey, setGamesRefreshKey] = useState(0);
  // Full-table export payload for the active view (published by the active section).
  const [exportPayload, setExportPayload] = useState<ExportPayload | null>(null);
  // The tie-breaking rules dialog, and a key bumped on save so Team Stats refetches in the new order.
  const [tieBreakerDialogOpen, setTieBreakerDialogOpen] = useState(false);
  const [rulesRefreshKey, setRulesRefreshKey] = useState(0);

  // Clear stale export data when switching views (the new section republishes on load).
  useEffect(() => {
    setExportPayload(null);
  }, [dataView]);

  useEffect(() => {
    let cancelled = false;
    StatsGroupAPI.getByTournament(tid, 0, 200, accessToken)
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
  }, [tid, accessToken]);

  const selectedDivisionId = statsGroups.find((sg) => sg.sgid === selectedGroup)?.division_id ?? null;

  const handleGroupChange = (e: SelectChangeEvent) => setSelectedGroup(e.target.value);
  const handleDataChange = (e: SelectChangeEvent) => setDataView(e.target.value as DataView);

  const content = useMemo(() => {
    switch (dataView) {
      case "team":
        return <TeamStatsSection statsGroupId={selectedGroup} onExportReady={setExportPayload} rulesRefreshKey={rulesRefreshKey} />;
      case "individual":
        return <IndividualStatsSection statsGroupId={selectedGroup} onExportReady={setExportPayload} />;
      case "games":
      default:
        return <GamesSelectionSection tid={tid} statsGroupId={selectedGroup} refreshKey={gamesRefreshKey} onExportReady={setExportPayload} />;
    }
  }, [dataView, tid, selectedGroup, gamesRefreshKey, rulesRefreshKey]);

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

          {dataView === "games" && canImport && (
            <ImportGameEventsButton tid={tid} onImported={() => setGamesRefreshKey((k) => k + 1)} />
          )}

          {dataView === "team" && canEditTieBreakers && selectedGroup && (
            <Button variant="outlined" onClick={() => setTieBreakerDialogOpen(true)}>
              Tie-breaking Rules
            </Button>
          )}

          {/* Question types feed the Memory Verse tie-breaker. They're set per division, so a
              custom group (whose games can span divisions) points to the divisions instead. */}
          {dataView === "team" && canSetQuestionTypes && selectedGroup && (
            selectedDivisionId ? (
              <SetQuestionTypesButton
                did={selectedDivisionId}
                buttonProps={{ variant: "outlined", size: "medium" }}
                onSaved={() => setRulesRefreshKey((k) => k + 1)}
              />
            ) : (
              <Tooltip title="Question types are set per division. Select a division's stats group, or set them on each division's Rounds page.">
                <span>
                  <Button variant="outlined" startIcon={<ListAltIcon />} disabled>
                    Set Question Types
                  </Button>
                </span>
              </Tooltip>
            )
          )}

          <ExportTableButton
            // Named at click time, so the file carries the moment it was exported.
            getPayload={() => exportPayload && { ...exportPayload, filename: exportFilename(tournamentName, exportPayload.filename) }}
            disabled={!exportPayload || exportPayload.rows.length === 0}
          />
        </Stack>
      </Paper>

      {/* ── Content ── */}
      <Box>{content}</Box>

      {/* Saving a new order re-sorts the standings; Cancel leaves them untouched. */}
      <TieBreakerRulesDialog
        statsGroupId={selectedGroup}
        isOpen={tieBreakerDialogOpen}
        onCancel={() => setTieBreakerDialogOpen(false)}
        onSaved={() => {
          setTieBreakerDialogOpen(false);
          setRulesRefreshKey((k) => k + 1);
        }}
      />
    </Stack>
  );
}
