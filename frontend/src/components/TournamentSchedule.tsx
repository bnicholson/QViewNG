import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import dayjs from 'dayjs'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import Button from '@mui/material/Button'
import ButtonGroup from '@mui/material/ButtonGroup'
import FormControl from '@mui/material/FormControl'
import InputLabel from '@mui/material/InputLabel'
import Select, { type SelectChangeEvent } from '@mui/material/Select'
import MenuItem from '@mui/material/MenuItem'
import Chip from '@mui/material/Chip'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Typography from '@mui/material/Typography'
import Tabs from '@mui/material/Tabs'
import Tab from '@mui/material/Tab'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import Divider from '@mui/material/Divider'
import Alert from '@mui/material/Alert'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import Paper from '@mui/material/Paper'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'

import { useAuth } from '../hooks/useAuth'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { PoolBracketGroupAPI, type PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI'
import { GameAPI, type GameRowTS, type PersonGameRowTS } from '../features/GameAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { RoomGroupAPI, type RoomGroupTS } from '../features/RoomGroupAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { TournamentAPI } from '../features/TournamentAPI'
import { DivisionEditorDialog } from './DivisionEditorDialog'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'
import RoundsManager, { type RoundGameEntry } from './RoundsManager'
import { PoolDetail, naturalCompare } from './PoolDetail'
import { PoolsManager } from './PoolsManager'

const PAGE = 0
const SIZE = 500
// Read-view results are paged (infinite scroll) rather than fetched all at once.
const RESULTS_PAGE_SIZE = 25

type Mode = 'read' | 'conflicts' | 'edit' | 'edit2'

interface Props {
  tid: string
  /** Whether the current user may create/modify schedule entities. */
  canEdit?: boolean
}

/**
 * A self-contained schedule editor for an entire tournament. Divisions → Rounds →
 * Pools/Brackets → Teams & Games are all reachable and editable from this single component, so it
 * can be dropped onto the Tournament profile (or anywhere else) without additional wiring.
 *
 * The component has two modes toggled by the buttons at the top: an "Edit" version (built first)
 * where changes happen inline (rather than in an EditorDialog), and a "Read" version to come.
 */
export const TournamentSchedule = ({ tid, canEdit = false }: Props) => {
  const { accessToken } = useAuth()
  const navigate = useNavigate()

  const [mode, setMode] = useState<Mode>('read')
  const [error, setError] = useState<string | null>(null)
  // The tournament's default round duration: how long each round is taken to last when finding
  // breaks/overlaps between rounds in Create / Edit.
  const [roundDuration, setRoundDuration] = useState(30)

  // Row 1 — divisions
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  const [selectedDid, setSelectedDid] = useState('')

  // Row 2 — the division's pools/brackets and teams
  const [divisionBrackets, setDivisionBrackets] = useState<PoolBracketTS[]>([])
  const [divisionTeams, setDivisionTeams] = useState<TeamTS[]>([])
  // Poolbracketgroups scope team placement: a team is in exactly one pool per group (Pools tab).
  const [poolBracketGroups, setPoolBracketGroups] = useState<PoolBracketGroupTS[]>([])
  const [selectedGroupId, setSelectedGroupId] = useState('')

  // The "active" pool/bracket — the target the unplaced-team chips add to. With the dropdown gone,
  // every pool is shown as a card and the active one is chosen by clicking its card.
  const [selectedBracketId, setSelectedBracketId] = useState('')

  // The card's top-level tab: Rounds (rounds + games), Pools, or Brackets.
  const [cardTab, setCardTab] = useState<'rounds' | 'pools' | 'brackets'>('rounds')

  // Teams and games per pool/bracket in the division (drive each card's team list + matrix).
  const [teamsByBracket, setTeamsByBracket] = useState<Record<string, TeamRowTS[]>>({})
  const [gamesByBracket, setGamesByBracket] = useState<Record<string, GameRowTS[]>>({})

  // Create-dialog visibility
  // Create/Edit dialog state — each carries the entity being edited, or null when creating.
  const [divDialog, setDivDialog] = useState<{ open: boolean; division: DivisionTS | null }>({ open: false, division: null })
  const [bracketDialog, setBracketDialog] = useState<{ open: boolean; type: 'pool' | 'bracket'; bracket: PoolBracketTS | null }>({ open: false, type: 'pool', bracket: null })

  // ── Loaders ──────────────────────────────────────────────────────────────

  const loadDivisions = useCallback(() => {
    DivisionAPI.getByTournament(tid, PAGE, SIZE)
      .then(setDivisions)
      .catch(() => setError('Failed to load divisions.'))
  }, [tid])

  const loadDivisionData = useCallback((did: string) => {
    PoolBracketAPI.getByDivision(did).then(setDivisionBrackets).catch(() => setError('Failed to load pools/brackets.'))
    PoolBracketGroupAPI.getByDivision(did).then(setPoolBracketGroups).catch(() => setError('Failed to load pool groups.'))
    TeamAPI.getByDivision(did, PAGE, SIZE).then(setDivisionTeams).catch(() => setError('Failed to load teams.'))
  }, [])


  // Pool brackets are division-scoped now, so placement/games load for all of the division's brackets.
  const loadPlacement = useCallback((brackets: PoolBracketTS[]) => {
    Promise.all(
      brackets.map(b =>
        TeamAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE).then(r => [b.pool_bracket_id, r.items] as const)
      )
    )
      .then(entries => setTeamsByBracket(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load team placements.'))
  }, [])

  const loadGamesForDivision = useCallback((brackets: PoolBracketTS[]) => {
    Promise.all(
      brackets.map(b =>
        GameAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE).then(r => [b.pool_bracket_id, r.items] as const)
      )
    )
      .then(entries => setGamesByBracket(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load games.'))
  }, [])

  // ── Effects ──────────────────────────────────────────────────────────────

  useEffect(() => { loadDivisions() }, [loadDivisions])

  useEffect(() => {
    if (!tid) return
    TournamentAPI.getById(tid)
      .then(t => setRoundDuration(t.default_round_duration ?? 30))
      .catch(() => { /* leave the 30-minute default if the tournament can't be loaded */ })
  }, [tid])

  // Auto-select the first division as soon as divisions load (keeping any valid current choice), so
  // the cascade below fills in and the user lands on a populated schedule instead of empty dropdowns.
  useEffect(() => {
    setSelectedDid(prev => divisions.some(d => d.did === prev) ? prev : (divisions[0]?.did ?? ''))
  }, [divisions])

  useEffect(() => {
    setSelectedBracketId('')
    setDivisionBrackets([])
    setDivisionTeams([])
    setPoolBracketGroups([])
    setSelectedGroupId('')
    if (selectedDid) loadDivisionData(selectedDid)
  }, [selectedDid, loadDivisionData])

  // Auto-select the first poolbracketgroup once the division's groups load (keeping a valid choice).
  useEffect(() => {
    setSelectedGroupId(prev => poolBracketGroups.some(g => g.poolbracketgroupid === prev) ? prev : (poolBracketGroups[0]?.poolbracketgroupid ?? ''))
  }, [poolBracketGroups])

  useEffect(() => {
    setTeamsByBracket({})
    setGamesByBracket({})
    if (selectedDid) {
      loadPlacement(divisionBrackets)
      loadGamesForDivision(divisionBrackets)
    }
  }, [selectedDid, divisionBrackets, loadPlacement, loadGamesForDivision])

  // ── Derived values ───────────────────────────────────────────────────────

  // Pool brackets belong to the division, so the whole division's brackets are in play.
  const pools = useMemo(() => divisionBrackets.filter(b => b.type === 'pool'), [divisionBrackets])
  const brackets = useMemo(() => divisionBrackets.filter(b => b.type === 'bracket'), [divisionBrackets])

  // The Pools tab is scoped to the selected poolbracketgroup: each team must be in exactly one pool
  // per group, so placement (and the pool tabs) only consider that group's pools. The Brackets tab
  // stays division-wide.
  const isBracketMode = cardTab === 'brackets'
  const groupPools = useMemo(
    () => pools.filter(p => p.poolbracketgroupid === selectedGroupId),
    [pools, selectedGroupId]
  )
  const row3Options = isBracketMode ? brackets : groupPools

  // Keep the active pool/bracket (the unplaced-team target) valid: default to the first one and
  // fall back to the first whenever the current one disappears (group/division change, deletion).
  useEffect(() => {
    setSelectedBracketId(prev =>
      row3Options.some(b => b.pool_bracket_id === prev) ? prev : (row3Options[0]?.pool_bracket_id ?? '')
    )
  }, [row3Options])

  // "Placed" teams are those in a pool/bracket relevant to the active tab: for Pools, only the
  // selected group's pools (one-per-group); for Brackets, all of the division's brackets.
  const placedTeamIds = useMemo(() => {
    const relevant = isBracketMode ? brackets : groupPools
    return new Set(relevant.flatMap(b => (teamsByBracket[b.pool_bracket_id] ?? []).map(t => t.teamid)))
  }, [isBracketMode, brackets, groupPools, teamsByBracket])
  const unplacedTeams = useMemo(
    () => divisionTeams.filter(t => !placedTeamIds.has(t.teamid)),
    [divisionTeams, placedTeamIds]
  )

  // Rounds tab: the division's games grouped by round id (with each game's pool name), so the
  // Pools matrix's games show under the round they belong to.
  const gamesByRound = useMemo(() => {
    const poolNameById = new Map(divisionBrackets.map(b => [b.pool_bracket_id, b.name]))
    const m = new Map<string, RoundGameEntry[]>()
    Object.entries(gamesByBracket).forEach(([pbId, games]) => {
      const poolName = poolNameById.get(pbId) ?? ''
      games.forEach(g => {
        const entry = { g, poolName, poolBracketId: pbId }
        const list = m.get(g.roundid)
        if (list) list.push(entry); else m.set(g.roundid, [entry])
      })
    })
    return m
  }, [gamesByBracket, divisionBrackets])

  const selectedBracket = divisionBrackets.find(b => b.pool_bracket_id === selectedBracketId) ?? null
  const selectedDivision = divisions.find(d => d.did === selectedDid) ?? null

  // ── Team placement handlers ──────────────────────────────────────────────

  const refresh = () => {
    loadPlacement(divisionBrackets)
    loadGamesForDivision(divisionBrackets)
  }


  const handleAddTeam = async (teamid: string) => {
    if (!selectedBracketId) return
    try {
      await TeamAPI.addToPoolBracket(selectedBracketId, teamid, accessToken)
      refresh()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to add team.')
    }
  }

  const handleRemoveTeam = async (bracketId: string, teamid: string) => {
    try {
      await TeamAPI.removeFromPoolBracket(bracketId, teamid, accessToken)
      refresh()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to remove team.')
    }
  }

  // ── Render ───────────────────────────────────────────────────────────────

  // Only editors may switch to Edit; everyone else is locked to the read-only view.
  const activeMode: Mode = canEdit ? mode : 'read'

  return (
    <Stack spacing={2}>
      {/* Read / Edit toggle — shown only to users who may edit the schedule. */}
      {canEdit && (
        <ButtonGroup variant="outlined" size="small">
          <Button variant={activeMode === 'read' ? 'contained' : 'outlined'} onClick={() => setMode('read')}>Read</Button>
          <Button variant={activeMode === 'conflicts' ? 'contained' : 'outlined'} onClick={() => setMode('conflicts')}>Conflicts</Button>
          {/* The original "V1" Create / Edit view is still implemented below (the `edit` mode) — its
              button is removed so only this view is reachable, but its components are kept. */}
          <Button variant={activeMode === 'edit2' ? 'contained' : 'outlined'} onClick={() => setMode('edit2')}>Create / Edit</Button>
        </ButtonGroup>
      )}

      {error && <Alert severity="error" onClose={() => setError(null)}>{error}</Alert>}

      {activeMode === 'read' ? (
        <ScheduleReadView tid={tid} />
      ) : activeMode === 'conflicts' ? (
        <Alert severity="info">Scheduling conflict resolution is coming soon.</Alert>
      ) : activeMode === 'edit2' ? (
        <DivisionRoundsCompare
          tid={tid}
          divisions={divisions}
          canEdit={canEdit}
          roundDurationMinutes={roundDuration}
        />
      ) : (
        <Stack spacing={2}>

          {/* Row 1 — Division */}
          <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Division</InputLabel>
              <Select
                label="Division"
                value={selectedDid}
                onChange={(e: SelectChangeEvent) => setSelectedDid(e.target.value)}
              >
                {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            </FormControl>
            <Button startIcon={<EditIcon />} onClick={() => setDivDialog({ open: true, division: selectedDivision })} disabled={!canEdit || !selectedDivision}>
              Edit
            </Button>
            {/* Divisions shouldn't be created after registration, and scheduling comes after
                registration closes — so no "Create Division" here. */}
            {/* <Button startIcon={<AddIcon />} onClick={() => setDivDialog({ open: true, division: null })} disabled={!canEdit}>
              Create Division
            </Button> */}
          </Box>

          {/* The schedule card: teams awaiting placement on top, then the pool/bracket tabs (with a
              "+" to create another at the end of the row), then the selected pool/bracket's detail. */}
          {selectedDid && (
            <Card variant="outlined">
              <CardContent>
                {/* Top-level tabs: Rounds (rounds + games), Pools, Brackets. */}
                <Tabs value={cardTab} onChange={(_e, v) => setCardTab(v)} sx={{ minHeight: 0 }}>
                  <Tab value="rounds" label="Rounds" />
                  <Tab value="pools" label="Pools" />
                  <Tab value="brackets" label="Brackets" />
                </Tabs>
                <Divider sx={{ mt: 1, mb: 1.5 }} />

                {cardTab === 'rounds' ? (
                  <RoundsManager
                    tid={tid}
                    did={selectedDid}
                    canEdit={canEdit}
                    roundDurationMinutes={roundDuration}
                    gamesByRound={gamesByRound}
                    onNavigateGame={(gid) => navigate(`/game/${gid}/overview`)}
                    onGamesChanged={refresh}
                    poolBrackets={divisionBrackets}
                    onPoolsChanged={() => loadDivisionData(selectedDid)}
                  />
                ) : cardTab === 'pools' ? (
                  <PoolsManager
                    tid={tid}
                    did={selectedDid}
                    canEdit={canEdit}
                    onChanged={() => loadDivisionData(selectedDid)}
                  />
                ) : (
                  <>
                    <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
                      Brackets organize a Divisions Teams into single-elimination-style tournament play, where the winner of each game advances to the next round until a champion remains. Use a Bracket when you want a knockout format rather than Round Robin scheduling.
                    </Alert>
                    {/* Teams awaiting placement — left aligned, above the bracket tabs */}
                    <Box sx={{ textAlign: 'left' }}>
                      <Typography variant="caption" color="text.secondary">
                        Teams needing placement{selectedBracket ? ` — click to add to the selected tab, "${selectedBracket.name}"` : ` — create a ${isBracketMode ? 'bracket' : 'pool'} first`}:
                      </Typography>
                      <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5 }}>
                        {unplacedTeams.length === 0
                          ? <Typography variant="body2" color="text.secondary">All teams placed.</Typography>
                          : unplacedTeams.map(t => (
                              <Chip
                                key={t.teamid}
                                label={t.name}
                                size="small"
                                onClick={canEdit && selectedBracketId ? () => handleAddTeam(t.teamid) : undefined}
                                disabled={!canEdit || !selectedBracketId}
                              />
                            ))
                        }
                      </Box>
                    </Box>

                    <Divider sx={{ my: 1.5 }} />

                    {row3Options.length === 0 ? (
                      <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
                        <Typography variant="body2" color="text.secondary">
                          This division has no {isBracketMode ? 'brackets' : 'pools'} yet:
                        </Typography>
                        <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: isBracketMode ? 'bracket' : 'pool', bracket: null })} disabled={!canEdit || (!isBracketMode && !selectedGroupId)}>
                          Create {isBracketMode ? 'Bracket' : 'Pool'}
                        </Button>
                      </Box>
                    ) : (
                      <>
                        {/* Tabs row: one tab per pool/bracket, plus edit/create actions. */}
                        <Box sx={{ display: 'flex', alignItems: 'center' }}>
                          <Typography variant="body1" sx={{ mr: 1, flexShrink: 0 }}>
                            {isBracketMode ? 'Brackets:' : 'Pools:'}
                          </Typography>
                          <Tabs
                            value={row3Options.some(b => b.pool_bracket_id === selectedBracketId) ? selectedBracketId : false}
                            onChange={(_e, value: string) => setSelectedBracketId(value)}
                            variant="scrollable"
                            scrollButtons="auto"
                            sx={{ minHeight: 0 }}
                          >
                            {row3Options.map(b => <Tab key={b.pool_bracket_id} value={b.pool_bracket_id} label={b.name} />)}
                          </Tabs>
                          <Tooltip title={`Edit ${isBracketMode ? 'Bracket' : 'Pool'}`}>
                            <span>
                              <IconButton
                                size="small"
                                onClick={() => setBracketDialog({ open: true, type: isBracketMode ? 'bracket' : 'pool', bracket: selectedBracket })}
                                disabled={!canEdit || !selectedBracket}
                                aria-label={`Edit ${isBracketMode ? 'Bracket' : 'Pool'}`}
                              >
                                <EditIcon />
                              </IconButton>
                            </span>
                          </Tooltip>
                          <Tooltip title={`Create ${isBracketMode ? 'Bracket' : 'Pool'}`}>
                            <span>
                              <IconButton
                                size="small"
                                onClick={() => setBracketDialog({ open: true, type: isBracketMode ? 'bracket' : 'pool', bracket: null })}
                                disabled={!canEdit || (!isBracketMode && !selectedGroupId)}
                                aria-label={`Create ${isBracketMode ? 'Bracket' : 'Pool'}`}
                              >
                                <AddIcon />
                              </IconButton>
                            </span>
                          </Tooltip>
                        </Box>

                        {selectedBracket && (
                          <PoolDetail
                            teams={teamsByBracket[selectedBracket.pool_bracket_id] ?? []}
                            games={gamesByBracket[selectedBracket.pool_bracket_id] ?? []}
                            canEdit={canEdit}
                            onRemoveTeam={(teamid) => handleRemoveTeam(selectedBracket.pool_bracket_id, teamid)}
                            onNavigateGame={(gid) => navigate(`/game/${gid}/overview`)}
                          />
                        )}
                      </>
                    )}
                  </>
                )}
              </CardContent>
            </Card>
          )}
        </Stack>
      )}

      {/* ── Create dialogs ── */}

      <DivisionEditorDialog
        tid={tid}
        division={divDialog.division}
        isOpen={divDialog.open}
        onCancel={() => setDivDialog({ open: false, division: null })}
        onSave={(division) => {
          setDivDialog({ open: false, division: null })
          // Reload the list, then select the just-created (or edited) division in the dropdown.
          DivisionAPI.getByTournament(tid, PAGE, SIZE)
            .then(ds => { setDivisions(ds); setSelectedDid(division.did) })
            .catch(() => setError('Failed to load divisions.'))
        }}
      />

      <PoolBracketEditorDialog
        tid={tid}
        did={selectedDid || undefined}
        type={bracketDialog.type}
        entityLabel={bracketDialog.type === 'bracket' ? 'Bracket' : 'Pool'}
        bracket={bracketDialog.bracket}
        poolBracketGroupId={bracketDialog.type === 'pool' ? (selectedGroupId || undefined) : undefined}
        isOpen={bracketDialog.open}
        onCancel={() => setBracketDialog(d => ({ ...d, open: false, bracket: null }))}
        onSave={(bracket) => {
          setBracketDialog(d => ({ ...d, open: false, bracket: null }))
          if (selectedDid) {
            PoolBracketAPI.getByDivision(selectedDid)
              .then(bs => { setDivisionBrackets(bs); setSelectedBracketId(bracket.pool_bracket_id) })
              .catch(() => setError('Failed to reload pools/brackets.'))
          }
        }}
      />
    </Stack>
  )
}

