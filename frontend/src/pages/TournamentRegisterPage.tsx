import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import Typography from '@mui/material/Typography'
import { useAuth } from '../hooks/useAuth'
import { TournamentGearRegistrationPanel } from '../components/TournamentGearRegistrationPanel'
import { TournamentTeamRegistrationPanel } from '../components/TournamentTeamRegistrationPanel'

type Tab = 'team' | 'gear'

export const TournamentRegisterPage = ({ tid, tname, initialTab = 'team' }: { tid: string; tname: string; initialTab?: 'team' | 'gear' | 'as-volunteer' }) => {
  const { session } = useAuth()
  const navigate = useNavigate()
  const [activeTab] = useState<Tab>(initialTab === 'gear' ? 'gear' : 'team')

  // Volunteer
  const [volunteerDialogOpen, setVolunteerDialogOpen] = useState(initialTab === 'as-volunteer')
  const [volunteerName, setVolunteerName] = useState('')

  useEffect(() => {
    if (!session?.userId) return
    fetch(`/api/users/${session.userId}`)
      .then(r => r.ok ? r.json() : null)
      .then(data => {
        if (data) setVolunteerName([data.fname, data.mname, data.lname].filter(Boolean).join(' '))
      })
      .catch(() => {})
  }, [session?.userId])

  return (
    <Box>

      {/* ── Tab Buttons ── */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1, mb: 3 }}>
        <Button
          variant={activeTab === 'team' ? 'contained' : 'outlined'}
          onClick={() => navigate(`/tournament/${tid}/register/team`)}
        >
          Team
        </Button>
        <Button
          variant={activeTab === 'gear' ? 'contained' : 'outlined'}
          onClick={() => navigate(`/tournament/${tid}/register/gear`)}
        >
          Gear
        </Button>
        <Button
          variant="outlined"
          disabled={!session}
          onClick={() => navigate(`/tournament/${tid}/register/as-volunteer`)}
        >
          As Volunteer
        </Button>
        {!session && (
          <Typography variant="caption" color="text.secondary" sx={{ mt: 1 }}>
            Sign in or create an account to register for this Tournament as a volunteer.
          </Typography>
        )}
      </Box>

      {/* ── Team Tab ── */}
      {activeTab === 'team' && (
        <TournamentTeamRegistrationPanel tid={tid} />
      )}

      {/* ── Gear Tab ── */}
      {activeTab === 'gear' && (
        <TournamentGearRegistrationPanel tid={tid} />
      )}

      {/* ── Volunteer Confirm Dialog ── */}
      <Dialog open={volunteerDialogOpen} onClose={() => setVolunteerDialogOpen(false)}>
        <DialogTitle>Register as Volunteer</DialogTitle>
        <DialogContent>
          <DialogContentText>
            You are registering as a volunteer for <strong>{tname}</strong> under the name <strong>{volunteerName}</strong>. Would you like to confirm?
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setVolunteerDialogOpen(false)}>Cancel</Button>
          <Button variant="contained" onClick={() => setVolunteerDialogOpen(false)}>Confirm</Button>
        </DialogActions>
      </Dialog>

    </Box>
  )
}
