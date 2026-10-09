import { useEffect, useState } from 'react'
import Alert from '@mui/material/Alert'
import Badge from '@mui/material/Badge'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import Typography from '@mui/material/Typography'
import GppMaybeIcon from '@mui/icons-material/GppMaybe'
import ShieldOutlinedIcon from '@mui/icons-material/ShieldOutlined'
import ContentCopyIcon from '@mui/icons-material/ContentCopy'
import { RoomAPI, type ClearStatus, type QuarantinedGameEventTS } from '../features/RoomAPI'
import { useAuth } from '../hooks/useAuth'

// ─── Quarantine ───────────────────────────────────────────────────────────────

/**
 * The Server Monitor's Quarantine control: an empty shield when the room has nothing quarantined,
 * a filled warning shield with a count when it does.
 */
export function QuarantineButton({ count, onClick }: { count: number; onClick: () => void }) {
  const has = count > 0
  return (
    <Tooltip title={has ? `${count} quarantined event${count === 1 ? '' : 's'} — click to view` : 'No quarantined events'}>
      <IconButton
        size="small"
        onClick={onClick}
        aria-label={has ? `View ${count} quarantined events` : 'View quarantined events (none)'}
        // A white backing keeps it legible on the monitor's red alert rows.
        sx={{ bgcolor: '#fff', border: '1px solid', borderColor: has ? '#d97706' : '#d1d5db', '&:hover': { bgcolor: '#fff7ed' } }}
      >
        <Badge badgeContent={has ? count : 0} color="warning" max={99}>
          {has
            ? <GppMaybeIcon fontSize="small" sx={{ color: '#d97706' }} />
            : <ShieldOutlinedIcon fontSize="small" sx={{ color: '#9ca3af' }} />}
        </Badge>
      </IconButton>
    </Tooltip>
  )
}

const KIND_LABEL: Record<QuarantinedGameEventTS['kind'], string> = {
  rejected: 'Rejected',
  cleared: 'Set aside while clearing',
}

