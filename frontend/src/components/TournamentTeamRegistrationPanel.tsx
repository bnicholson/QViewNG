import { useState, useEffect, useCallback } from 'react'
import { useNavigate } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import MenuItem from '@mui/material/MenuItem'
import Paper from '@mui/material/Paper'
import Select from '@mui/material/Select'
import Stack from '@mui/material/Stack'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import TextField from '@mui/material/TextField'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import Typography from '@mui/material/Typography'
import AddIcon from '@mui/icons-material/Add'
import DeleteIcon from '@mui/icons-material/Delete'
import EditIcon from '@mui/icons-material/Edit'
import GroupsIcon from '@mui/icons-material/Groups'
import { TeamAPI, type TeamTS, type TeamChangeset } from '../features/TeamAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { RosterAPI, type RosterTS } from '../features/RosterAPI'
import { type UserTS } from '../features/UserAPI'
import { useAuth } from '../hooks/useAuth'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'

// ── Constants ─────────────────────────────────────────────────────────────────

const MAX_QUIZZERS = 6

// ── Helpers ───────────────────────────────────────────────────────────────────

interface TeamFormState {
  name: string
  did: string
  quizzers: (string | null)[]
}

const emptyForm = (): TeamFormState => ({
  name: '',
  did: '',
  quizzers: Array<string | null>(MAX_QUIZZERS).fill(null),
})

function teamToForm(team: TeamTS): TeamFormState {
  return {
    name: team.name,
    did: team.did,
    quizzers: [
      team.quizzer_one_id,
      team.quizzer_two_id,
      team.quizzer_three_id,
      team.quizzer_four_id,
      team.quizzer_five_id,
      team.quizzer_six_id,
    ],
  }
}

function formToChangeset(form: TeamFormState): TeamChangeset {
  return {
    name: form.name,
    did: form.did,
    quizzer_one_id: form.quizzers[0],
    quizzer_two_id: form.quizzers[1],
    quizzer_three_id: form.quizzers[2],
    quizzer_four_id: form.quizzers[3],
    quizzer_five_id: form.quizzers[4],
    quizzer_six_id: form.quizzers[5],
  }
}

function userLabel(u: UserTS): string {
  const name = [u.fname, u.mname, u.lname].filter(Boolean).join(' ')
  return name || u.username
}

// ── Component ─────────────────────────────────────────────────────────────────

interface Props {
  tid: string
}

