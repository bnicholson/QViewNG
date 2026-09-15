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
import { DivisionSessionAPI, type DivisionSessionTS } from '../features/DivisionSessionAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI'
import { GameAPI, type GameRowTS, type PersonGameRowTS } from '../features/GameAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { DivisionEditorDialog } from './DivisionEditorDialog'
import { DivisionSessionEditorDialog } from './DivisionSessionEditorDialog'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'

const PAGE = 0
const SIZE = 500
// Read-view results are paged (infinite scroll) rather than fetched all at once.
const RESULTS_PAGE_SIZE = 25

// Numeric-aware string compare so "Room 10" sorts after "Room 9", not right after "Room 1".
const naturalCompare = (a: string, b: string) =>
  a.localeCompare(b, undefined, { numeric: true, sensitivity: 'base' })

type Mode = 'read' | 'conflicts' | 'edit'
type SessionType = 'Round Robin' | 'Tournament Bracket(s)' | 'Undecided'

interface Props {
  tid: string
  /** Whether the current user may create/modify schedule entities. */
  canEdit?: boolean
}

/**
 * A self-contained schedule editor for an entire tournament. Divisions → Division Sessions →
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

  // Row 1 — divisions
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  const [selectedDid, setSelectedDid] = useState('')

  // Row 2 — sessions + the division's pools/brackets and teams
  const [sessions, setSessions] = useState<DivisionSessionTS[]>([])
  const [selectedSessionId, setSelectedSessionId] = useState('')
  const [divisionBrackets, setDivisionBrackets] = useState<PoolBracketTS[]>([])
  const [divisionTeams, setDivisionTeams] = useState<TeamTS[]>([])

  // The "active" pool/bracket — the target the unplaced-team chips add to. With the dropdown gone,
  // every pool is shown as a card and the active one is chosen by clicking its card.
  const [selectedBracketId, setSelectedBracketId] = useState('')

  // Teams and games per pool/bracket in the selected session (drive each card's team list + matrix).
  const [teamsByBracket, setTeamsByBracket] = useState<Record<string, TeamRowTS[]>>({})
  const [gamesByBracket, setGamesByBracket] = useState<Record<string, GameRowTS[]>>({})

  // Create-dialog visibility
  // Create/Edit dialog state — each carries the entity being edited, or null when creating.
  const [divDialog, setDivDialog] = useState<{ open: boolean; division: DivisionTS | null }>({ open: false, division: null })
  const [sessionDialog, setSessionDialog] = useState<{ open: boolean; session: DivisionSessionTS | null }>({ open: false, session: null })
  const [bracketDialog, setBracketDialog] = useState<{ open: boolean; type: 'pool' | 'bracket'; bracket: PoolBracketTS | null }>({ open: false, type: 'pool', bracket: null })

  // ── Loaders ──────────────────────────────────────────────────────────────

  const loadDivisions = useCallback(() => {
    DivisionAPI.getByTournament(tid, PAGE, SIZE)
      .then(setDivisions)
      .catch(() => setError('Failed to load divisions.'))
  }, [tid])

  const loadDivisionData = useCallback((did: string) => {
    DivisionSessionAPI.getByDivision(did).then(setSessions).catch(() => setError('Failed to load sessions.'))
    PoolBracketAPI.getByDivision(did).then(setDivisionBrackets).catch(() => setError('Failed to load pools/brackets.'))
    TeamAPI.getByDivision(did, PAGE, SIZE).then(setDivisionTeams).catch(() => setError('Failed to load teams.'))
  }, [])

  const loadPlacement = useCallback((sessionId: string, brackets: PoolBracketTS[]) => {
    const sessionBrackets = brackets.filter(b => b.division_session_id === sessionId)
    Promise.all(
      sessionBrackets.map(b =>
        TeamAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE).then(r => [b.pool_bracket_id, r.items] as const)
      )
    )
      .then(entries => setTeamsByBracket(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load team placements.'))
  }, [])

  const loadGamesForSession = useCallback((sessionId: string, brackets: PoolBracketTS[]) => {
    const sessionBrackets = brackets.filter(b => b.division_session_id === sessionId)
    Promise.all(
      sessionBrackets.map(b =>
        GameAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE).then(r => [b.pool_bracket_id, r.items] as const)
      )
    )
      .then(entries => setGamesByBracket(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load games.'))
  }, [])

  // ── Effects ──────────────────────────────────────────────────────────────

  useEffect(() => { loadDivisions() }, [loadDivisions])

  // Auto-select the first division as soon as divisions load (keeping any valid current choice), so
  // the cascade below fills in and the user lands on a populated schedule instead of empty dropdowns.
  useEffect(() => {
    setSelectedDid(prev => divisions.some(d => d.did === prev) ? prev : (divisions[0]?.did ?? ''))
  }, [divisions])

  useEffect(() => {
    setSelectedSessionId('')
    setSelectedBracketId('')
    setSessions([])
    setDivisionBrackets([])
    setDivisionTeams([])
    if (selectedDid) loadDivisionData(selectedDid)
  }, [selectedDid, loadDivisionData])

  // Auto-select the first session once the chosen division's sessions load (which in turn auto-selects
  // its first pool/bracket via the row3Options effect below).
  useEffect(() => {
    setSelectedSessionId(prev => sessions.some(s => s.division_session_id === prev) ? prev : (sessions[0]?.division_session_id ?? ''))
  }, [sessions])

  useEffect(() => {
    setTeamsByBracket({})
    setGamesByBracket({})
    if (selectedSessionId) {
      loadPlacement(selectedSessionId, divisionBrackets)
      loadGamesForSession(selectedSessionId, divisionBrackets)
    }
  }, [selectedSessionId, divisionBrackets, loadPlacement, loadGamesForSession])

  // ── Derived values ───────────────────────────────────────────────────────

  const sessionBrackets = useMemo(
    () => divisionBrackets.filter(b => b.division_session_id === selectedSessionId),
    [divisionBrackets, selectedSessionId]
  )
  const pools = useMemo(() => sessionBrackets.filter(b => b.type === 'pool'), [sessionBrackets])
  const brackets = useMemo(() => sessionBrackets.filter(b => b.type === 'bracket'), [sessionBrackets])

  // A session holds either pools (Round Robin) or brackets (Tournament Bracket(s)), never both;
  // until one is added it is Undecided and either kind may be started.
  const sessionType: SessionType =
    pools.length > 0 ? 'Round Robin'
    : brackets.length > 0 ? 'Tournament Bracket(s)'
    : 'Undecided'

  // The pools/brackets shown as cards, and whether we're in bracket mode.
  const isBracketMode = sessionType === 'Tournament Bracket(s)'
  const row3Options = isBracketMode ? brackets : pools

  // Keep the active pool/bracket (the unplaced-team target) valid: default to the first one and
  // fall back to the first whenever the current one disappears (session/division change, deletion).
  useEffect(() => {
    setSelectedBracketId(prev =>
      row3Options.some(b => b.pool_bracket_id === prev) ? prev : (row3Options[0]?.pool_bracket_id ?? '')
    )
  }, [row3Options])

  const placedTeamIds = useMemo(
    () => new Set(Object.values(teamsByBracket).flat().map(t => t.teamid)),
    [teamsByBracket]
  )
  const unplacedTeams = useMemo(
    () => divisionTeams.filter(t => !placedTeamIds.has(t.teamid)),
    [divisionTeams, placedTeamIds]
  )

  const selectedBracket = sessionBrackets.find(b => b.pool_bracket_id === selectedBracketId) ?? null
  const selectedDivision = divisions.find(d => d.did === selectedDid) ?? null
  const selectedSession = sessions.find(s => s.division_session_id === selectedSessionId) ?? null

  // ── Team placement handlers ──────────────────────────────────────────────

  const refresh = () => {
    loadPlacement(selectedSessionId, divisionBrackets)
    loadGamesForSession(selectedSessionId, divisionBrackets)
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
          <Button variant={activeMode === 'edit' ? 'contained' : 'outlined'} onClick={() => setMode('edit')}>Create / Edit</Button>
        </ButtonGroup>
      )}

      {error && <Alert severity="error" onClose={() => setError(null)}>{error}</Alert>}

      {activeMode === 'read' ? (
        <ScheduleReadView tid={tid} />
      ) : activeMode === 'conflicts' ? (
        <Alert severity="info">Scheduling conflict resolution is coming soon.</Alert>
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
            <Button startIcon={<AddIcon />} onClick={() => setDivDialog({ open: true, division: null })} disabled={!canEdit}>
              Create Division
            </Button>
          </Box>

          {/* Row 2a — Session + type + create */}
          <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
            <FormControl size="small" sx={{ minWidth: 240 }} disabled={!selectedDid}>
              <InputLabel>Session</InputLabel>
              <Select
                label="Session"
                value={selectedSessionId}
                onChange={(e: SelectChangeEvent) => setSelectedSessionId(e.target.value)}
              >
                {sessions.map(s => <MenuItem key={s.division_session_id} value={s.division_session_id}>{s.name}</MenuItem>)}
              </Select>
            </FormControl>
            {selectedSessionId && (
              <Typography variant="body2" color="text.secondary">Type: <strong>{sessionType}</strong></Typography>
            )}
            <Button startIcon={<EditIcon />} onClick={() => setSessionDialog({ open: true, session: selectedSession })} disabled={!canEdit || !selectedSession}>
              Edit
            </Button>
            <Button startIcon={<AddIcon />} onClick={() => setSessionDialog({ open: true, session: null })} disabled={!canEdit || !selectedDid}>
              Create Session
            </Button>
          </Box>

          {/* The schedule card: teams awaiting placement on top, then the pool/bracket tabs (with a
              "+" to create another at the end of the row), then the selected pool/bracket's detail. */}
          {selectedSessionId && (
            <Card variant="outlined">
              <CardContent>
                {/* Teams awaiting placement — left aligned, above the tabs */}
                <Box sx={{ textAlign: 'left' }}>
                  <Typography variant="caption" color="text.secondary">
                    Teams needing placement{selectedBracket ? ` — click to add to the selected tab, "${selectedBracket.name}"` : ' — create a pool/bracket first'}:
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

                {sessionType === 'Undecided' ? (
                  // No pools/brackets yet — offer to create the first of either kind.
                  <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
                    <Typography variant="body2" color="text.secondary">This session has no pools or brackets yet:</Typography>
                    <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'pool', bracket: null })} disabled={!canEdit}>
                      Create Pool
                    </Button>
                    <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'bracket', bracket: null })} disabled={!canEdit}>
                      Create Bracket
                    </Button>
                  </Box>
                ) : (
                  <>
                    {/* Tabs row: a "Pools:"/"Brackets:" label, one tab per pool/bracket, then a "+" to create another. */}
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
                            disabled={!canEdit}
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
          loadDivisions()
          setSelectedDid(division.did)
        }}
      />

      <DivisionSessionEditorDialog
        tid={tid}
        lockedDivisionId={selectedDid || undefined}
        session={sessionDialog.session}
        isOpen={sessionDialog.open}
        onCancel={() => setSessionDialog({ open: false, session: null })}
        onSave={(sessionRow) => {
          setSessionDialog({ open: false, session: null })
          if (selectedDid) loadDivisionData(selectedDid)
          setSelectedSessionId(sessionRow.division_session_id)
        }}
      />

      <PoolBracketEditorDialog
        tid={tid}
        did={selectedDid || undefined}
        type={bracketDialog.type}
        entityLabel={bracketDialog.type === 'bracket' ? 'Bracket' : 'Pool'}
        lockedSessionId={selectedSessionId || undefined}
        bracket={bracketDialog.bracket}
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

