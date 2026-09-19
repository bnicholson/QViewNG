import { useState, useEffect, useCallback, useMemo } from 'react'
import Box from '@mui/material/Box'
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
  // Pool/bracket edit dialog (TimePool): opened from the chips above the sessions.
  const [poolDialog, setPoolDialog] = useState<{ open: boolean; bracket: PoolBracketTS | null }>({ open: false, bracket: null })

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

  // Games for a round, sorted by room name so the list reads in the same room order as the Pools matrix.
  const gamesForRound = (roundId: string): RoundGameEntry[] =>
    [...(gamesByRound?.get(roundId) ?? [])].sort((a, b) => a.g.room_name.localeCompare(b.g.room_name, undefined, { numeric: true }))

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

      {/* Pools/brackets for the division — editable chips above the first session. */}
      {isTimePool && poolBrackets && poolBrackets.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
          {poolBrackets.map(pb => (
            <Chip
              key={pb.pool_bracket_id}
              variant="outlined"
              sx={{ height: 'auto', borderRadius: '8px', '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 } }}
              label={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1 }}>
                    {pb.name}
                  </Typography>
                  {canEdit && (
                    <Tooltip title={`Edit ${pb.type === 'bracket' ? 'bracket' : 'pool'}`}>
                      <IconButton size="small" sx={{ p: 0.25 }} onClick={() => setPoolDialog({ open: true, bracket: pb })}>
                        <EditIcon sx={{ fontSize: 15 }} />
                      </IconButton>
                    </Tooltip>
                  )}
                </Box>
              }
            />
          ))}
        </Box>
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
                      <Button size="small" color="error" startIcon={<DeleteIcon />} onClick={() => handleDeleteSession(session)}>
                        Delete Session
                      </Button>
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
                                <Button size="small" color="error" startIcon={<DeleteIcon />} onClick={() => handleDelete(round)}>
                                  Delete Round
                                </Button>
                              </>
                            )}
                          </Box>

                          {/* TimePool: the round's games, keeping the Pools matrix's matchups visible. */}
                          {isTimePool && (
                            roundGames.length === 0 ? (
                              <Typography variant="caption" color="text.secondary" sx={{ pl: 0.5 }}>No games in this round.</Typography>
                            ) : (
                              // Games as wrapping chips so they fill the row's whitespace and cut down on vertical scrolling.
                              <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, pl: 0.5 }}>
                                {roundGames.map(({ g, poolName, poolBracketId }) => (
                                  <Chip
                                    key={g.gid}
                                    variant="outlined"
                                    onClick={onNavigateGame ? () => onNavigateGame(g.gid) : undefined}
                                    sx={{
                                      height: 'auto',
                                      borderRadius: '8px',
                                      bgcolor: 'rgba(0, 128, 128, 0.08)',
                                      borderColor: 'primary.main',
                                      color: 'primary.main',
                                      cursor: onNavigateGame ? 'pointer' : 'default',
                                      '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                                    }}
                                    label={
                                      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.25 }}>
                                        {/* Row 1: room name + edit/delete */}
                                        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                                          <Typography component="span" variant="caption" sx={{ fontWeight: 600, color: 'primary.main', whiteSpace: 'nowrap', lineHeight: 1 }}>
                                            Room: {g.room_name}
                                          </Typography>
                                          {canEdit && (
                                            <>
                                              <Tooltip title="Edit game">
                                                <IconButton size="small" sx={{ p: 0.25, color: 'primary.main' }} onClick={(e) => { e.stopPropagation(); setGameDialog({ open: true, roundGroupId: session.roundgroup_id, roundId: round.roundid, game: g, poolBracketId }) }}>
                                                  <EditIcon sx={{ fontSize: 15 }} />
                                                </IconButton>
                                              </Tooltip>
                                              <Tooltip title="Delete game">
                                                <IconButton size="small" color="error" sx={{ p: 0.25 }} onClick={(e) => { e.stopPropagation(); handleDeleteGame(g.gid) }}>
                                                  <DeleteIcon sx={{ fontSize: 15 }} />
                                                </IconButton>
                                              </Tooltip>
                                            </>
                                          )}
                                        </Box>
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