// ── Division comparison (rounds per division) ─────────────────────────────────

/** One division's cached schedule data: its pools/brackets, and games grouped by round. */
interface DivisionCompareData {
  poolBrackets: PoolBracketTS[]
  gamesByRound: Map<string, RoundGameEntry[]>
}

/**
 * The second "Create / Edit" view: a tab per division, each showing that division's Rounds
 * (a RoundsManager). Every division's data is loaded once and kept in
 * state, and every division's manager stays mounted (hidden when inactive), so switching tabs is
 * instant and the data stays cached for quick side-by-side comparison.
 */
const DivisionRoundsCompare = ({ tid, divisions, canEdit, roundDurationMinutes }: {
  tid: string
  divisions: DivisionTS[]
  canEdit: boolean
  /** The tournament's default round duration (minutes), used to find breaks between rounds. */
  roundDurationMinutes: number
}) => {
  const navigate = useNavigate()
  const [active, setActive] = useState('')
  const [dataByDid, setDataByDid] = useState<Record<string, DivisionCompareData>>({})
  const [error, setError] = useState<string | null>(null)

  // Load (or reload) one division's pools/brackets + games-by-round and cache it.
  const loadDivision = useCallback((did: string) => {
    PoolBracketAPI.getByDivision(did)
      .then(brackets =>
        Promise.all(
          brackets.map(b => GameAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE).then(r => [b, r.items] as const))
        ).then(entries => {
          const gamesByRound = new Map<string, RoundGameEntry[]>()
          entries.forEach(([b, games]) => games.forEach(g => {
            const entry = { g, poolName: b.name, poolBracketId: b.pool_bracket_id }
            const list = gamesByRound.get(g.roundid)
            if (list) list.push(entry); else gamesByRound.set(g.roundid, [entry])
          }))
          setDataByDid(prev => ({ ...prev, [did]: { poolBrackets: brackets, gamesByRound } }))
        })
      )
      .catch(() => setError('Failed to load division schedule data.'))
  }, [])

  // Default the active tab to the first division, keeping a still-valid choice.
  useEffect(() => {
    setActive(prev => divisions.some(d => d.did === prev) ? prev : (divisions[0]?.did ?? ''))
  }, [divisions])

  // Eagerly load every division once so all tabs are instant and stay cached for comparison.
  useEffect(() => { divisions.forEach(d => loadDivision(d.did)) }, [divisions, loadDivision])

  if (divisions.length === 0) {
    return (
      <Alert severity="info">This tournament has no divisions yet.</Alert>
    )
  }

  return (
    <Card variant="outlined">
      <CardContent>
        {error && <Alert severity="error" sx={{ mb: 1 }} onClose={() => setError(null)}>{error}</Alert>}
        {/* One tab per division so schedules can be compared side by side. */}
        <Tabs value={active || false} onChange={(_e, v: string) => setActive(v)} variant="scrollable" scrollButtons="auto" sx={{ minHeight: 0 }}>
          {divisions.map(d => <Tab key={d.did} value={d.did} label={d.dname} />)}
        </Tabs>
        <Divider sx={{ mt: 1, mb: 1.5 }} />

        {/* Every loaded division stays mounted (hidden unless active) so its data stays cached. */}
        {divisions.map(d => {
          const data = dataByDid[d.did]
          return (
            <Box key={d.did} hidden={d.did !== active}>
              {data ? (
                <RoundsManager
                  tid={tid}
                  did={d.did}
                  canEdit={canEdit}
                  roundDurationMinutes={roundDurationMinutes}
                  gamesByRound={data.gamesByRound}
                  onNavigateGame={(gid) => navigate(`/game/${gid}/overview`)}
                  onGamesChanged={() => loadDivision(d.did)}
                  poolBrackets={data.poolBrackets}
                  onPoolsChanged={() => loadDivision(d.did)}
                />
              ) : (
                <Typography variant="body2" color="text.secondary">Loading…</Typography>
              )}
            </Box>
          )
        })}
      </CardContent>
    </Card>
  )
}