export const TournamentTeamRegistrationPanel = ({ tid }: Props) => {
  const { session, accessToken, isCheckingAuth } = useAuth()
  const navigate = useNavigate()
  const userId = session?.userId

  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const [myTeams, setMyTeams] = useState<TeamTS[]>([])
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  const [rosters, setRosters] = useState<RosterTS[]>([])
  const [quizzersByRoster, setQuizzersByRoster] = useState<Record<string, UserTS[]>>({})

  // Form state
  const [editingTeamId, setEditingTeamId] = useState<string | null>(null)
  const [formOpen, setFormOpen] = useState(false)
  const [form, setForm] = useState<TeamFormState>(emptyForm())
  const [saving, setSaving] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)

  // Quizzer picker filter
  const [selectedRosterId, setSelectedRosterId] = useState<string>('all')

  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState)

  // ── Data loading ────────────────────────────────────────────────────────────

  const load = useCallback(async () => {
    if (!userId) { setLoading(false); return }
    setLoading(true)
    setError(null)
    try {
      const [divs, teamsResult, rosterList] = await Promise.all([
        DivisionAPI.getByTournament(tid, 0, 100),
        TeamAPI.getByTournament(tid, 0, 100),
        RosterAPI.getByCoach(userId),
      ])
      setDivisions(divs)
      setMyTeams(teamsResult.items.filter(t => t.coachid === userId))
      setRosters(rosterList)

      if (rosterList.length > 0) {
        const quizzerResults = await Promise.all(
          rosterList.map(r => RosterAPI.getQuizzers(r.rosterid).catch(() => [] as UserTS[]))
        )
        const map: Record<string, UserTS[]> = {}
        rosterList.forEach((r, i) => { map[r.rosterid] = quizzerResults[i] })
        setQuizzersByRoster(map)
      }
    } catch (e: any) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }, [userId, tid])

  useEffect(() => { load() }, [load])

  // ── Derived ─────────────────────────────────────────────────────────────────

  const allQuizzers = Object.values(quizzersByRoster)
    .flat()
    .filter((u, i, arr) => arr.findIndex(x => x.id === u.id) === i)

  const pickerQuizzers: UserTS[] =
    selectedRosterId === 'all'
      ? allQuizzers
      : (quizzersByRoster[selectedRosterId] ?? [])

  const divisionName = (did: string) => divisions.find(d => d.did === did)?.dname ?? did

  const activeQuizzersInForm = new Set(form.quizzers.filter((q): q is string => q !== null))

  const selectedRosterQuizzers =
    selectedRosterId !== 'all' ? (quizzersByRoster[selectedRosterId] ?? []) : []

  const canAssignAll =
    selectedRosterId !== 'all' &&
    selectedRosterQuizzers.length > 0 &&
    selectedRosterQuizzers.length <= MAX_QUIZZERS

  // ── Form helpers ─────────────────────────────────────────────────────────────

  const openCreate = () => {
    setEditingTeamId(null)
    setForm(emptyForm())
    setFormError(null)
    setSelectedRosterId('all')
    setFormOpen(true)
  }

  const openEdit = (team: TeamTS) => {
    setEditingTeamId(team.teamid)
    setForm(teamToForm(team))
    setFormError(null)
    setSelectedRosterId('all')
    setFormOpen(true)
  }

  const closeForm = () => {
    setFormOpen(false)
    setEditingTeamId(null)
    setForm(emptyForm())
    setFormError(null)
  }

  const toggleQuizzer = (quizzerId: string) => {
    setForm(prev => {
      const slots = [...prev.quizzers]
      const existingIdx = slots.indexOf(quizzerId)
      if (existingIdx >= 0) {
        slots[existingIdx] = null
        return { ...prev, quizzers: slots }
      }
      const emptyIdx = slots.indexOf(null)
      if (emptyIdx < 0) return prev // all 6 slots full
      slots[emptyIdx] = quizzerId
      return { ...prev, quizzers: slots }
    })
  }

  const clearSlot = (idx: number) => {
    setForm(prev => {
      const slots = [...prev.quizzers]
      slots[idx] = null
      return { ...prev, quizzers: slots }
    })
  }

  const assignAllFromRoster = (rosterId: string) => {
    const quizzers = quizzersByRoster[rosterId] ?? []
    if (quizzers.length > MAX_QUIZZERS) return
    const slots: (string | null)[] = [
      ...quizzers.map(q => q.id),
      ...Array<null>(MAX_QUIZZERS - quizzers.length).fill(null),
    ]
    setForm(prev => ({ ...prev, quizzers: slots }))
  }

  // ── Save / Delete ────────────────────────────────────────────────────────────

  const handleSave = async () => {
    if (!form.name.trim()) { setFormError('Team name is required.'); return }
    if (!form.did) { setFormError('Division is required.'); return }
    if (!userId || !accessToken) return

    setSaving(true)
    setFormError(null)
    try {
      if (editingTeamId) {
        const updated = await TeamAPI.update(editingTeamId, formToChangeset(form), accessToken)
        setMyTeams(prev => prev.map(t => t.teamid === editingTeamId ? updated : t))
        closeForm()
      } else {
        const created = await TeamAPI.create({
          name: form.name,
          did: form.did,
          coachid: userId,
          quizzer_one_id: form.quizzers[0],
          quizzer_two_id: form.quizzers[1],
          quizzer_three_id: form.quizzers[2],
          quizzer_four_id: form.quizzers[3],
          quizzer_five_id: form.quizzers[4],
          quizzer_six_id: form.quizzers[5],
        }, accessToken)
        setMyTeams(prev => [...prev, created])
        closeForm()
      }
    } catch (e: any) {
      if (e.message?.includes('(409)')) {
        setFormError('Team name already taken by another team in the team\'s division. Please choose another team name.')
      } else {
        setFormError(e.message)
      }
    } finally {
      setSaving(false)
    }
  }

  const confirmDelete = (team: TeamTS) => {
    setConfirmDialog({
      isOpen: true,
      title: 'Delete this team?',
      message: `"${team.name}" will be permanently removed from this tournament.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: async () => {
        setConfirmDialog(confirmDialogDefaultState)
        try {
          await TeamAPI.delete(team.teamid, accessToken)
          setMyTeams(prev => prev.filter(t => t.teamid !== team.teamid))
          if (editingTeamId === team.teamid) closeForm()
        } catch (e: any) {
          setError(e.message)
        }
      },
    })
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
        Please <strong>sign in</strong> or <strong>create an account</strong> to register a team
        for this tournament.
      </Alert>
    )
  }

  // ── Loading ──────────────────────────────────────────────────────────────────

  if (loading) {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 3 }}>
        <CircularProgress size={20} />
        <Typography color="text.secondary">Loading…</Typography>
      </Box>
    )
  }

  // ── Hard error ───────────────────────────────────────────────────────────────

  if (error && myTeams.length === 0 && !formOpen) {
    return (
      <Alert severity="error" action={
        <Button color="inherit" size="small" onClick={() => { setError(null); load() }}>Retry</Button>
      }>
        {error}
      </Alert>
    )
  }

  // ── UI ────────────────────────────────────────────────────────────────────────

  return (
    <Box>
      {/* ── Header ── */}
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
        <Typography variant="h6">My Registered Teams</Typography>
        <Button
          size="small"
          variant="contained"
          startIcon={<AddIcon />}
          sx={{ ml: 'auto' }}
          onClick={openCreate}
          disabled={formOpen && editingTeamId === null}
        >
          Register New Team
        </Button>
      </Box>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
      )}

      {/* ── Teams table ── */}
      {myTeams.length === 0 ? (
        <Typography variant="body2" color="text.secondary" sx={{ mb: 3 }}>
          No teams registered for this tournament yet.
        </Typography>
      ) : (
        <TableContainer component={Paper} variant="outlined" sx={{ borderRadius: 2, mb: 3 }}>
          <Table size="small">
            <TableHead>
              <TableRow sx={{ '& th': { fontWeight: 600, backgroundColor: 'action.hover' } }}>
                <TableCell>Name</TableCell>
                <TableCell>Division</TableCell>
                <TableCell>Quizzers</TableCell>
                <TableCell align="right">Actions</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {myTeams.map(team => {
                const quizzerCount = [
                  team.quizzer_one_id, team.quizzer_two_id, team.quizzer_three_id,
                  team.quizzer_four_id, team.quizzer_five_id, team.quizzer_six_id,
                ].filter(Boolean).length
                const isEditing = editingTeamId === team.teamid
                return (
                  <TableRow key={team.teamid} selected={isEditing} hover>
                    <TableCell>
                      <Typography
                        variant="body2"
                        fontWeight={isEditing ? 600 : 400}
                        onClick={() => isEditing ? closeForm() : openEdit(team)}
                        sx={{ cursor: 'pointer', '&:hover': { textDecoration: 'underline' } }}
                      >
                        {team.name}
                      </Typography>
                    </TableCell>
                    <TableCell>
                      <Typography variant="body2">{divisionName(team.did)}</Typography>
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={`${quizzerCount} / ${MAX_QUIZZERS}`}
                        size="small"
                        color={quizzerCount === MAX_QUIZZERS ? 'success' : quizzerCount === 0 ? 'default' : 'warning'}
                        variant="outlined"
                      />
                    </TableCell>
                    <TableCell align="right">
                      <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 0.5 }}>
                        <IconButton
                          size="small"
                          onClick={() => isEditing ? closeForm() : openEdit(team)}
                          title={isEditing ? 'Cancel editing' : 'Edit team'}
                        >
                          <EditIcon fontSize="small" color={isEditing ? 'primary' : undefined} />
                        </IconButton>
                        {session?.hasPermission('team:delete') && (
                          <IconButton size="small" color="error" onClick={() => confirmDelete(team)}>
                            <DeleteIcon fontSize="small" />
                          </IconButton>
                        )}
                      </Box>
                    </TableCell>
                  </TableRow>
                )
              })}
            </TableBody>
          </Table>
        </TableContainer>
      )}

      {/* ── Team form ── */}
      {formOpen && (
        <Paper variant="outlined" sx={{ p: 2.5, borderRadius: 2 }}>
          {formError && (
            <Alert severity="error" sx={{ mb: 2 }} onClose={() => setFormError(null)}>
              {formError}
            </Alert>
          )}

          <Stack spacing={2.5}>
            {/* ── Name + Division ── */}
            <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
              <Box sx={{ flex: '1 1 200px' }}>
                <Typography variant="caption" color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
                  Team Name *
                </Typography>
                <TextField
                  size="small"
                  fullWidth
                  placeholder="Team Name"
                  value={form.name}
                  onChange={e => setForm(p => ({ ...p, name: e.target.value }))}
                />
              </Box>
              <Box sx={{ flex: '1 1 200px' }}>
                <Typography variant="caption" color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
                  Division *
                </Typography>
                <Select
                  size="small"
                  fullWidth
                  value={form.did}
                  onChange={e => setForm(p => ({ ...p, did: e.target.value }))}
                  displayEmpty
                  renderValue={val => val ? divisionName(val) : <em>Select a division</em>}
                >
                  {divisions.map(d => (
                    <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>
                  ))}
                </Select>
              </Box>
              <Box sx={{ flex: '1 1 160px' }}>
                <Typography variant="caption" color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
                  Coach
                </Typography>
                <Typography variant="body2" color="text.secondary" sx={{ mt: 0.75 }}>
                  You (logged-in user)
                </Typography>
              </Box>
            </Box>

            <Divider />

            {/* ── Quizzer slots ── */}
            <Box>
              <Typography variant="caption" color="text.secondary" sx={{ mb: 1, display: 'block' }}>
                Quizzer Slots — {activeQuizzersInForm.size} / {MAX_QUIZZERS} assigned
              </Typography>
              <Box sx={{ display: 'flex', gap: 1, flexWrap: 'wrap' }}>
                {form.quizzers.map((qid, idx) => {
                  const quizzer = qid ? allQuizzers.find(u => u.id === qid) : null
                  return (
                    <Chip
                      key={idx}
                      label={quizzer ? userLabel(quizzer) : `Slot ${idx + 1}`}
                      color={quizzer ? 'primary' : 'default'}
                      variant={quizzer ? 'filled' : 'outlined'}
                      onDelete={quizzer ? () => clearSlot(idx) : undefined}
                      sx={{ opacity: quizzer ? 1 : 0.45 }}
                    />
                  )
                })}
              </Box>
            </Box>

            <Divider />

            {/* ── Quizzer picker ── */}
            <Box>
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, mb: 1.5, flexWrap: 'wrap' }}>
                <Typography variant="caption" color="text.secondary">
                  Manager team quizzers by roster:
                </Typography>
                <ToggleButtonGroup
                  size="small"
                  exclusive
                  value={selectedRosterId}
                  onChange={(_, val) => { if (val) setSelectedRosterId(val) }}
                  sx={{ flexWrap: 'wrap', gap: 0.5 }}
                >
                  <ToggleButton value="all">All Quizzers</ToggleButton>
                  {rosters.map(r => (
                    <ToggleButton key={r.rosterid} value={r.rosterid}>{r.name}</ToggleButton>
                  ))}
                </ToggleButtonGroup>
                {canAssignAll && (
                  <Button
                    size="small"
                    variant="outlined"
                    startIcon={<GroupsIcon />}
                    onClick={() => assignAllFromRoster(selectedRosterId)}
                  >
                    Assign All ({selectedRosterQuizzers.length})
                  </Button>
                )}
              </Box>

              {pickerQuizzers.length === 0 ? (
                <Typography variant="body2" color="text.secondary">
                  {rosters.length === 0
                    ? 'No rosters found.'
                    : 'No quizzers in this roster.'}
                </Typography>
              ) : (
                <Box sx={{ display: 'flex', gap: 0.75, flexWrap: 'wrap' }}>
                  {pickerQuizzers.map(u => {
                    const assigned = activeQuizzersInForm.has(u.id)
                    const full = activeQuizzersInForm.size >= MAX_QUIZZERS && !assigned
                    return (
                      <Chip
                        key={u.id}
                        label={userLabel(u)}
                        color={assigned ? 'success' : 'default'}
                        variant={assigned ? 'filled' : 'outlined'}
                        onClick={full ? undefined : () => toggleQuizzer(u.id)}
                        sx={{ cursor: full ? 'not-allowed' : 'pointer', opacity: full ? 0.45 : 1 }}
                      />
                    )
                  })}
                </Box>
              )}
            </Box>

            {/* ── Form actions ── */}
            <Box sx={{ display: 'flex', gap: 1, pt: 0.5 }}>
              <Button
                variant="contained"
                disabled={saving}
                onClick={handleSave}
                startIcon={saving ? <CircularProgress size={14} /> : undefined}
              >
                {editingTeamId ? 'Save Changes' : 'Register Team'}
              </Button>
              <Button variant="outlined" onClick={closeForm} disabled={saving}>
                Cancel
              </Button>
              {editingTeamId && session?.hasPermission('team:delete') && (
                <Button
                  variant="outlined"
                  color="error"
                  disabled={saving}
                  startIcon={<DeleteIcon />}
                  sx={{ ml: 'auto' }}
                  onClick={() => {
                    const t = myTeams.find(tt => tt.teamid === editingTeamId)
                    if (t) confirmDelete(t)
                  }}
                >
                  Remove Team Registration
                </Button>
              )}
            </Box>
          </Stack>
        </Paper>
      )}

      {/* ── No rosters nudge ── */}
      {!formOpen && rosters.length === 0 && (
        <Alert severity="info" sx={{ mt: 2 }}>
          You have no rosters yet.{' '}
          <Button
            size="small"
            variant="text"
            sx={{ p: 0, minWidth: 0, textDecoration: 'underline', verticalAlign: 'baseline' }}
            onClick={() => navigate(`/user/${userId}/my-rosters`)}
          >
            Add quizzers to a roster
          </Button>
          {' '}in your profile to assign them to teams here.
        </Alert>
      )}

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        title={confirmDialog.title}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
      />
    </Box>
  )
}
