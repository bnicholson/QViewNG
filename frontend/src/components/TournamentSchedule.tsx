import { useCallback, useEffect, useMemo, useState } from 'react'
import { useNavigate } from 'react-router-dom'
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

import { useAuth } from '../hooks/useAuth'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { DivisionSessionAPI, type DivisionSessionTS } from '../features/DivisionSessionAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI'
import { GameAPI, type GameRowTS } from '../features/GameAPI'
import { DivisionEditorDialog } from './DivisionEditorDialog'
import { DivisionSessionEditorDialog } from './DivisionSessionEditorDialog'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'

const PAGE = 0
const SIZE = 500

type Mode = 'read' | 'edit'
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

  const [mode, setMode] = useState<Mode>('edit')
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
  const [divDialogOpen, setDivDialogOpen] = useState(false)
  const [sessionDialogOpen, setSessionDialogOpen] = useState(false)
  const [bracketDialog, setBracketDialog] = useState<{ open: boolean; type: 'pool' | 'bracket' }>({ open: false, type: 'pool' })

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

  return (
    <Stack spacing={2}>
      {/* Read / Edit toggle */}
      <ButtonGroup variant="outlined" size="small">
        <Button variant={mode === 'read' ? 'contained' : 'outlined'} onClick={() => setMode('read')}>Read</Button>
        <Button variant={mode === 'edit' ? 'contained' : 'outlined'} onClick={() => setMode('edit')}>Edit</Button>
      </ButtonGroup>

      {error && <Alert severity="error" onClose={() => setError(null)}>{error}</Alert>}

      {mode === 'read' ? (
        <Alert severity="info">The read-only view is coming next. Switch to <strong>Edit</strong> to make changes.</Alert>
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
            <Button startIcon={<AddIcon />} onClick={() => setDivDialogOpen(true)} disabled={!canEdit}>
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
            <Button startIcon={<AddIcon />} onClick={() => setSessionDialogOpen(true)} disabled={!canEdit || !selectedDid}>
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
                    <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'pool' })} disabled={!canEdit}>
                      Create Pool
                    </Button>
                    <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'bracket' })} disabled={!canEdit}>
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
                      <Tooltip title={`Create ${isBracketMode ? 'Bracket' : 'Pool'}`}>
                        <span>
                          <IconButton
                            size="small"
                            onClick={() => setBracketDialog({ open: true, type: isBracketMode ? 'bracket' : 'pool' })}
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
                        onNavigateTeam={(teamid) => navigate(`/team/${teamid}/overview`)}
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
        isOpen={divDialogOpen}
        onCancel={() => setDivDialogOpen(false)}
        onSave={(division) => {
          setDivDialogOpen(false)
          loadDivisions()
          setSelectedDid(division.did)
        }}
      />

      <DivisionSessionEditorDialog
        tid={tid}
        lockedDivisionId={selectedDid || undefined}
        isOpen={sessionDialogOpen}
        onCancel={() => setSessionDialogOpen(false)}
        onSave={(sessionRow) => {
          setSessionDialogOpen(false)
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
        isOpen={bracketDialog.open}
        onCancel={() => setBracketDialog(d => ({ ...d, open: false }))}
        onSave={(bracket) => {
          setBracketDialog(d => ({ ...d, open: false }))
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
  onNavigateTeam: (teamid: string) => void
}

/**
 * The detail for the selected pool/bracket (its name is shown by its tab): the teams in it (chips)
 * and a Rooms × Rounds matrix of the matchups played in it, rendered with each team's name. Renders
 * inline (no card of its own) since it lives inside the schedule card, below the tabs.
 */
const PoolDetail = ({ teams, games, canEdit, onRemoveTeam, onNavigateTeam }: PoolDetailProps) => {
  // Distinct rooms (X axis) and rounds (Y axis) taken from the games in this pool/bracket.
  const rooms = useMemo(() => {
    const m = new Map<string, string>()
    games.forEach(g => m.set(g.roomid, g.room_name))
    return [...m.entries()].sort((a, b) => a[1].localeCompare(b[1]))
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

  // A single team's name, clickable through to its profile.
  const teamToken = (teamid: string, name: string) => (
    <Box
      component="span"
      key={teamid}
      onClick={() => onNavigateTeam(teamid)}
      sx={{ cursor: 'pointer', color: 'primary.main', textDecoration: 'underline', px: 0.25, whiteSpace: 'nowrap' }}
    >
      {name}
    </Box>
  )

  const matchup = (g: GameRowTS) => {
    const tokens: React.ReactNode[] = [teamToken(g.leftteamid, g.left_team_name)]
    if (g.centerteamid) tokens.push(teamToken(g.centerteamid, g.center_team_name ?? ''))
    tokens.push(teamToken(g.rightteamid, g.right_team_name))
    return tokens.reduce<React.ReactNode[]>((acc, tok, i) =>
      i === 0 ? [tok] : [...acc, <span key={`v${i}`}> v </span>, tok], [])
  }

  return (
    <Box sx={{ mt: 1.5, textAlign: 'left' }}>
        {/* Teams in this pool/bracket (chips) */}
        <Typography variant="caption" color="text.secondary">
          Teams{canEdit ? ' — click a team to remove it from this pool/bracket' : ''}:
        </Typography>
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5, mb: 1 }}>
          {teams.length === 0
            ? <Typography variant="body2" color="text.secondary">No teams yet.</Typography>
            : teams.map(t => (
                <Chip
                  key={t.teamid}
                  label={t.name}
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