// ── Read view ────────────────────────────────────────────────────────────────

/** "Left v Center v Right" from a game row's team names. */
const matchupText = (g: GameRowTS): string =>
  [g.left_team_name, g.center_team_name, g.right_team_name].filter(Boolean).join(' v ')

const startTimeText = (g: GameRowTS): string =>
  g.scheduled_start_time ? dayjs(g.scheduled_start_time).format('MMM D, YYYY h:mm A') : 'Unscheduled'

const contextText = (g: GameRowTS): string =>
  [g.division_name, g.room_name, g.round_number != null ? `Round ${g.round_number}` : null].filter(Boolean).join(' • ')

/** Matchup text linked to the game's profile. */
const MatchupLink = ({ g, onNavigateGame }: { g: GameRowTS; onNavigateGame: (gid: string) => void }) => (
  <Box
    component="span"
    onClick={() => onNavigateGame(g.gid)}
    sx={{ cursor: 'pointer', color: 'primary.main', textDecoration: 'underline', fontWeight: 600 }}
  >
    {matchupText(g)}
  </Box>
)

/** A stacked list of game cards: matchup (links to the game), scheduled start time, and context. */
const GameCards = ({ games, onNavigateGame }: { games: GameRowTS[]; onNavigateGame: (gid: string) => void }) => {
  if (games.length === 0) return <Typography variant="body2" color="text.secondary">No games.</Typography>
  return (
    <Stack spacing={1}>
      {games.map(g => (
        <Card key={g.gid} variant="outlined">
          <CardContent sx={{ py: 1.5, '&:last-child': { pb: 1.5 } }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', gap: 1, flexWrap: 'wrap', alignItems: 'baseline' }}>
              <MatchupLink g={g} onNavigateGame={onNavigateGame} />
              <Typography variant="body2" color="text.secondary">{startTimeText(g)}</Typography>
            </Box>
            <Typography variant="caption" color="text.secondary">{contextText(g)}</Typography>
          </CardContent>
        </Card>
      ))}
    </Stack>
  )
}

/**
 * Like GameCards, but each matchup is followed on the same line by the selected person's role in
 * that game — e.g. "(Quizzer for Team X)" — where the team name (not the role text, and not the
 * game link) links to the team's profile.
 */
const PersonGameCards = ({ rows, onNavigateGame, onNavigateTeam }: {
  rows: PersonGameRowTS[]
  onNavigateGame: (gid: string) => void
  onNavigateTeam: (teamId: string) => void
}) => {
  if (rows.length === 0) return <Typography variant="body2" color="text.secondary">No games.</Typography>

  const roleAnnotation = (pr: PersonGameRowTS) => {
    const hasTeam = (pr.person_role === 'Coach' || pr.person_role === 'Quizzer') && pr.person_role_team_id
    return (
      <Typography component="span" variant="body2" color="text.secondary">
        {hasTeam ? (
          <>
            ({pr.person_role} for{' '}
            <Box
              component="span"
              onClick={() => onNavigateTeam(pr.person_role_team_id!)}
              sx={{ cursor: 'pointer', color: 'primary.main', textDecoration: 'underline' }}
            >
              {pr.person_role_team_name}
            </Box>)
          </>
        ) : `(${pr.person_role})`}
      </Typography>
    )
  }

  return (
    <Stack spacing={1}>
      {rows.map(pr => (
        <Card key={pr.gid} variant="outlined">
          <CardContent sx={{ py: 1.5, '&:last-child': { pb: 1.5 } }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', gap: 1, flexWrap: 'wrap', alignItems: 'baseline' }}>
              <Box sx={{ display: 'flex', gap: 1, flexWrap: 'wrap', alignItems: 'baseline' }}>
                <MatchupLink g={pr} onNavigateGame={onNavigateGame} />
                {roleAnnotation(pr)}
              </Box>
              <Typography variant="body2" color="text.secondary">{startTimeText(pr)}</Typography>
            </Box>
            <Typography variant="caption" color="text.secondary">{contextText(pr)}</Typography>
          </CardContent>
        </Card>
      ))}
    </Stack>
  )
}

type ReadFilter = { kind: '' | 'room' | 'person' | 'team' | 'division'; id: string }

/**
 * The read-only view of the schedule. A "Filter By" selector picks one of Room, Person, Team, or a
 * Division; the matching filters appear below it. Room/Person/Team show a single list of matching
 * games (sorted by scheduled start time on the backend, paged via infinite scroll); the Person filter
 * labels each game with the user's role in it. The Division filter instead shows one read-only card
 * (teams + Rooms × Rounds matrix) with every round across all of the division's pools/brackets.
 */
const ScheduleReadView = ({ tid }: { tid: string }) => {
  const navigate = useNavigate()
  const [error, setError] = useState<string | null>(null)

  const [rooms, setRooms] = useState<RoomTS[]>([])
  const [persons, setPersons] = useState<UserTS[]>([])
  const [personRole, setPersonRole] = useState('All')   // narrows the Person dropdown by capacity
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  // Buildings (roomgroups) for the "Building" dropdown that narrows the Room filter; "All" = no narrowing.
  const [roomgroups, setRoomGroups] = useState<RoomGroupTS[]>([])
  const [buildingFilter, setBuildingFilter] = useState('All')

  // The Team filter's Division dropdown (defaulting to "All") narrows which teams the Team dropdown
  // offers; it isn't itself the filter.
  const [teamDiv, setTeamDiv] = useState('All')
  const [teamOptions, setTeamOptions] = useState<{ teamid: string; name: string }[]>([])

  // Division filter: the chosen division's read-only card, built from all of its pools/brackets.
  const [dvDivision, setDvDivision] = useState('')
  const [dvBrackets, setDvBrackets] = useState<PoolBracketTS[]>([])
  const [dvTeams, setDvTeams] = useState<TeamRowTS[]>([])
  const [dvGames, setDvGames] = useState<GameRowTS[]>([])

  // Exactly one filter is active at a time; results are paged in as the user scrolls.
  // Defaults to Person, so a visitor only has to pick someone from the list.
  const [filter, setFilter] = useState<ReadFilter>({ kind: 'person', id: '' })
  const [games, setGames] = useState<GameRowTS[]>([])          // room/team results
  const [personGames, setPersonGames] = useState<PersonGameRowTS[]>([])
  const [totalCount, setTotalCount] = useState(0)
  const [nextPage, setNextPage] = useState(0)
  const [loading, setLoading] = useState(false)

  // `loadingRef` blocks overlapping loads; `requestToken` invalidates in-flight responses from a
  // filter that has since changed; `sentinelRef` is the element the infinite scroll observes.
  const loadingRef = useRef(false)
  const requestToken = useRef(0)
  const sentinelRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    RoomAPI.getByTournament(tid, PAGE, SIZE).then(rs => setRooms([...rs].sort((a, b) => naturalCompare(a.name, b.name)))).catch(() => setError('Failed to load rooms.'))
    RoomGroupAPI.getByTournament(tid).then(rgs => setRoomGroups([...rgs].sort((a, b) => naturalCompare(a.name, b.name)))).catch(() => setError('Failed to load buildings.'))
    DivisionAPI.getByTournament(tid, PAGE, SIZE).then(setDivisions).catch(() => setError('Failed to load divisions.'))
  }, [tid])

  // The Person dropdown's people, narrowed by the selected Role ("All" = everyone).
  useEffect(() => {
    UserAPI.getPersonsByTournament(tid, personRole).then(setPersons).catch(() => setError('Failed to load people.'))
  }, [tid, personRole])

  // Fetch one page of results for the active filter, replacing (page 0) or appending (scroll).
  const loadPage = (pageNum: number, append: boolean) => {
    if (!filter.id || loadingRef.current) return
    loadingRef.current = true
    setLoading(true)
    const token = requestToken.current
    const finish = () => { if (token === requestToken.current) { loadingRef.current = false; setLoading(false) } }
    if (filter.kind === 'person') {
      GameAPI.getRowsByPersonInTournament(tid, filter.id, pageNum, RESULTS_PAGE_SIZE)
        .then(r => {
          if (token !== requestToken.current) return
          setTotalCount(r.count)
          setPersonGames(prev => append ? [...prev, ...r.items] : r.items)
          setNextPage(pageNum + 1)
        })
        .catch(() => setError('Failed to load the person\'s games.'))
        .finally(finish)
    } else {
      const request = filter.kind === 'room'
        ? GameAPI.getRowsByRoom(filter.id, pageNum, RESULTS_PAGE_SIZE)
        : GameAPI.getRowsByTeam(filter.id, pageNum, RESULTS_PAGE_SIZE)
      request
        .then(r => {
          if (token !== requestToken.current) return
          setTotalCount(r.count)
          setGames(prev => append ? [...prev, ...r.items] : r.items)
          setNextPage(pageNum + 1)
        })
        .catch(() => setError(filter.kind === 'room' ? 'Failed to load room games.' : 'Failed to load the team\'s games.'))
        .finally(finish)
    }
  }

  // Reset and load the first page whenever the active filter changes. Bumping the token abandons any
  // in-flight response from the previous filter.
  useEffect(() => {
    requestToken.current += 1
    loadingRef.current = false
    setGames([]); setPersonGames([]); setTotalCount(0); setNextPage(0)
    if (filter.id) loadPage(0, false)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [filter, tid])

  const loadedCount = filter.kind === 'person' ? personGames.length : games.length
  const hasMore = !!filter.id && loadedCount < totalCount

  // Infinite scroll: load the next page as the bottom sentinel scrolls into view.
  useEffect(() => {
    const el = sentinelRef.current
    if (!el || !hasMore) return
    const observer = new IntersectionObserver(
      entries => { if (entries[0].isIntersecting) loadPage(nextPage, true) },
      { rootMargin: '200px' },
    )
    observer.observe(el)
    return () => observer.disconnect()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hasMore, nextPage, filter, loading])

  // The Team dropdown's options: the chosen division's teams, else all of the tournament's teams.
  useEffect(() => {
    if (filter.kind !== 'team') return
    const norm = (arr: { teamid: string; name: string }[]) => arr.map(t => ({ teamid: t.teamid, name: t.name }))
    if (teamDiv !== 'All') {
      TeamAPI.getByDivision(teamDiv, PAGE, SIZE).then(items => setTeamOptions(norm(items))).catch(() => setError('Failed to load teams.'))
    } else {
      TeamAPI.getByTournament(tid, PAGE, SIZE).then(r => setTeamOptions(norm(r.items))).catch(() => setError('Failed to load teams.'))
    }
  }, [filter.kind, teamDiv, tid])

  // Division filter: load the chosen division's pool/brackets.
  useEffect(() => {
    if (!dvDivision) { setDvBrackets([]); return }
    PoolBracketAPI.getByDivision(dvDivision).then(setDvBrackets).catch(() => setError('Failed to load pools/brackets.'))
  }, [dvDivision])

  // The card shows every round, so combine the teams and games of all of the division's
  // pools/brackets. A team placed in several of them is listed once.
  useEffect(() => {
    if (dvBrackets.length === 0) { setDvTeams([]); setDvGames([]); return }
    let cancelled = false
    Promise.all(dvBrackets.map(b => Promise.all([
      TeamAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE),
      GameAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE),
    ])))
      .then(results => {
        if (cancelled) return
        const teamsById = new Map<string, TeamRowTS>()
        results.forEach(([t]) => t.items.forEach(team => { if (!teamsById.has(team.teamid)) teamsById.set(team.teamid, team) }))
        setDvTeams([...teamsById.values()])
        setDvGames(results.flatMap(([, g]) => g.items))
      })
      .catch(() => { if (!cancelled) setError('Failed to load the division\'s schedule.') })
    return () => { cancelled = true }
  }, [dvBrackets])

  // The "Building" dropdown narrows the Room dropdown's options; "All" shows every room. Changing it
  // blanks the current Room selection.
  const roomsForBuilding = buildingFilter === 'All' ? rooms : rooms.filter(r => r.roomgroupid === buildingFilter)
  const onBuildingChange = (v: string) => { setBuildingFilter(v); setFilter({ kind: 'room', id: '' }) }

  // Auto-select a fetched dropdown's lone option so its downstream fetch starts without an extra
  // click. Each is gated to the active Filter By kind and only fires while that dropdown is unset.
  useEffect(() => {
    if (filter.kind === 'room' && !filter.id && roomsForBuilding.length === 1) setFilter({ kind: 'room', id: roomsForBuilding[0].roomid })
  }, [filter.kind, filter.id, roomsForBuilding])
  useEffect(() => {
    if (filter.kind === 'person' && !filter.id && persons.length === 1) setFilter({ kind: 'person', id: persons[0].id })
  }, [filter.kind, filter.id, persons])
  useEffect(() => {
    if (filter.kind === 'team' && !filter.id && teamOptions.length === 1) setFilter({ kind: 'team', id: teamOptions[0].teamid })
  }, [filter.kind, filter.id, teamOptions])
  useEffect(() => {
    if (filter.kind === 'division' && !dvDivision && divisions.length === 1) setDvDivision(divisions[0].did)
  }, [filter.kind, dvDivision, divisions])

  const personLabel = (u: UserTS) => [u.fname, u.mname, u.lname].filter(Boolean).join(' ')
  const goGame = (gid: string) => navigate(`/game/${gid}/overview`)
  const goTeam = (teamId: string) => navigate(`/team/${teamId}/overview`)
  const valueFor = (kind: ReadFilter['kind']) => (filter.kind === kind ? filter.id : '')

  // Changing the Role re-narrows the Person list and blanks the Person selection.
  const onPersonRoleChange = (v: string) => { setPersonRole(v); setFilter({ kind: 'person', id: '' }) }

  // Changing the Division blanks the Team selection.
  const onTeamDivChange = (v: string) => { setTeamDiv(v); setFilter({ kind: 'team', id: '' }) }

  return (
    // `pt` gives the outlined dropdowns' shrunk floating labels room so they aren't clipped at the top
    // edge of the profile's overflow container (notably for visitors, who have no toggle above them).
    <Stack spacing={2} sx={{ textAlign: 'left', pt: 1 }}>
      {error && <Alert severity="error" onClose={() => setError(null)}>{error}</Alert>}

      {/* Dropdowns stacked vertically: "Filter By" then the matching filter's dropdown below it. */}
      <Stack spacing={2} sx={{ alignItems: 'flex-start' }}>
        <FormControl size="small" sx={{ minWidth: 240 }}>
          <InputLabel>Filter By</InputLabel>
          <Select
            label="Filter By"
            value={filter.kind}
            onChange={(e: SelectChangeEvent) => setFilter({ kind: e.target.value as ReadFilter['kind'], id: '' })}
          >
            <MenuItem value="person">Person</MenuItem>
            <MenuItem value="team">Team</MenuItem>
            <MenuItem value="room">Room</MenuItem>
            <MenuItem value="division">Division</MenuItem>
          </Select>
        </FormControl>

        {filter.kind === 'division' && (
          <>
            {/* The division alone picks the card below. */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Division</InputLabel>
              <Select label="Division" value={dvDivision} onChange={(e: SelectChangeEvent) => setDvDivision(e.target.value)}>
                {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            </FormControl>
          </>
        )}

        {filter.kind === 'room' && (
          <>
            {/* Building narrows which rooms the Room dropdown offers; defaults to "All". */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Building</InputLabel>
              <Select label="Building" value={buildingFilter} onChange={(e: SelectChangeEvent) => onBuildingChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                {roomgroups.map(rg => <MenuItem key={rg.roomgroupid} value={rg.roomgroupid}>{rg.name}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Room</InputLabel>
              <Select label="Room" value={valueFor('room')} onChange={(e: SelectChangeEvent) => setFilter({ kind: 'room', id: e.target.value })}>
                {roomsForBuilding.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
              </Select>
            </FormControl>
          </>
        )}
        {filter.kind === 'person' && (
          <>
            {/* Role narrows which people the Person dropdown offers; defaults to "All". */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Role</InputLabel>
              <Select label="Role" value={personRole} onChange={(e: SelectChangeEvent) => onPersonRoleChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                <MenuItem value="quizzer">Quizzer</MenuItem>
                <MenuItem value="coach">Coach</MenuItem>
                <MenuItem value="quizmaster">Quizmaster</MenuItem>
                <MenuItem value="content_judge">Content Judge</MenuItem>
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Person</InputLabel>
              <Select label="Person" value={valueFor('person')} onChange={(e: SelectChangeEvent) => setFilter({ kind: 'person', id: e.target.value })}>
                {persons.map(u => <MenuItem key={u.id} value={u.id}>{personLabel(u)}</MenuItem>)}
              </Select>
            </FormControl>
          </>
        )}
        {filter.kind === 'team' && (
          <>
            {/* Division narrows the Team list; defaults to "All". */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Division</InputLabel>
              <Select label="Division" value={teamDiv} onChange={(e: SelectChangeEvent) => onTeamDivChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Team</InputLabel>
              <Select label="Team" value={valueFor('team')} onChange={(e: SelectChangeEvent) => setFilter({ kind: 'team', id: e.target.value })}>
                {teamOptions.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
              </Select>
            </FormControl>
          </>
        )}
      </Stack>

      {/* Results: for the Division filter, a read-only card of the division's rounds; otherwise a count
          header, the game cards, and an infinite-scroll sentinel + loading note. */}
      {filter.kind === 'division' ? (
        dvDivision
          ? <PoolDetail teams={dvTeams} games={dvGames} canEdit={false} onRemoveTeam={() => {}} onNavigateGame={goGame} />
          : <Typography variant="body2" color="text.secondary">Pick a division to see its rounds.</Typography>
      ) : !filter.kind ? (
        <Typography variant="body2" color="text.secondary">Choose what to filter by above.</Typography>
      ) : !filter.id ? (
        <Typography variant="body2" color="text.secondary">Pick a {filter.kind} to see {filter.kind === 'person' ? 'their' : 'its'} games.</Typography>
      ) : loading && loadedCount === 0 ? (
        <Typography variant="body2" color="text.secondary">Loading…</Typography>
      ) : (
        <Stack spacing={1}>
          <Typography variant="subtitle2">{totalCount} {totalCount === 1 ? 'Game' : 'Games'}</Typography>
          {filter.kind === 'person'
            ? <PersonGameCards rows={personGames} onNavigateGame={goGame} onNavigateTeam={goTeam} />
            : <GameCards games={games} onNavigateGame={goGame} />}
          {hasMore && <div ref={sentinelRef} style={{ height: 1 }} />}
          {loading && loadedCount > 0 && <Typography variant="body2" color="text.secondary">Loading more…</Typography>}
        </Stack>
      )}
    </Stack>
  )
}
