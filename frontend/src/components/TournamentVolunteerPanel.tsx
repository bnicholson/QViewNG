import { useState, useEffect } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import Divider from '@mui/material/Divider'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import CheckCircleIcon from '@mui/icons-material/CheckCircle'
import PersonIcon from '@mui/icons-material/Person'
import { useAuth } from '../hooks/useAuth'
import { AdminAPI } from '../features/AdminAPI'

interface Props {
  tid: string
  tname: string
}

export const TournamentVolunteerPanel = ({ tid, tname }: Props) => {
  const { session, isCheckingAuth } = useAuth()
  const userId = session?.userId

  const [volunteerName, setVolunteerName] = useState('')
  const [loadingName, setLoadingName] = useState(false)
  const [registered, setRegistered] = useState(false)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [confirmDialog, setConfirmDialog] = useState<'register' | 'remove' | null>(null)

  useEffect(() => {
    if (!userId) return
    setLoadingName(true)
    fetch(`/api/users/${userId}`)
      .then(r => r.ok ? r.json() : null)
      .then(data => {
        if (data) setVolunteerName([data.fname, data.mname, data.lname].filter(Boolean).join(' ') || data.username)
      })
      .catch(() => {})
      .finally(() => setLoadingName(false))
  }, [userId])

  useEffect(() => {
    if (!userId) return
    AdminAPI.getByTournament(tid, 0, 100)
      .then(admins => setRegistered(admins.some(a => a.id === userId)))
      .catch(() => {})
  }, [tid, userId])

  const handleRegister = async () => {
    setConfirmDialog(null)
    setSaving(true)
    setError(null)
    try {
      await AdminAPI.create(tid, { tournamentid: tid, adminid: userId!, role_description: 'Volunteer', access_lvl: 0 })
      setRegistered(true)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  const handleRemove = async () => {
    setConfirmDialog(null)
    setSaving(true)
    setError(null)
    try {
      await AdminAPI.delete(tid, userId!)
      setRegistered(false)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  // ── Auth check in progress ───────────────────────────────────────────────────

  if (isCheckingAuth) {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 3 }}>
        <CircularProgress size={20} />
        <Typography color="text.secondary">Checking authentication…</Typography>
      </Box>
    )
  }

  // ── Not logged in ────────────────────────────────────────────────────────────

  if (!userId) {
    return (
      <Alert severity="info">
        Please <strong>sign in</strong> or <strong>create an account</strong> to register as a
        volunteer for this tournament.
      </Alert>
    )
  }

  return (
    <Box sx={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
      )}

      <Paper variant="outlined" sx={{ p: 2.5, borderRadius: 2, maxWidth: 480, textAlign: "center" }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, mb: 2 }}>
          <PersonIcon color="action" />
          <Box>
            <Typography variant="caption" color="text.secondary">Registering as</Typography>
            {loadingName ? (
              <CircularProgress size={12} sx={{ display: 'block', mt: 0.5 }} />
            ) : (
              <Typography variant="body1" fontWeight={500}>{volunteerName || '—'}</Typography>
            )}
          </Box>
        </Box>

        <Divider sx={{ mb: 2 }} />

        <Typography variant="body2" color="text.secondary" sx={{ mb: 2.5 }}>
          You are registering as a volunteer for <strong>{tname}</strong>. Volunteers assist with
          running the tournament.
        </Typography>

        {registered ? (
          <Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
              <CheckCircleIcon color="success" fontSize="small" />
              <Typography variant="body2" color="success.main" fontWeight={500}>
                Registered as volunteer
              </Typography>
            </Box>
            <Button
              variant="outlined"
              color="error"
              disabled={saving}
              startIcon={saving ? <CircularProgress size={14} /> : undefined}
              onClick={() => setConfirmDialog('remove')}
            >
              Remove Registration
            </Button>
          </Box>
        ) : (
          <Button
            variant="contained"
            disabled={saving || loadingName}
            startIcon={saving ? <CircularProgress size={14} /> : undefined}
            onClick={() => setConfirmDialog('register')}
          >
            Register as Volunteer
          </Button>
        )}
      </Paper>

      {/* ── Register confirmation ── */}
      <Dialog open={confirmDialog === 'register'} onClose={() => setConfirmDialog(null)}>
        <DialogTitle>Register as Volunteer?</DialogTitle>
        <DialogContent>
          <DialogContentText>
            You will be registered as a volunteer for <strong>{tname}</strong>.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfirmDialog(null)}>Cancel</Button>
          <Button variant="contained" onClick={handleRegister}>Confirm</Button>
        </DialogActions>
      </Dialog>

      {/* ── Remove confirmation ── */}
      <Dialog open={confirmDialog === 'remove'} onClose={() => setConfirmDialog(null)}>
        <DialogTitle>Remove Volunteer Registration?</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Are you sure you want to remove your volunteer registration for <strong>{tname}</strong>?
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfirmDialog(null)}>Cancel</Button>
          <Button variant="outlined" color="error" onClick={handleRemove}>Remove</Button>
        </DialogActions>
      </Dialog>
    </Box>
  )
}