/** The room's quarantined events: what each was, and why it was not applied. */
export function QuarantineDialog({
  roomid,
  roomName,
  isOpen,
  onClose,
}: {
  roomid: string
  roomName: string
  isOpen: boolean
  onClose: () => void
}) {
  const { accessToken } = useAuth()
  const [rows, setRows] = useState<QuarantinedGameEventTS[]>([])
  // Mounted per opening (see RoomMonitorTable), so each opening starts out loading.
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!isOpen) return
    let cancelled = false
    RoomAPI.getQuarantine(roomid, accessToken)
      .then((r) => { if (!cancelled) setRows(r) })
      .catch((e: Error) => { if (!cancelled) setError(e.message) })
      .finally(() => { if (!cancelled) setLoading(false) })
    return () => { cancelled = true }
  }, [isOpen, roomid, accessToken])

  const th = { textAlign: 'left' as const, padding: '6px 8px', fontSize: 12, color: '#6b7280', borderBottom: '1px solid #e5e7eb', whiteSpace: 'nowrap' as const }
  const td = { padding: '6px 8px', fontSize: 13, borderBottom: '1px solid #f3f4f6', verticalAlign: 'top' as const }

  return (
    <Dialog open={isOpen} onClose={onClose} maxWidth="lg" fullWidth>
      <DialogTitle>Quarantined events — {roomName}</DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          QView told this room's QuizMachine that each of these events arrived, so it could keep sending,
          but did not apply them. <strong>Rejected</strong> events failed a check they would fail on every
          retry. <strong>Set aside while clearing</strong> events were held during a Clear Pending + Resend
          and are normally recovered when the room resends its games. Nothing here has been deleted.
        </Typography>

        {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
        {loading && <Typography variant="body2">Loading…</Typography>}
        {!loading && !error && rows.length === 0 && (
          <Typography variant="body2" color="text.secondary">This room has no quarantined events.</Typography>
        )}

        {rows.length > 0 && (
          <div style={{ overflowX: 'auto' }}>
            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
              <thead>
                <tr>
                  {['When', 'Kind', 'Reason', 'Round', 'Q.Event', 'Code', 'Name', 'Team/Seat', 'Game', ''].map((h, i) => (
                    <th key={i} style={th}>{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {rows.map((r) => (
                  <tr key={r.quarantine_id}>
                    <td style={{ ...td, whiteSpace: 'nowrap' }}>{new Date(r.created_at).toLocaleString()}</td>
                    <td style={td}>
                      <Chip
                        size="small"
                        label={KIND_LABEL[r.kind]}
                        color={r.kind === 'rejected' ? 'error' : 'default'}
                        variant={r.kind === 'rejected' ? 'filled' : 'outlined'}
                      />
                    </td>
                    <td style={{ ...td, minWidth: 260 }}>{r.reason}</td>
                    <td style={td}>{r.round || '—'}</td>
                    <td style={td}>{r.question}.{r.eventnum}</td>
                    <td style={td}>{r.event || '—'}</td>
                    <td style={td}>{r.name || '—'}</td>
                    <td style={td}>{r.team}/{r.quizzer}</td>
                    <td style={{ ...td, fontFamily: 'monospace', fontSize: 11 }} title={r.gid_raw}>
                      {r.gid ? r.gid.slice(0, 8) : (r.gid_raw || '—')}
                    </td>
                    <td style={td}>
                      {/* The exact request, for inspection or replay. */}
                      <Tooltip title="Copy the request exactly as the client sent it">
                        <IconButton size="small" aria-label="Copy request" onClick={() => navigator.clipboard?.writeText(r.query_string)}>
                          <ContentCopyIcon fontSize="inherit" />
                        </IconButton>
                      </Tooltip>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose} variant="contained" disableElevation>Close</Button>
      </DialogActions>
    </Dialog>
  )
}

// ─── Clear Pending + Resend ───────────────────────────────────────────────────

const STATUS_LABEL: Record<ClearStatus, string> = {
  clearing: 'Clearing…',
  resending: 'Resending…',
  complete: 'Recovered',
  unconfirmed: 'Resend not confirmed',
  cancelled: 'Cancelled',
  expired: 'Expired',
}

const STATUS_HELP: Record<ClearStatus, string> = {
  clearing: 'Acknowledging this room’s events without applying them, until it reports nothing pending.',
  resending: 'Asking the room to resend the affected games, until every set-aside event is back.',
  complete: 'Every set-aside event came back and was stored.',
  unconfirmed: 'The room did not resend everything within 10 minutes. The events are still in Quarantine.',
  cancelled: 'Stopped before anything was set aside.',
  expired: 'The 5-minute clearing window ended with nothing set aside.',
}

/** Where a clear stands, on the Server Monitor's Pending cell. */
export function ClearStatusChip({ status, setAside }: { status: ClearStatus; setAside: number }) {
  const color = status === 'complete' ? 'success' : status === 'unconfirmed' ? 'error' : status === 'clearing' || status === 'resending' ? 'warning' : 'default'
  const label = status === 'clearing' && setAside > 0 ? `${STATUS_LABEL[status]} (${setAside} set aside)` : STATUS_LABEL[status]
  return (
    <Tooltip title={STATUS_HELP[status]}>
      <Chip size="small" label={label} color={color} sx={{ bgcolor: color === 'default' ? '#fff' : undefined }} />
    </Tooltip>
  )
}

/** The red "this room looks stuck" button on the Server Monitor's Pending cell. */
export function ClearPendingButton({ onClick, disabled }: { onClick: () => void; disabled?: boolean }) {
  return (
    <Button
      size="small"
      variant="contained"
      color="error"
      disableElevation
      onClick={onClick}
      disabled={disabled}
      // The white border keeps it distinct on the monitor's red alert rows.
      sx={{ whiteSpace: 'nowrap', border: '1px solid #fff', textTransform: 'none', fontWeight: 600 }}
    >
      Clear Pending + Resend?
    </Button>
  )
}

/** Explains what Clear Pending + Resend does before the operator commits to it. */
export function ClearPendingDialog({
  roomName,
  pending,
  isOpen,
  busy,
  error,
  onCancel,
  onConfirm,
}: {
  roomName: string
  pending: number | null
  isOpen: boolean
  busy: boolean
  error: string | null
  onCancel: () => void
  onConfirm: () => void
}) {
  return (
    <Dialog open={isOpen} onClose={busy ? undefined : onCancel} maxWidth="sm" fullWidth>
      <DialogTitle>Clear Pending + Resend — {roomName}</DialogTitle>
      <DialogContent>
        <Typography variant="body2" sx={{ mb: 1.5 }}>
          This room’s QuizMachine has <strong>{pending ?? 'some'}</strong> game event{pending === 1 ? '' : 's'} waiting
          to send, and that number has not gone down for at least 90 seconds. That usually means the server is
          refusing one event every time it is retried, and every event queued behind it is stuck.
        </Typography>

        <Typography variant="subtitle2" sx={{ fontWeight: 700, mt: 2, mb: 0.5 }}>If you proceed</Typography>
        <ol style={{ margin: 0, paddingLeft: 20 }}>
          <li>
            <Typography variant="body2">
              For up to <strong>5 minutes</strong>, QView tells the room each event arrived <em>without applying it</em>,
              so its queue empties. Every one of those events is kept in this room’s <strong>Quarantine</strong>.
              Nothing is deleted.
            </Typography>
          </li>
          <li>
            <Typography variant="body2">
              Once the room reports nothing pending, QView asks it to <strong>resend in full</strong> every game it held
              events for, plus the game it is on now — repeating once a minute for up to 10 minutes until all of
              those events have come back.
            </Typography>
          </li>
          <li>
            <Typography variant="body2">
              Until the resend arrives, this room’s scores in Games in Progress and Server Stats may be behind.
            </Typography>
          </li>
        </ol>

        <Typography variant="subtitle2" sx={{ fontWeight: 700, mt: 2, mb: 0.5 }}>Good to know</Typography>
        <ul style={{ margin: 0, paddingLeft: 20 }}>
          <li>
            <Typography variant="body2">
              The room must keep checking in. If it goes offline, clearing stops after 5 minutes; anything already
              held is asked for again when it next checks in.
            </Typography>
          </li>
          <li>
            <Typography variant="body2">
              An event that is genuinely bad — for example, one for a game that no longer exists — lands in
              Quarantine with the reason instead of blocking the room again.
            </Typography>
          </li>
          <li>
            <Typography variant="body2">
              You can cancel while it is still clearing. Anything already held is still asked for.
            </Typography>
          </li>
        </ul>

        {error && <Alert severity="error" sx={{ mt: 2 }}>{error}</Alert>}
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel} disabled={busy}>Cancel</Button>
        <Button onClick={onConfirm} variant="contained" color="error" disableElevation disabled={busy}>
          {busy ? 'Starting…' : 'Clear Pending + Resend'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
