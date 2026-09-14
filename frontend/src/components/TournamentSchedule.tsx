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

  // Row 3 — the selected pool/bracket
  const [selectedBracketId, setSelectedBracketId] = useState('')

  // Teams per pool/bracket in the selected session (drives placement + the card's team list)
  const [teamsByBracket, setTeamsByBracket] = useState<Record<string, TeamRowTS[]>>({})

  // Row 4 — games in the selected pool/bracket (drive the matchup matrix)
  const [cardGames, setCardGames] = useState<GameRowTS[]>([])

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

  const loadGames = useCallback((bracketId: string) => {
    if (!bracketId) { setCardGames([]); return }
    GameAPI.getRowsByPoolBracket(bracketId, PAGE, SIZE)
      .then(r => setCardGames(r.items))
      .catch(() => setError('Failed to load games.'))
  }, [])

  // ── Effects ──────────────────────────────────────────────────────────────

  useEffect(() => { loadDivisions() }, [loadDivisions])

  useEffect(() => {
    setSelectedSessionId('')
    setSelectedBracketId('')
    setSessions([])
    setDivisionBrackets([])
    setDivisionTeams([])
    if (selectedDid) loadDivisionData(selectedDid)
  }, [selectedDid, loadDivisionData])

  useEffect(() => {
    setSelectedBracketId('')
    setTeamsByBracket({})
    if (selectedSessionId) loadPlacement(selectedSessionId, divisionBrackets)
  }, [selectedSessionId, divisionBrackets, loadPlacement])

  useEffect(() => { loadGames(selectedBracketId) }, [selectedBracketId, loadGames])

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

  // The pools/brackets the Row-3 dropdown offers, and whether we're in bracket mode.
  const isBracketMode = sessionType === 'Tournament Bracket(s)'
  const row3Options = isBracketMode ? brackets : pools

  const placedTeamIds = useMemo(
    () => new Set(Object.values(teamsByBracket).flat().map(t => t.teamid)),
    [teamsByBracket]
  )
  const unplacedTeams = useMemo(
    () => divisionTeams.filter(t => !placedTeamIds.has(t.teamid)),
    [divisionTeams, placedTeamIds]
  )

  const selectedBracket = sessionBrackets.find(b => b.pool_bracket_id === selectedBracketId) ?? null
  const cardTeams = teamsByBracket[selectedBracketId] ?? []

  // ── Team placement handlers ──────────────────────────────────────────────

  const refresh = () => {
    loadPlacement(selectedSessionId, divisionBrackets)
    loadGames(selectedBracketId)
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

  const handleRemoveTeam = async (teamid: string) => {
    if (!selectedBracketId) return
    try {
      await TeamAPI.removeFromPoolBracket(selectedBracketId, teamid, accessToken)
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

          {/* Row 2b — teams awaiting placement */}
          {selectedSessionId && (
            <Box>
              <Typography variant="caption" color="text.secondary">
                Teams needing placement{selectedBracketId ? ` — click to add to "${selectedBracket?.name}"` : ' — select a pool/bracket first'}:
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
          )}

          {/* Row 3 / Row 3 Alternate — Pools or Brackets */}
          {selectedSessionId && (
            <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
              {sessionType === 'Undecided' ? (
                <>
                  <Typography variant="body2" color="text.secondary">This session has no pools or brackets yet:</Typography>
                  <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'pool' })} disabled={!canEdit}>
                    Create Pool
                  </Button>
                  <Button startIcon={<AddIcon />} onClick={() => setBracketDialog({ open: true, type: 'bracket' })} disabled={!canEdit}>
                    Create Bracket
                  </Button>
                </>
              ) : (
                <>
                  <FormControl size="small" sx={{ minWidth: 240 }}>
                    <InputLabel>{isBracketMode ? 'Brackets' : 'Pools'}</InputLabel>
                    <Select
                      label={isBracketMode ? 'Brackets' : 'Pools'}
                      value={selectedBracketId}
                      onChange={(e: SelectChangeEvent) => setSelectedBracketId(e.target.value)}
                    >
                      {row3Options.map(b => <MenuItem key={b.pool_bracket_id} value={b.pool_bracket_id}>{b.name}</MenuItem>)}
                    </Select>
                  </FormControl>
                  <Button
                    startIcon={<AddIcon />}
                    onClick={() => setBracketDialog({ open: true, type: isBracketMode ? 'bracket' : 'pool' })}
                    disabled={!canEdit}
                  >
                    Create {isBracketMode ? 'Bracket' : 'Pool'}
                  </Button>
                </>
              )}
            </Box>
          )}

          {/* Row 4 — the pool/bracket card */}
          {selectedBracket && (
            <PoolCard
              bracket={selectedBracket}
              teams={cardTeams}
              games={cardGames}
              canEdit={canEdit}
              onRemoveTeam={handleRemoveTeam}
              onNavigateTeam={(teamid) => navigate(`/team/${teamid}/overview`)}
            />
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

// ── Pool/Bracket card ────────────────────────────────────────────────────────

interface PoolCardProps {
  bracket: PoolBracketTS
  teams: TeamRowTS[]
  games: GameRowTS[]
  canEdit: boolean
  onRemoveTeam: (teamid: string) => void
  onNavigateTeam: (teamid: string) => void
}

/**
 * One pool/bracket: its name, the teams in it (numbered chips), and a Rooms × Rounds matrix of the
 * matchups played in it, rendered with each team's assigned number.
 */
const PoolCard = ({ bracket, teams, games, canEdit, onRemoveTeam, onNavigateTeam }: PoolCardProps) => {
  // Each team gets a 1-based number, used both on its chip and in the matrix cells.
  const numberByTeam = useMemo(() => {
    const m = new Map<string, number>()
    teams.forEach((t, i) => m.set(t.teamid, i + 1))
    return m
  }, [teams])

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

  // A single team's number, with the team name on hover and a click through to its profile.
  const teamToken = (teamid: string, name: string) => {
    const n = numberByTeam.get(teamid)
    return (
      <Tooltip title={name} key={teamid}>
        <Box
          component="span"
          onClick={() => onNavigateTeam(teamid)}
          sx={{ cursor: 'pointer', textDecoration: 'underline', fontWeight: 600, px: 0.25 }}
        >
          {n ?? '—'}
        </Box>
      </Tooltip>
    )
  }

  const matchup = (g: GameRowTS) => {
    const tokens: React.ReactNode[] = [teamToken(g.leftteamid, g.left_team_name)]
    if (g.centerteamid) tokens.push(teamToken(g.centerteamid, g.center_team_name ?? ''))
    tokens.push(teamToken(g.rightteamid, g.right_team_name))
    return tokens.reduce<React.ReactNode[]>((acc, tok, i) =>
      i === 0 ? [tok] : [...acc, <span key={`v${i}`}> v </span>, tok], [])
  }

  return (
    <Card variant="outlined">
      <CardContent>
        {/* Card Row 1 — name */}
        <Typography variant="h6">{bracket.name}</Typography>
        <Divider sx={{ my: 1 }} />

        {/* Card Row 2 — teams (numbered chips) */}
        <Typography variant="caption" color="text.secondary">
          Teams{canEdit ? ' — click a team to remove it from this pool/bracket' : ''}:
        </Typography>
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5, mb: 1 }}>
          {teams.length === 0
            ? <Typography variant="body2" color="text.secondary">No teams yet.</Typography>
            : teams.map((t, i) => (
                <Chip
                  key={t.teamid}
                  label={`${i + 1}. ${t.name}`}
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
      </CardContent>
    </Card>
  )
}
