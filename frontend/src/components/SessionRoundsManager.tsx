import { useState, useEffect, useCallback, useMemo, type Dispatch, type SetStateAction } from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import Chip from '@mui/material/Chip'
import Alert from '@mui/material/Alert'
import Snackbar from '@mui/material/Snackbar'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import DragIndicatorIcon from '@mui/icons-material/DragIndicator'
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward'
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward'
import VisibilityIcon from '@mui/icons-material/Visibility'
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff'
import dayjs from 'dayjs'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { RoundGroupAPI, type RoundGroupTS } from '../features/RoundGroupAPI'
import { type GameRowTS } from '../features/GameAPI'
import { GameAPI } from '../features/GameAPI'
import { type PoolBracketTS } from '../features/PoolBracketAPI'
import { RoundEditorDialog } from './RoundEditorDialog'
import { RoundGroupEditorDialog } from './RoundGroupEditorDialog'
import { GameEditorDialog } from './GameEditorDialog'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'
import { useAuth } from '../hooks/useAuth'

const PAGE = 0
const SIZE = 500

// Each card "type" gets its own colour so the concentric nesting reads at a glance. Rounds use amber
// (not green) so they don't clash with the teal/green font of the linked games shown inside them.
const SESSION_CARD = { bg: '#eef2ff', border: '#c7d2fe', label: '#3730a3' }   // indigo
const ROUND_CARD = { bg: '#fff7ed', border: '#fed7aa', label: '#c2410c' }     // orange

/** One game as shown under a round in the TimePool variant, with the pool it belongs to. */
export interface RoundGameEntry {
  g: GameRowTS
  poolName: string
  /** The game's pool bracket id (GameRowTS doesn't carry it) — needed to edit the game. */
  poolBracketId: string
}

function formatStart(iso: string | null): string {
  return iso ? dayjs(iso).format('MMM D, YYYY · h:mm A') : 'Unscheduled'
}

/** "Left v Center v Right" from a game row's team names. */
function matchupText(g: GameRowTS): string {
  return [g.left_team_name, g.center_team_name, g.right_team_name].filter(Boolean).join(' v ')
}

// Sort rounds by scheduled start (unscheduled last), then name.
function byStartThenName(a: RoundTS, b: RoundTS): number {
  if (a.scheduled_start_time && b.scheduled_start_time) {
    if (a.scheduled_start_time !== b.scheduled_start_time) return a.scheduled_start_time < b.scheduled_start_time ? -1 : 1
  } else if (a.scheduled_start_time) return -1
  else if (b.scheduled_start_time) return 1
  return a.name.localeCompare(b.name, undefined, { numeric: true })
}

interface Props {
  tid: string
  /** The division whose sessions/rounds are managed. */
  did: string
  /** The division's sessions (roundgroups). */
  roundgroups: RoundGroupTS[]
  canEdit: boolean
  /** Called after a session is created/renamed so the parent can reload the sessions list. */
  onRoundGroupsChanged: () => void
  /**
   * 'timing' (default) is the plain Sessions/Rounds manager with drag-and-drop.
   * 'timepool' also shows each round's games and moves rounds between adjacent sessions with
   * up/down arrows at the session edges instead of drag-and-drop.
   */
  variant?: 'timing' | 'timepool'
  /** TimePool only: games grouped by round id (with their pool name), shown under each round. */
  gamesByRound?: Map<string, RoundGameEntry[]>
  /** TimePool only: navigate to a game's profile when its matchup is clicked. */
  onNavigateGame?: (gid: string) => void
  /** TimePool only: called after a game is added/edited/deleted so the parent can reload games. */
  onGamesChanged?: () => void
  /** TimePool only: the division's pools/brackets, shown as editable chips above the sessions. */
  poolBrackets?: PoolBracketTS[]
  /** TimePool only: called after a pool/bracket is edited so the parent can reload it. */
  onPoolsChanged?: () => void
}

