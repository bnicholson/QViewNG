import { useState, useEffect, useCallback, useMemo } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import Alert from '@mui/material/Alert'
import Snackbar from '@mui/material/Snackbar'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import DragIndicatorIcon from '@mui/icons-material/DragIndicator'
import dayjs from 'dayjs'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { type RoundGroupTS } from '../features/RoundGroupAPI'
import { RoundEditorDialog } from './RoundEditorDialog'
import { RoundGroupEditorDialog } from './RoundGroupEditorDialog'
import { useAuth } from '../hooks/useAuth'

const PAGE = 0
const SIZE = 500

// Each card "type" gets its own colour so the concentric nesting reads at a glance.
const SESSION_CARD = { bg: '#eef2ff', border: '#c7d2fe', label: '#3730a3' }   // indigo
const ROUND_CARD = { bg: '#ecfdf5', border: '#a7f3d0', label: '#065f46' }     // emerald

function formatStart(iso: string | null): string {
  return iso ? dayjs(iso).format('MMM D, YYYY · h:mm A') : 'Unscheduled'
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
}

export default function SessionRoundsManager({ tid, did, roundgroups, canEdit, onRoundGroupsChanged }: Props) {
  const { accessToken } = useAuth()
  const [rounds, setRounds] = useState<RoundTS[]>([])
  const [notice, setNotice] = useState<string | null>(null)
  const [draggingId, setDraggingId] = useState<string | null>(null)
  const [dragOverSession, setDragOverSession] = useState<string | null>(null)

  // Round create/edit dialog: `session` = the session to create in; `round` = the round being edited.
  const [roundDialog, setRoundDialog] = useState<{ open: boolean; sessionId: string; round: RoundTS | null }>({ open: false, sessionId: '', round: null })
  // Session create/edit dialog.
  const [sessionDialog, setSessionDialog] = useState<{ open: boolean; roundgroup: RoundGroupTS | null }>({ open: false, roundgroup: null })

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

  // Drop a round onto a session → move it there (persists immediately).
  const handleDropOnSession = async (targetSessionId: string, roundId: string) => {
    setDragOverSession(null)
    setDraggingId(null)
    const round = rounds.find(r => r.roundid === roundId)
    if (!round || round.roundgroup_id === targetSessionId) return
    try {
      await RoundAPI.update(roundId, { roundgroup_id: targetSessionId }, accessToken)
      loadRounds()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to move round.')
    }
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

  return (
    <Box sx={{ textAlign: 'left' }}>
      <Box sx={{ mb: 1 }}>
        <Typography variant="body2" color="text.secondary">
          This tab lets you manage time by modifying your Rounds and Sessions. Use Sessions to organize your Rounds into sequential groups. (There are usually longer-than-normal breaks of no quizzing in between each Session.)
        </Typography>
      </Box>
      <Box sx={{ mb: 1 }}>
        <Typography variant="body2" color="text.secondary">
          Drag a round from one session onto another to move it. Rounds with a game can't be deleted.
        </Typography>
      </Box>

      {sortedSessions.length === 0 ? (
        <Typography variant="body2" color="text.secondary">This division has no sessions yet.</Typography>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          {sortedSessions.map(session => {
            const sessionRounds = roundsBySession.get(session.roundgroup_id) ?? []
            const isDropTarget = dragOverSession === session.roundgroup_id && draggingId != null
            return (
              <Box
                key={session.roundgroup_id}
                onDragOver={(e) => { e.preventDefault(); setDragOverSession(session.roundgroup_id) }}
                onDragLeave={() => setDragOverSession(prev => prev === session.roundgroup_id ? null : prev)}
                onDrop={(e) => { e.preventDefault(); handleDropOnSession(session.roundgroup_id, e.dataTransfer.getData('text/plain')) }}
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
                    {session.name}
                  </Typography>
                  {canEdit && (
                    <>
                      <Button size="small" startIcon={<EditIcon />} onClick={() => setSessionDialog({ open: true, roundgroup: session })}>
                        Edit Session
                      </Button>
                      <Button size="small" startIcon={<AddIcon />} onClick={() => setRoundDialog({ open: true, sessionId: session.roundgroup_id, round: null })}>
                        Add Round
                      </Button>
                    </>
                  )}
                </Box>

                {/* Round cards (concentric, own colour) */}
                {sessionRounds.length === 0 ? (
                  <Typography variant="caption" color="text.secondary">No rounds yet.</Typography>
                ) : (
                  <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                    {sessionRounds.map(round => (
                      <Box
                        key={round.roundid}
                        draggable={canEdit}
                        onDragStart={(e) => { e.dataTransfer.setData('text/plain', round.roundid); e.dataTransfer.effectAllowed = 'move'; setDraggingId(round.roundid) }}
                        onDragEnd={() => { setDraggingId(null); setDragOverSession(null) }}
                        sx={{
                          display: 'flex', alignItems: 'center', gap: 0.5,
                          bgcolor: ROUND_CARD.bg,
                          border: `1px solid ${ROUND_CARD.border}`,
                          borderRadius: 1.5, px: 1, py: 0.75,
                          opacity: draggingId === round.roundid ? 0.5 : 1,
                          cursor: canEdit ? 'grab' : 'default',
                        }}
                      >
                        {canEdit && <DragIndicatorIcon fontSize="small" sx={{ color: '#9ca3af' }} />}
                        <Box sx={{ flex: 1, minWidth: 0 }}>
                          <Typography variant="body2" sx={{ fontWeight: 600, color: ROUND_CARD.label }}>{round.name}</Typography>
                          <Typography variant="caption" color="text.secondary">{formatStart(round.scheduled_start_time)}</Typography>
                        </Box>
                        {canEdit && (
                          <>
                            <Tooltip title="Edit round">
                              <IconButton size="small" onClick={() => setRoundDialog({ open: true, sessionId: session.roundgroup_id, round })}>
                                <EditIcon fontSize="inherit" />
                              </IconButton>
                            </Tooltip>
                            <Tooltip title="Delete round">
                              <IconButton size="small" onClick={() => handleDelete(round)}>
                                <DeleteIcon fontSize="inherit" />
                              </IconButton>
                            </Tooltip>
                          </>
                        )}
                      </Box>
                    ))}
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
