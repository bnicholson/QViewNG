import { useState, useEffect } from 'react'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import DialogTitle from '@mui/material/DialogTitle'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { ConfirmDialog, confirmDialogDefaultState } from '../components/ConfirmDialog'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { TeamAPI, type NewTeamPayload } from '../features/TeamAPI'
import { useAuth } from '../hooks/useAuth'

type Tab = 'team' | 'gear'

interface TeamFormState {
  name: string
  did: string
  coachid: string
}

const emptyTeamForm: TeamFormState = { name: '', did: '', coachid: '' }

export const TournamentRegisterPage = ({ tid, tname, initialTab = 'team' }: { tid: string; tname: string; initialTab?: 'team' | 'gear' | 'as-volunteer' }) => {
  const { session } = useAuth()
  const [activeTab, setActiveTab] = useState<Tab>(initialTab === 'gear' ? 'gear' : 'team')

  // Team form
  const [form, setForm] = useState<TeamFormState>(emptyTeamForm)
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  const [users, setUsers] = useState<UserTS[]>([])
  const [alertOpened, setAlertOpened] = useState(false)
  const [errorMsg, setErrorMsg] = useState('')
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState)
  const [saveSuccess, setSaveSuccess] = useState(false)

  // Volunteer
  const [volunteerDialogOpen, setVolunteerDialogOpen] = useState(initialTab === 'as-volunteer')
  const [volunteerName, setVolunteerName] = useState('')

  useEffect(() => {
    Promise.all([
      DivisionAPI.getByTournament(tid, 0, 100),
      UserAPI.get(0, 200),
    ])
      .then(([divs, userResult]) => {
        setDivisions(divs)
        setUsers(userResult.items)
      })
      .catch(() => console.error('Failed to load registration form data'))
  }, [tid])

  useEffect(() => {
    if (!session?.userId) return
    fetch(`/api/users/${session.userId}`)
      .then(r => r.ok ? r.json() : null)
      .then(data => {
        if (data) setVolunteerName([data.fname, data.mname, data.lname].filter(Boolean).join(' '))
      })
      .catch(() => {})
  }, [session?.userId])

  const userLabel = (u: UserTS) =>
    [u.fname, u.mname, u.lname].filter(Boolean).join(' ') + ` (${u.username})`

  const handleTeamSave = async () => {
    if (!form.name.trim()) { setErrorMsg('Team name is required.'); setAlertOpened(true); return }
    if (!form.did) { setErrorMsg('Division is required.'); setAlertOpened(true); return }
    if (!form.coachid) { setErrorMsg('Coach is required.'); setAlertOpened(true); return }

    const payload: NewTeamPayload = { name: form.name, did: form.did, coachid: form.coachid }
    try {
      await TeamAPI.create(payload)
      setForm(emptyTeamForm)
      setAlertOpened(false)
      setSaveSuccess(true)
    } catch (err: any) {
      setErrorMsg('Failed to register team: ' + err.message)
      setAlertOpened(true)
    }
  }

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    message: 'Cancel if you want to make more changes.',
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleTeamSave() },
    title: 'Register this team?',
  })

  return (
    <Box>

      {/* ── Tab Buttons ── */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1, mb: 3 }}>
        <Typography variant="body1" sx={{ fontWeight: 600, alignSelf: 'center' }}>Register:</Typography>
        <Button
          variant={activeTab === 'team' ? 'contained' : 'outlined'}
          onClick={() => setActiveTab('team')}
        >
          Team
        </Button>
        <Button
          variant={activeTab === 'gear' ? 'contained' : 'outlined'}
          onClick={() => setActiveTab('gear')}
        >
          Gear
        </Button>
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start' }}>
          <Button
            variant="outlined"
            disabled={!session}
            onClick={() => setVolunteerDialogOpen(true)}
          >
            As Volunteer
          </Button>
          {!session && (
            <Typography variant="caption" color="text.secondary" sx={{ mt: 0.5, maxWidth: 240 }}>
              Sign in or create an account to register for this Tournament as a volunteer.
            </Typography>
          )}
        </Box>
      </Box>

      {/* ── Team Tab ── */}
      {activeTab === 'team' && (
        <Box>
          <Typography variant="h6" sx={{ mb: 2 }}>Register Team</Typography>

          <Collapse in={saveSuccess}>
            <Alert severity="success" sx={{ mb: 2 }} onClose={() => setSaveSuccess(false)}>
              Team registered successfully!
            </Alert>
          </Collapse>

          <Collapse in={alertOpened}>
            <Alert
              severity="error"
              action={
                <IconButton aria-label="close" color="inherit" size="small" onClick={() => setAlertOpened(false)}>
                  <CloseIcon fontSize="inherit" />
                </IconButton>
              }
              sx={{ mb: 2 }}
            >
              <AlertTitle>Error</AlertTitle>
              {errorMsg}
            </Alert>
          </Collapse>

          <List>
            <ListItem>
              <Grid container spacing={2}>
                <Grid item xs={12} sm={6}>
                  <InputLabel>Team Name (*required)</InputLabel>
                  <TextField
                    variant="outlined"
                    placeholder="Team Name"
                    value={form.name}
                    fullWidth
                    onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <InputLabel>Division (*required)</InputLabel>
                  <Select
                    value={form.did}
                    onChange={(e) => setForm(s => ({ ...s, did: e.target.value }))}
                    displayEmpty
                    fullWidth
                    renderValue={(val) => {
                      if (!val) return <em>Select a division</em>
                      return divisions.find(d => d.did === val)?.dname ?? val
                    }}
                  >
                    {divisions.map(d => (
                      <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>
                    ))}
                  </Select>
                </Grid>
              </Grid>
            </ListItem>

            <ListItem>
              <Grid container spacing={2}>
                <Grid item xs={12} sm={6}>
                  <InputLabel>Coach (*required)</InputLabel>
                  <Select
                    value={form.coachid}
                    onChange={(e) => setForm(s => ({ ...s, coachid: e.target.value }))}
                    displayEmpty
                    fullWidth
                    renderValue={(val) => {
                      if (!val) return <em>Select a coach</em>
                      const u = users.find(u => u.id === val)
                      return u ? userLabel(u) : val
                    }}
                  >
                    {users.map(u => (
                      <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>
                    ))}
                  </Select>
                </Grid>
              </Grid>
            </ListItem>

            <ListItem>
              <Button variant="contained" onClick={openSaveDialog}>Register Team</Button>
            </ListItem>
          </List>
        </Box>
      )}

      {/* ── Gear Tab ── */}
      {activeTab === 'gear' && (
        <Typography>#</Typography>
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

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
        title={confirmDialog.title}
      />

    </Box>
  )
}