export default function SessionRoundsManager({ tid, did, roundgroups, canEdit, onRoundGroupsChanged, variant = 'timing', gamesByRound, onNavigateGame, onGamesChanged, poolBrackets, onPoolsChanged }: Props) {
  const { accessToken } = useAuth()
  const isTimePool = variant === 'timepool'
  const [rounds, setRounds] = useState<RoundTS[]>([])
  const [notice, setNotice] = useState<string | null>(null)
  const [draggingId, setDraggingId] = useState<string | null>(null)
  const [dragOverSession, setDragOverSession] = useState<string | null>(null)

  // Round create/edit dialog: `session` = the session to create in; `round` = the round being edited.
  const [roundDialog, setRoundDialog] = useState<{ open: boolean; sessionId: string; round: RoundTS | null }>({ open: false, sessionId: '', round: null })
  // Session create/edit dialog.
  const [sessionDialog, setSessionDialog] = useState<{ open: boolean; roundgroup: RoundGroupTS | null }>({ open: false, roundgroup: null })
  // Game add/edit dialog (TimePool): `game` = the game being edited (null = add to `roundId`).
  const [gameDialog, setGameDialog] = useState<{ open: boolean; roundGroupId: string; roundId: string; game: GameRowTS | null; poolBracketId?: string }>({ open: false, roundGroupId: '', roundId: '', game: null })
  const closeGameDialog = () => setGameDialog({ open: false, roundGroupId: '', roundId: '', game: null })
  // Pool/bracket edit dialog (TimePool): opened from the pencil on the chips above the sessions.
  const [poolDialog, setPoolDialog] = useState<{ open: boolean; bracket: PoolBracketTS | null }>({ open: false, bracket: null })
  // Pools whose games are hidden below (client-side visual filter toggled by clicking a pool chip).
  const [hiddenPoolIds, setHiddenPoolIds] = useState<Set<string>>(new Set())
  const [hiddenRoomIds, setHiddenRoomIds] = useState<Set<string>>(new Set())
  const [hiddenTeamIds, setHiddenTeamIds] = useState<Set<string>>(new Set())
  // Toggle an id in one of the visual-filter sets (clicking a Pool/Room/Team chip hides it).
  const makeToggle = (setter: Dispatch<SetStateAction<Set<string>>>) => (id: string) =>
    setter(prev => {
      const next = new Set(prev)
      if (next.has(id)) next.delete(id); else next.add(id)
      return next
    })
  const togglePoolFilter = makeToggle(setHiddenPoolIds)
  const toggleRoomFilter = makeToggle(setHiddenRoomIds)
  const toggleTeamFilter = makeToggle(setHiddenTeamIds)

  const loadRounds = useCallback(() => {
    RoundAPI.getByDivision(did, PAGE, SIZE)
      .then(setRounds)
      .catch(() => setNotice('Failed to load rounds.'))
  }, [did])

  useEffect(() => { loadRounds() }, [loadRounds])

  const roundsBySession = useMemo(() => {
    const m = new Map<string, RoundTS[]>()
    rounds.forEach(r => { const list = m.get(r.roundgroup_id); if (list) list.push(r); else m.set(r.roundgroup_id, [r]) })
    m.forEach(list => list.sort(byStartThenName))
    return m
  }, [rounds])

  const handleDelete = async (round: RoundTS) => {
    try {
      await RoundAPI.delete(round.roundid, accessToken)
      loadRounds()
    } catch (e) {
      // Surfaces the backend's "a game is associated with it" guard message.
      setNotice(e instanceof Error ? e.message : 'Failed to delete round.')
    }
  }

  const handleDeleteSession = async (session: RoundGroupTS) => {
    try {
      await RoundGroupAPI.delete(session.roundgroup_id, accessToken)
      onRoundGroupsChanged()
      loadRounds()
    } catch (e) {
      // Surfaces the backend's "still has rounds" guard message.
      setNotice(e instanceof Error ? e.message : 'Failed to delete session.')
    }
  }

  // Move a round to another session (persists immediately). Used by both drag-and-drop (timing) and
  // the up/down arrows (timepool). The round keeps its start time, so the overall sequence is preserved.
  const moveRound = async (roundId: string, targetSessionId: string) => {
    const round = rounds.find(r => r.roundid === roundId)
    if (!round || round.roundgroup_id === targetSessionId) return
    try {
      await RoundAPI.update(roundId, { roundgroup_id: targetSessionId }, accessToken)
      loadRounds()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to move round.')
    }
  }

  const handleDeleteGame = async (gid: string) => {
    try {
      await GameAPI.delete(gid, accessToken)
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete game.')
    }
  }

  // Drop a round onto a session → move it there.
  const handleDropOnSession = async (targetSessionId: string, roundId: string) => {
    setDragOverSession(null)
    setDraggingId(null)
    await moveRound(roundId, targetSessionId)
  }

  // Order sessions by their earliest round's start time. Sessions with no (scheduled) rounds sort
  // to the end; ties fall back to name.
  const sortedSessions = useMemo(() => {
    const earliestStart = (roundgroupId: string): string | null => {
      // roundsBySession is already sorted (scheduled first, ascending), so the first scheduled round is the earliest.
      const list = roundsBySession.get(roundgroupId) ?? []
      return list.find(r => r.scheduled_start_time)?.scheduled_start_time ?? null
    }
    return [...roundgroups].sort((a, b) => {
      const ea = earliestStart(a.roundgroup_id)
      const eb = earliestStart(b.roundgroup_id)
      if (ea && eb) { if (ea !== eb) return ea < eb ? -1 : 1 }
      else if (ea) return -1
      else if (eb) return 1
      return a.name.localeCompare(b.name, undefined, { numeric: true })
    })
  }, [roundgroups, roundsBySession])

  // Distinct rooms and teams that appear in the division's games — drive the Rooms/Teams filter chips.
  const roomsInGames = useMemo(() => {
    const m = new Map<string, string>()
    gamesByRound?.forEach(list => list.forEach(({ g }) => m.set(g.roomid, g.room_name)))
    return [...m.entries()].map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }))
  }, [gamesByRound])
  const teamsInGames = useMemo(() => {
    const m = new Map<string, string>()
    gamesByRound?.forEach(list => list.forEach(({ g }) => {
      m.set(g.leftteamid, g.left_team_name)
      if (g.centerteamid) m.set(g.centerteamid, g.center_team_name ?? '')
      m.set(g.rightteamid, g.right_team_name)
    }))
    return [...m.entries()].map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }))
  }, [gamesByRound])

  // A game is hidden if its pool or its room is toggled off, or — for teams — if EVERY team in the
  // game is toggled off. (A matchup stays visible as long as at least one of its teams is visible, so
  // showing a single team surfaces all of that team's games.)
  const gameHiddenByFilter = (e: RoundGameEntry): boolean => {
    if (hiddenPoolIds.has(e.poolBracketId) || hiddenRoomIds.has(e.g.roomid)) return true
    const teamIds = [e.g.leftteamid, e.g.rightteamid, ...(e.g.centerteamid != null ? [e.g.centerteamid] : [])]
    return teamIds.length > 0 && teamIds.every(id => hiddenTeamIds.has(id))
  }

  // Games for a round (visual filters applied), sorted by room name so the list reads in the same
  // room order as the Pools matrix.
  const gamesForRound = (roundId: string): RoundGameEntry[] =>
    [...(gamesByRound?.get(roundId) ?? [])]
      .filter(e => !gameHiddenByFilter(e))
      .sort((a, b) => a.g.room_name.localeCompare(b.g.room_name, undefined, { numeric: true }))

  // A leading "show/hide all" eye chip for a filter row. Open eye when anything is hidden (click reveals
  // all); closed eye when everything is visible (click hides all).
  const allToggleChip = (noun: string, allIds: string[], hiddenIds: Set<string>, setHidden: Dispatch<SetStateAction<Set<string>>>) => {
    const anyHidden = allIds.some(id => hiddenIds.has(id))
    return (
      <Tooltip title={anyHidden ? `Show all ${noun}` : `Hide all ${noun}`}>
        <Chip
          variant="outlined"
          onClick={() => setHidden(anyHidden ? new Set() : new Set(allIds))}
          sx={{ height: 'auto', borderRadius: '8px', cursor: 'pointer', '& .MuiChip-label': { display: 'flex', alignItems: 'center', px: 0.75, py: 0.5 } }}
          label={anyHidden ? <VisibilityIcon sx={{ fontSize: 16 }} /> : <VisibilityOffIcon sx={{ fontSize: 16 }} />}
        />
      </Tooltip>
    )
  }

  return (
    <Box sx={{ textAlign: 'left' }}>
      {isTimePool ? (
        <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
          This tab combines your schedule's time (Sessions and Rounds) with the games scheduled in each round. Use the up/down arrows at the top and bottom edges of a session to move its boundary round into the neighbouring session.
        </Alert>
      ) : (
        <>
          <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
            This tab lets you manage time by modifying your Rounds and Sessions. Use Sessions to organize your Rounds into sequential groups. (There are usually longer-than-normal breaks of no quizzing in between each Session.)
          </Alert>
          <Box sx={{ mb: 1 }}>
            <Typography variant="body2" color="text.secondary">
              Drag a round from one session onto another to move it. Rounds with a game can't be deleted.
            </Typography>
          </Box>
        </>
      )}

      {/* Filters card — client-side show/hide of the games below by Pool, Room, or Team. */}
      {isTimePool && ((poolBrackets?.length ?? 0) > 0 || roomsInGames.length > 0 || teamsInGames.length > 0) && (
        <Card variant="outlined" sx={{ mb: 1.5 }}>
          <CardContent sx={{ pb: '12px !important' }}>
            <Typography variant="subtitle2" sx={{ fontWeight: 700, mb: 1 }}>Filters</Typography>

      {/* Pools/brackets for the division — editable chips above the first session. */}
      {isTimePool && poolBrackets && poolBrackets.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Pools:</Typography>
          {allToggleChip('pools', poolBrackets.map(pb => pb.pool_bracket_id), hiddenPoolIds, setHiddenPoolIds)}
          {poolBrackets.map(pb => {
            // Clicking the chip body toggles whether this pool's games show below; the pencil edits it.
            const hidden = hiddenPoolIds.has(pb.pool_bracket_id)
            return (
              <Chip
                key={pb.pool_bracket_id}
                variant="outlined"
                onClick={() => togglePoolFilter(pb.pool_bracket_id)}
                sx={{
                  height: 'auto',
                  borderRadius: '8px',
                  cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                    <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                      {pb.name}
                    </Typography>
                    {canEdit && (
                      <Tooltip title={`Edit ${pb.type === 'bracket' ? 'bracket' : 'pool'}`}>
                        <IconButton size="small" sx={{ p: 0.25 }} onClick={(e) => { e.stopPropagation(); setPoolDialog({ open: true, bracket: pb }) }}>
                          <EditIcon sx={{ fontSize: 15 }} />
                        </IconButton>
                      </Tooltip>
                    )}
                  </Box>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Rooms filter — click a room chip to show/hide its games below. */}
      {isTimePool && roomsInGames.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Rooms:</Typography>
          {allToggleChip('rooms', roomsInGames.map(r => r.id), hiddenRoomIds, setHiddenRoomIds)}
          {roomsInGames.map(room => {
            const hidden = hiddenRoomIds.has(room.id)
            return (
              <Chip
                key={room.id}
                variant="outlined"
                onClick={() => toggleRoomFilter(room.id)}
                sx={{
                  height: 'auto', borderRadius: '8px', cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                    {room.name}
                  </Typography>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Teams filter — click a team chip to show/hide the games that team plays in. */}
      {isTimePool && teamsInGames.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Teams:</Typography>
          {allToggleChip('teams', teamsInGames.map(t => t.id), hiddenTeamIds, setHiddenTeamIds)}
          {teamsInGames.map(team => {
            const hidden = hiddenTeamIds.has(team.id)
            return (
              <Chip
                key={team.id}
                variant="outlined"
                onClick={() => toggleTeamFilter(team.id)}
                sx={{
                  height: 'auto', borderRadius: '8px', cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                    {team.name}
                  </Typography>
                }
              />
            )
          })}
        </Box>
      )}
          </CardContent>
        </Card>
      )}

      {sortedSessions.length === 0 ? (
        <Typography variant="body2" color="text.secondary">This division has no sessions yet.</Typography>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          {sortedSessions.map((session, sessionIdx) => {
            const sessionRounds = roundsBySession.get(session.roundgroup_id) ?? []
            const isDropTarget = !isTimePool && dragOverSession === session.roundgroup_id && draggingId != null
            const prevSession = sortedSessions[sessionIdx - 1]
            const nextSession = sortedSessions[sessionIdx + 1]
            // Arrows only move a session's boundary round into an adjacent session.
            const canMoveUp = isTimePool && canEdit && prevSession != null && sessionRounds.length > 0
            const canMoveDown = isTimePool && canEdit && nextSession != null && sessionRounds.length > 0
            return (
              <Box
                key={session.roundgroup_id}
                onDragOver={!isTimePool ? (e) => { e.preventDefault(); setDragOverSession(session.roundgroup_id) } : undefined}
                onDragLeave={!isTimePool ? () => setDragOverSession(prev => prev === session.roundgroup_id ? null : prev) : undefined}
                onDrop={!isTimePool ? (e) => { e.preventDefault(); handleDropOnSession(session.roundgroup_id, e.dataTransfer.getData('text/plain')) } : undefined}
                sx={{
                  bgcolor: SESSION_CARD.bg,
                  border: `2px solid ${isDropTarget ? SESSION_CARD.label : SESSION_CARD.border}`,
                  borderRadius: 2, p: 1.5,
                  transition: 'border-color 120ms',
                }}
              >
                {/* Session label + actions */}
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
                  <Typography variant="subtitle2" sx={{ flex: 1, fontWeight: 700, color: SESSION_CARD.label }}>
                    Session: {session.name}
                  </Typography>
                  {canEdit && (
                    <>
                      <Button size="small" startIcon={<AddIcon />} onClick={() => setRoundDialog({ open: true, sessionId: session.roundgroup_id, round: null })}>
                        Add Round
                      </Button>
                      <Button size="small" startIcon={<EditIcon />} onClick={() => setSessionDialog({ open: true, roundgroup: session })}>
                        Edit Session
                      </Button>
                      <Tooltip title={sessionRounds.length > 0 ? "This session can't be deleted while it has a round. Delete or move its rounds first." : ''}>
                        <span>
                          <Button size="small" color="error" startIcon={<DeleteIcon />} disabled={sessionRounds.length > 0} onClick={() => handleDeleteSession(session)}>
                            Delete Session
                          </Button>
                        </span>
                      </Tooltip>
                    </>
                  )}
                </Box>

                {/* Up arrow — move this session's first (top) round up into the previous session. */}
                {canMoveUp && (
                  <Box sx={{ display: 'flex', justifyContent: 'center', mb: 0.5 }}>
                    <Tooltip title={`Move “${sessionRounds[0].name}” up to ${prevSession.name}`}>
                      <IconButton size="small" onClick={() => moveRound(sessionRounds[0].roundid, prevSession.roundgroup_id)}>
                        <ArrowUpwardIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  </Box>
                )}

                {/* Round cards (concentric, own colour) */}
                {sessionRounds.length === 0 ? (
                  <Typography variant="caption" color="text.secondary">No rounds yet.</Typography>
                ) : (
                  <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                    {sessionRounds.map(round => {
                      const roundGames = isTimePool ? gamesForRound(round.roundid) : []
                      // Whether the round has any games at all (ignoring the pool-visibility filter). A
                      // round with a game can't be deleted, so the Delete button is disabled; it also
                      // gates the "No games in this round." message (only shown when truly empty).
                      const roundHasGames = (gamesByRound?.get(round.roundid)?.length ?? 0) > 0
                      const roundHasAnyGames = isTimePool && roundHasGames
                      return (
                        <Box
                          key={round.roundid}
                          draggable={canEdit && !isTimePool}
                          onDragStart={!isTimePool ? (e) => { e.dataTransfer.setData('text/plain', round.roundid); e.dataTransfer.effectAllowed = 'move'; setDraggingId(round.roundid) } : undefined}
                          onDragEnd={!isTimePool ? () => { setDraggingId(null); setDragOverSession(null) } : undefined}
                          sx={{
                            display: 'flex', flexDirection: 'column', gap: 0.5,
                            bgcolor: ROUND_CARD.bg,
                            border: `1px solid ${ROUND_CARD.border}`,
                            borderRadius: 1.5, px: 1, py: 0.75,
                            opacity: draggingId === round.roundid ? 0.5 : 1,
                            cursor: (canEdit && !isTimePool) ? 'grab' : 'default',
                          }}
                        >
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                            {canEdit && !isTimePool && <DragIndicatorIcon fontSize="small" sx={{ color: '#9ca3af' }} />}
                            <Box sx={{ flex: 1, minWidth: 0, display: 'flex', alignItems: 'baseline', gap: 1, flexWrap: 'wrap' }}>
                              <Typography variant="body2" sx={{ fontWeight: 600, color: ROUND_CARD.label }}>Round: {round.name}</Typography>
                              <Typography variant="caption" color="text.secondary">{formatStart(round.scheduled_start_time)}</Typography>
                            </Box>
                            {canEdit && (
                              <>
                                {isTimePool && (
                                  <Button size="small" startIcon={<AddIcon />} onClick={() => setGameDialog({ open: true, roundGroupId: session.roundgroup_id, roundId: round.roundid, game: null })}>
                                    Add Game
                                  </Button>
                                )}
                                <Button size="small" startIcon={<EditIcon />} onClick={() => setRoundDialog({ open: true, sessionId: session.roundgroup_id, round })}>
                                  Edit Round
                                </Button>
                                <Tooltip title={roundHasGames ? "This round can't be deleted while it has a game. Delete or move its games first." : ''}>
                                  <span>
                                    <Button size="small" color="error" startIcon={<DeleteIcon />} disabled={roundHasGames} onClick={() => handleDelete(round)}>
                                      Delete Round
                                    </Button>
                                  </span>
                                </Tooltip>
                              </>
                            )}
                          </Box>

                          {/* TimePool: the round's games, keeping the Pools matrix's matchups visible.
                              A round with games that are all hidden by the pool filter renders nothing
                              here; "No games in this round." only shows when the round is truly empty. */}
                          {isTimePool && (
                            !roundHasAnyGames ? (
                              <Typography variant="caption" color="text.secondary" sx={{ pl: 0.5 }}>No games in this round.</Typography>
                            ) : roundGames.length === 0 ? null : (
                              // Games as wrapping chips so they fill the row's whitespace and cut down on vertical scrolling.
                              <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, pl: 0.5 }}>
                                {roundGames.map(({ g, poolName, poolBracketId }) => (
                                  // No icons on the chip: clicking it opens the game editor, where the
                                  // user can edit, delete, or view the game's profile.
                                  <Chip
                                    key={g.gid}
                                    variant="outlined"
                                    onClick={() => setGameDialog({ open: true, roundGroupId: session.roundgroup_id, roundId: round.roundid, game: g, poolBracketId })}
                                    sx={{
                                      height: 'auto',
                                      borderRadius: '8px',
                                      bgcolor: 'rgba(0, 128, 128, 0.08)',
                                      borderColor: 'primary.main',
                                      color: 'primary.main',
                                      cursor: 'pointer',
                                      '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                                    }}
                                    label={
                                      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.25 }}>
                                        {/* Row 1: room */}
                                        <Typography component="span" variant="caption" sx={{ fontWeight: 600, color: 'primary.main', whiteSpace: 'nowrap', lineHeight: 1 }}>
                                          Room: {g.room_name}
                                        </Typography>
                                        {/* Row 2: teams */}
                                        <Typography component="span" variant="caption" sx={{ color: 'primary.main', lineHeight: 1.2 }}>
                                          {matchupText(g)}
                                        </Typography>
                                        {/* Row 3: pool */}
                                        {poolName && (
                                          <Typography component="span" variant="caption" sx={{ color: 'text.secondary', lineHeight: 1 }}>
                                            Pool: {poolName}
                                          </Typography>
                                        )}
                                      </Box>
                                    }
                                  />
                                ))}
                              </Box>
                            )
                          )}
                        </Box>
                      )
                    })}
                  </Box>
                )}

                {/* Down arrow — move this session's last (bottom) round down into the next session. */}
                {canMoveDown && (
                  <Box sx={{ display: 'flex', justifyContent: 'center', mt: 0.5 }}>
                    <Tooltip title={`Move “${sessionRounds[sessionRounds.length - 1].name}” down to ${nextSession.name}`}>
                      <IconButton size="small" onClick={() => moveRound(sessionRounds[sessionRounds.length - 1].roundid, nextSession.roundgroup_id)}>
                        <ArrowDownwardIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  </Box>
                )}
              </Box>
            )
          })}
        </Box>
      )}

      {/* Add a new session — a "+" button directly below the last session, left-aligned. */}
      {canEdit && (
        <Box sx={{ mt: 1.5 }}>
          <Tooltip title="Add session">
            <Button size="small" variant="outlined" sx={{ minWidth: 40, px: 1 }}
              onClick={() => setSessionDialog({ open: true, roundgroup: null })}>
              + Add a Session
            </Button>
          </Tooltip>
        </Box>
      )}

      <RoundEditorDialog
        tid={tid}
        lockedDivisionId={did}
        lockedRoundGroupId={roundDialog.sessionId || undefined}
        round={roundDialog.round}
        isOpen={roundDialog.open}
        onCancel={() => setRoundDialog({ open: false, sessionId: '', round: null })}
        onSave={() => { setRoundDialog({ open: false, sessionId: '', round: null }); loadRounds() }}
      />

      <RoundGroupEditorDialog
        tid={tid}
        lockedDivisionId={did}
        roundgroup={sessionDialog.roundgroup}
        isOpen={sessionDialog.open}
        onCancel={() => setSessionDialog({ open: false, roundgroup: null })}
        onSave={() => { setSessionDialog({ open: false, roundgroup: null }); onRoundGroupsChanged() }}
      />

      {isTimePool && (
        <GameEditorDialog
          tid={tid}
          lockedDivisionId={did}
          // Adding to a round locks the session/round; editing leaves them changeable.
          lockedRoundGroupId={gameDialog.game ? undefined : (gameDialog.roundGroupId || undefined)}
          lockedRoundId={gameDialog.game ? undefined : (gameDialog.roundId || undefined)}
          game={gameDialog.game}
          gamePoolBracketId={gameDialog.poolBracketId}
          isOpen={gameDialog.open}
          onCancel={closeGameDialog}
          onSave={() => { closeGameDialog(); onGamesChanged?.() }}
          // Editing an existing game: allow viewing its profile and deleting it from the dialog.
          onView={gameDialog.game && onNavigateGame ? () => onNavigateGame(gameDialog.game!.gid) : undefined}
          onDelete={gameDialog.game ? () => { const gid = gameDialog.game!.gid; closeGameDialog(); handleDeleteGame(gid) } : undefined}
        />
      )}

      {isTimePool && (
        <PoolBracketEditorDialog
          tid={tid}
          did={did}
          type={poolDialog.bracket?.type ?? 'pool'}
          entityLabel={poolDialog.bracket?.type === 'bracket' ? 'Bracket' : 'Pool'}
          bracket={poolDialog.bracket}
          isOpen={poolDialog.open}
          onCancel={() => setPoolDialog({ open: false, bracket: null })}
          onSave={() => { setPoolDialog({ open: false, bracket: null }); onPoolsChanged?.(); onGamesChanged?.() }}
        />
      )}

      <Snackbar
        open={!!notice}
        autoHideDuration={6000}
        onClose={() => setNotice(null)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
      >
        <Alert severity="warning" onClose={() => setNotice(null)} sx={{ maxWidth: 520 }}>
          {notice}
        </Alert>
      </Snackbar>
    </Box>
  )
}