// ── Pool/Bracket detail ──────────────────────────────────────────────────────

interface PoolDetailProps {
  teams: TeamRowTS[]
  games: GameRowTS[]
  canEdit: boolean
  onRemoveTeam: (teamid: string) => void
  onNavigateGame: (gid: string) => void
}

/**
 * The detail for the selected pool/bracket (its name is shown by its tab): the teams in it (chips)
 * and a Rooms × Rounds matrix of the matchups played in it, rendered with each team's name. Renders
 * inline (no card of its own) since it lives inside the schedule card, below the tabs.
 */
const PoolDetail = ({ teams, games, canEdit, onRemoveTeam, onNavigateGame }: PoolDetailProps) => {
  // Each team gets a 1-based number (its order in the pool), shown on its chip and in the matrix so
  // the compact matchup cells stay unambiguous.
  const numberByTeam = useMemo(() => {
    const m = new Map<string, number>()
    teams.forEach((t, i) => m.set(t.teamid, i + 1))
    return m
  }, [teams])

  // The scheduled start of the pool/bracket's first round — the earliest game start time (the
  // stored timestamps are ISO strings, so a lexical min is the chronological min).
  const firstRoundStart = useMemo(() => {
    const times = games.map(g => g.scheduled_start_time).filter((t): t is string => !!t)
    return times.length === 0 ? null : times.reduce((a, b) => (a < b ? a : b))
  }, [games])

  // Distinct rooms (X axis) and rounds (Y axis) taken from the games in this pool/bracket.
  const rooms = useMemo(() => {
    const m = new Map<string, string>()
    games.forEach(g => m.set(g.roomid, g.room_name))
    return [...m.entries()].sort((a, b) => naturalCompare(a[1], b[1]))
  }, [games])

  const rounds = useMemo(() => {
    const m = new Map<string, number | null>()
    games.forEach(g => m.set(g.roundid, g.round_number))
    return [...m.entries()].sort((a, b) => (a[1] ?? Number.MAX_SAFE_INTEGER) - (b[1] ?? Number.MAX_SAFE_INTEGER))
  }, [games])

  const gamesByCell = useMemo(() => {
    const m = new Map<string, GameRowTS[]>()
    games.forEach(g => {
      const key = `${g.roundid}|${g.roomid}`
      const list = m.get(key)
      if (list) list.push(g); else m.set(key, [g])
    })
    return m
  }, [games])

  // A single team's "N: name" (plain text — the whole matchup cell is the link, see below).
  const teamToken = (teamid: string, name: string) => {
    const n = numberByTeam.get(teamid)
    return <span key={teamid}>{n != null ? `${n}: ${name}` : name}</span>
  }

  // The whole cell's matchup text is one link to that game's profile.
  const matchup = (g: GameRowTS) => {
    const tokens: React.ReactNode[] = [teamToken(g.leftteamid, g.left_team_name)]
    if (g.centerteamid) tokens.push(teamToken(g.centerteamid, g.center_team_name ?? ''))
    tokens.push(teamToken(g.rightteamid, g.right_team_name))
    const withSeparators = tokens.reduce<React.ReactNode[]>((acc, tok, i) =>
      i === 0 ? [tok] : [...acc, <span key={`v${i}`}> v </span>, tok], [])
    return (
      <Box
        component="span"
        onClick={() => onNavigateGame(g.gid)}
        sx={{ cursor: 'pointer', color: 'primary.main', textDecoration: 'underline' }}
      >
        {withSeparators}
      </Box>
    )
  }

  return (
    <Box sx={{ mt: 1.5, textAlign: 'left' }}>
        {/* First-round start time (just under the tabs) */}
        <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
          Start time: {firstRoundStart ? dayjs(firstRoundStart).format('MMM D, YYYY h:mm A') : '—'}
        </Typography>

        {/* Teams in this pool/bracket (chips) */}
        <Typography variant="caption" color="text.secondary">
          Teams{canEdit ? ' — click a team to remove it from this pool/bracket' : ''}:
        </Typography>
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5, mb: 1 }}>
          {teams.length === 0
            ? <Typography variant="body2" color="text.secondary">No teams yet.</Typography>
            : teams.map((t, i) => (
                <Chip
                  key={t.teamid}
                  label={`${i + 1}: ${t.name}`}
                  size="small"
                  onClick={canEdit ? () => onRemoveTeam(t.teamid) : undefined}
                  onDelete={canEdit ? () => onRemoveTeam(t.teamid) : undefined}
                />
              ))
          }
        </Box>

        {/* Card Row 3 — Rooms × Rounds matchup matrix */}
        {games.length === 0 ? (
          <Typography variant="body2" color="text.secondary">No games scheduled.</Typography>
        ) : (
          <TableContainer component={Paper} variant="outlined" sx={{ overflowX: 'auto' }}>
            <Table size="small">
              <TableHead>
                <TableRow>
                  <TableCell sx={{ fontWeight: 600 }}>Round \ Room</TableCell>
                  {rooms.map(([roomid, roomName]) => (
                    <TableCell key={roomid} align="center" sx={{ fontWeight: 600 }}>{roomName}</TableCell>
                  ))}
                </TableRow>
              </TableHead>
              <TableBody>
                {rounds.map(([roundid, roundNumber]) => (
                  <TableRow key={roundid}>
                    <TableCell sx={{ fontWeight: 600 }}>{roundNumber != null ? `Round ${roundNumber}` : 'Round'}</TableCell>
                    {rooms.map(([roomid]) => {
                      const cellGames = gamesByCell.get(`${roundid}|${roomid}`) ?? []
                      return (
                        <TableCell key={roomid} align="center">
                          {cellGames.map((g, i) => (
                            <Box key={g.gid} sx={{ whiteSpace: 'nowrap' }}>
                              {i > 0 && <Divider sx={{ my: 0.5 }} />}
                              {matchup(g)}
                            </Box>
                          ))}
                        </TableCell>
                      )
                    })}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        )}
    </Box>
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
 * Division→Session→Pool drill-down; the matching filters appear below it. Room/Person/Team show a
 * single list of matching games (sorted by scheduled start time on the backend, paged via infinite
 * scroll); the Person filter labels each game with the user's role in it. The Division drill-down
 * instead shows the chosen pool/bracket's read-only card (teams + Rooms × Rounds matrix).
 */
const ScheduleReadView = ({ tid }: { tid: string }) => {
  const navigate = useNavigate()
  const [error, setError] = useState<string | null>(null)

  const [rooms, setRooms] = useState<RoomTS[]>([])
  const [persons, setPersons] = useState<UserTS[]>([])
  const [personRole, setPersonRole] = useState('All')   // narrows the Person dropdown by capacity
  const [divisions, setDivisions] = useState<DivisionTS[]>([])

  // Team-filter cascade (Division → Session → Pool/Bracket), each defaulting to "All". These narrow
  // which teams the Team dropdown offers; they aren't themselves the filter.
  const [teamDiv, setTeamDiv] = useState('All')
  const [teamSession, setTeamSession] = useState('All')
  const [teamPool, setTeamPool] = useState('All')
  const [sessionsForDiv, setSessionsForDiv] = useState<DivisionSessionTS[]>([])
  const [bracketsForDiv, setBracketsForDiv] = useState<PoolBracketTS[]>([])
  const [teamOptions, setTeamOptions] = useState<{ teamid: string; name: string }[]>([])

  // Division drill-down (Filter By = "Division | Session | Pool"): Division → Session → Pool/Bracket,
  // ending in a read-only card for the chosen pool/bracket.
  const [dvDivision, setDvDivision] = useState('')
  const [dvSession, setDvSession] = useState('')
  const [dvBracket, setDvBracket] = useState('')
  const [dvSessions, setDvSessions] = useState<DivisionSessionTS[]>([])
  const [dvBrackets, setDvBrackets] = useState<PoolBracketTS[]>([])
  const [dvTeams, setDvTeams] = useState<TeamRowTS[]>([])
  const [dvGames, setDvGames] = useState<GameRowTS[]>([])

  // Exactly one filter is active at a time; results are paged in as the user scrolls.
  const [filter, setFilter] = useState<ReadFilter>({ kind: '', id: '' })
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

  // Team cascade: when a division is chosen, load its sessions and pool/brackets (for the Session and
  // Pool/Bracket dropdowns). "All" clears them.
  useEffect(() => {
    if (teamDiv === 'All') { setSessionsForDiv([]); setBracketsForDiv([]); return }
    DivisionSessionAPI.getByDivision(teamDiv).then(setSessionsForDiv).catch(() => setError('Failed to load sessions.'))
    PoolBracketAPI.getByDivision(teamDiv).then(setBracketsForDiv).catch(() => setError('Failed to load pools/brackets.'))
  }, [teamDiv])

  // The Team dropdown's options, narrowed by the most specific cascade selection: a pool/bracket's
  // teams, else the session's teams (union across its pool/brackets), else the division's teams, else
  // all of the tournament's teams.
  const sessionBracketsForTeam = useMemo(
    () => bracketsForDiv.filter(b => b.division_session_id === teamSession),
    [bracketsForDiv, teamSession],
  )
  useEffect(() => {
    if (filter.kind !== 'team') return
    const norm = (arr: { teamid: string; name: string }[]) => arr.map(t => ({ teamid: t.teamid, name: t.name }))
    if (teamPool !== 'All') {
      TeamAPI.getRowsByPoolBracket(teamPool, PAGE, SIZE).then(r => setTeamOptions(norm(r.items))).catch(() => setError('Failed to load teams.'))
    } else if (teamSession !== 'All') {
      Promise.all(sessionBracketsForTeam.map(b => TeamAPI.getRowsByPoolBracket(b.pool_bracket_id, PAGE, SIZE)))
        .then(results => {
          const byId = new Map<string, { teamid: string; name: string }>()
          results.forEach(r => r.items.forEach(t => byId.set(t.teamid, { teamid: t.teamid, name: t.name })))
          setTeamOptions([...byId.values()])
        })
        .catch(() => setError('Failed to load teams.'))
    } else if (teamDiv !== 'All') {
      TeamAPI.getByDivision(teamDiv, PAGE, SIZE).then(items => setTeamOptions(norm(items))).catch(() => setError('Failed to load teams.'))
    } else {
      TeamAPI.getByTournament(tid, PAGE, SIZE).then(r => setTeamOptions(norm(r.items))).catch(() => setError('Failed to load teams.'))
    }
  }, [filter.kind, teamDiv, teamSession, teamPool, sessionBracketsForTeam, tid])

  // Division drill-down: load the chosen division's sessions and pool/brackets.
  useEffect(() => {
    if (!dvDivision) { setDvSessions([]); setDvBrackets([]); return }
    DivisionSessionAPI.getByDivision(dvDivision).then(setDvSessions).catch(() => setError('Failed to load sessions.'))
    PoolBracketAPI.getByDivision(dvDivision).then(setDvBrackets).catch(() => setError('Failed to load pools/brackets.'))
  }, [dvDivision])

  const dvSessionBrackets = useMemo(
    () => dvBrackets.filter(b => b.division_session_id === dvSession),
    [dvBrackets, dvSession],
  )

  // Division drill-down: once a specific pool/bracket is chosen, load its teams + games for the card.
  useEffect(() => {
    if (!dvBracket) { setDvTeams([]); setDvGames([]); return }
    TeamAPI.getRowsByPoolBracket(dvBracket, PAGE, SIZE).then(r => setDvTeams(r.items)).catch(() => setError('Failed to load teams.'))
    GameAPI.getRowsByPoolBracket(dvBracket, PAGE, SIZE).then(r => setDvGames(r.items)).catch(() => setError('Failed to load games.'))
  }, [dvBracket])

  // Auto-select a fetched dropdown's lone option so its downstream fetch starts without an extra
  // click. Each is gated to the active Filter By kind and only fires while that dropdown is unset.
  useEffect(() => {
    if (filter.kind === 'room' && !filter.id && rooms.length === 1) setFilter({ kind: 'room', id: rooms[0].roomid })
  }, [filter.kind, filter.id, rooms])
  useEffect(() => {
    if (filter.kind === 'person' && !filter.id && persons.length === 1) setFilter({ kind: 'person', id: persons[0].id })
  }, [filter.kind, filter.id, persons])
  useEffect(() => {
    if (filter.kind === 'team' && !filter.id && teamOptions.length === 1) setFilter({ kind: 'team', id: teamOptions[0].teamid })
  }, [filter.kind, filter.id, teamOptions])
  useEffect(() => {
    if (filter.kind === 'division' && !dvDivision && divisions.length === 1) setDvDivision(divisions[0].did)
  }, [filter.kind, dvDivision, divisions])
  useEffect(() => {
    if (filter.kind === 'division' && dvDivision && !dvSession && dvSessions.length === 1) setDvSession(dvSessions[0].division_session_id)
  }, [filter.kind, dvDivision, dvSession, dvSessions])
  useEffect(() => {
    if (filter.kind === 'division' && dvSession && !dvBracket && dvSessionBrackets.length === 1) setDvBracket(dvSessionBrackets[0].pool_bracket_id)
  }, [filter.kind, dvSession, dvBracket, dvSessionBrackets])

  const personLabel = (u: UserTS) => [u.fname, u.mname, u.lname].filter(Boolean).join(' ')
  const goGame = (gid: string) => navigate(`/game/${gid}/overview`)
  const goTeam = (teamId: string) => navigate(`/team/${teamId}/overview`)
  const valueFor = (kind: ReadFilter['kind']) => (filter.kind === kind ? filter.id : '')

  // Changing the Role re-narrows the Person list and blanks the Person selection.
  const onPersonRoleChange = (v: string) => { setPersonRole(v); setFilter({ kind: 'person', id: '' }) }

  // Changing any cascade dropdown resets those below it and blanks the Team selection.
  const onTeamDivChange = (v: string) => { setTeamDiv(v); setTeamSession('All'); setTeamPool('All'); setFilter({ kind: 'team', id: '' }) }
  const onTeamSessionChange = (v: string) => { setTeamSession(v); setTeamPool('All'); setFilter({ kind: 'team', id: '' }) }
  const onTeamPoolChange = (v: string) => { setTeamPool(v); setFilter({ kind: 'team', id: '' }) }

  // Division drill-down: changing a level resets the ones below it.
  const onDvDivisionChange = (v: string) => { setDvDivision(v); setDvSession(''); setDvBracket('') }
  const onDvSessionChange = (v: string) => { setDvSession(v); setDvBracket('') }
  const onDvBracketChange = (v: string) => { setDvBracket(v) }

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
            <MenuItem value="division">Division | Session | Pool/Bracket</MenuItem>
          </Select>
        </FormControl>

        {filter.kind === 'division' && (
          <>
            {/* Drill down Division → Session → Pool/Bracket to view that pool/bracket's card below. */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Division</InputLabel>
              <Select label="Division" value={dvDivision} onChange={(e: SelectChangeEvent) => onDvDivisionChange(e.target.value)}>
                {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }} disabled={!dvDivision}>
              <InputLabel>Session</InputLabel>
              <Select label="Session" value={dvSession} onChange={(e: SelectChangeEvent) => onDvSessionChange(e.target.value)}>
                {dvSessions.map(s => <MenuItem key={s.division_session_id} value={s.division_session_id}>{s.name}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }} disabled={!dvSession}>
              <InputLabel>Pool/Bracket</InputLabel>
              <Select label="Pool/Bracket" value={dvBracket} onChange={(e: SelectChangeEvent) => onDvBracketChange(e.target.value)}>
                {dvSessionBrackets.map(b => <MenuItem key={b.pool_bracket_id} value={b.pool_bracket_id}>{b.name}</MenuItem>)}
              </Select>
            </FormControl>
          </>
        )}

        {filter.kind === 'room' && (
          <FormControl size="small" sx={{ minWidth: 240 }}>
            <InputLabel>Room</InputLabel>
            <Select label="Room" value={valueFor('room')} onChange={(e: SelectChangeEvent) => setFilter({ kind: 'room', id: e.target.value })}>
              {rooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
            </Select>
          </FormControl>
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
            {/* Division → Session → Pool/Bracket narrow the Team list; each defaults to "All". */}
            <FormControl size="small" sx={{ minWidth: 240 }}>
              <InputLabel>Division</InputLabel>
              <Select label="Division" value={teamDiv} onChange={(e: SelectChangeEvent) => onTeamDivChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }} disabled={teamDiv === 'All'}>
              <InputLabel>Session</InputLabel>
              <Select label="Session" value={teamSession} onChange={(e: SelectChangeEvent) => onTeamSessionChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                {sessionsForDiv.map(s => <MenuItem key={s.division_session_id} value={s.division_session_id}>{s.name}</MenuItem>)}
              </Select>
            </FormControl>
            <FormControl size="small" sx={{ minWidth: 240 }} disabled={teamSession === 'All'}>
              <InputLabel>Pool/Bracket</InputLabel>
              <Select label="Pool/Bracket" value={teamPool} onChange={(e: SelectChangeEvent) => onTeamPoolChange(e.target.value)}>
                <MenuItem value="All">All</MenuItem>
                {sessionBracketsForTeam.map(b => <MenuItem key={b.pool_bracket_id} value={b.pool_bracket_id}>{b.name}</MenuItem>)}
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

      {/* Results: for the Division drill-down, a read-only pool/bracket card; otherwise a count
          header, the game cards, and an infinite-scroll sentinel + loading note. */}
      {filter.kind === 'division' ? (
        dvBracket
          ? <PoolDetail teams={dvTeams} games={dvGames} canEdit={false} onRemoveTeam={() => {}} onNavigateGame={goGame} />
          : <Typography variant="body2" color="text.secondary">Pick a division, session, and pool/bracket to see its card.</Typography>
      ) : !filter.kind ? (
        <Typography variant="body2" color="text.secondary">Choose what to filter by above.</Typography>
      ) : !filter.id ? (
        <Typography variant="body2" color="text.secondary">Pick a {filter.kind} to see its games.</Typography>
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
