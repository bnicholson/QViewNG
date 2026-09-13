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
import TextField from '@mui/material/TextField'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import Typography from '@mui/material/Typography'
import AddIcon from '@mui/icons-material/Add'
import DeleteIcon from '@mui/icons-material/Delete'
import EditIcon from '@mui/icons-material/Edit'
import GroupsIcon from '@mui/icons-material/Groups'
import { TeamAPI, type TeamTS, type MyTeamTS, type TeamChangeset } from '../features/TeamAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { RosterAPI, type RosterTS } from '../features/RosterAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { useAuth } from '../hooks/useAuth'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate'

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

  const [myTeams, setMyTeams] = useState<MyTeamTS[]>([])
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const [divisions, setDivisions] = useState<DivisionTS[]>([])
  const [rosters, setRosters] = useState<RosterTS[]>([])
  const [quizzersByRoster, setQuizzersByRoster] = useState<Record<string, UserTS[]>>({})
  // The coach's full "My Quizzers" aggregate (roster quizzers + quizzers they created, even if on
  // no roster). Drives the nudge / table visibility and the picker's "All Quizzers" tab.
  const [allQuizzersFull, setAllQuizzersFull] = useState<UserTS[]>([])
  const myQuizzersCount = allQuizzersFull.length

  // Keep the page in range as teams are added/removed.
  useEffect(() => { setPage(0) }, [myTeams.length])

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
      const [divs, myTeamRows, rosterList, myQuizzers] = await Promise.all([
        DivisionAPI.getByTournament(tid, 0, 100),
        // One call returns exactly this user's registered teams for the tournament, enriched with
        // division name — no over-fetching all tournament teams or a second lookup for names.
        TeamAPI.getMyByTournament(tid, accessToken),
        RosterAPI.getByCoach(userId),
        // Same aggregate as the "My Quizzers" table — includes quizzers the coach created.
        UserAPI.getRosterQuizzerRows(userId, 0, 500).catch(() => ({ count: 0, items: [] })),
      ])
      setDivisions(divs)
      setMyTeams(myTeamRows)
      setRosters(rosterList)
      setAllQuizzersFull(myQuizzers.items.map(q => ({
        id: q.quizzer_id,
        username: '',
        email: q.email,
        fname: q.fname,
        mname: q.mname,
        lname: q.lname,
        activated: true,
        created_at: q.created_at,
        updated_at: q.updated_at,
        del_fl: false,
      })))

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
  }, [userId, tid, accessToken])

  useEffect(() => { load() }, [load])

  // ── Derived ─────────────────────────────────────────────────────────────────

  // True when the coach's "My Quizzers" aggregate is empty — counting both roster quizzers and
  // quizzers they created (even if on no roster). Drives the nudge notice and hides the table.
  const hasNoQuizzers = myQuizzersCount === 0

  // "All Quizzers" tab shows the full aggregate (incl. created quizzers); each roster tab shows
  // only that roster's quizzers.
  const pickerQuizzers: UserTS[] =
    selectedRosterId === 'all'
      ? allQuizzersFull
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
      // Registering / editing is a single write; we enrich the returned team with its division name
      // locally (from the divisions already loaded) so no extra fetch is needed to refresh the table.
      if (editingTeamId) {
        const updated = await TeamAPI.update(editingTeamId, formToChangeset(form), accessToken)
        const enriched: MyTeamTS = { ...updated, division_name: divisionName(updated.did) }
        setMyTeams(prev => prev.map(t => t.teamid === editingTeamId ? enriched : t))
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
        const enriched: MyTeamTS = { ...created, division_name: divisionName(created.did) }
        setMyTeams(prev => [...prev, enriched])
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

  // Teams table columns. Name links to the team's profile and Division to the division's profile;
  // the Quizzers chip and Edit/Delete actions are unchanged.
  const teamColumns: ColumnDef<TeamTS>[] = [
    {
      header: 'Name',
      render: (team) => <EntityLink to={`/team/${team.teamid}`}>{team.name}</EntityLink>,
    },
    {
      header: 'Division',
      render: (team) => <EntityLink to={`/division/${team.did}`}>{divisionName(team.did)}</EntityLink>,
    },
    {
      header: 'Quizzers',
      render: (team) => {
        const quizzerCount = [
          team.quizzer_one_id, team.quizzer_two_id, team.quizzer_three_id,
          team.quizzer_four_id, team.quizzer_five_id, team.quizzer_six_id,
        ].filter(Boolean).length
        return (
          <Chip
            label={`${quizzerCount} / ${MAX_QUIZZERS}`}
            size="small"
            color={quizzerCount === MAX_QUIZZERS ? 'success' : quizzerCount === 0 ? 'default' : 'warning'}
            variant="outlined"
          />
        )
      },
    },
    {
      header: 'Actions',
      render: (team) => {
        const isEditing = editingTeamId === team.teamid
        return (
          <Box sx={{ display: 'flex', justifyContent: 'center', gap: 0.5 }}>
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
        )
      },
    },
  ]

  // ── UI ────────────────────────────────────────────────────────────────────────

  return (
    <Box>
      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
      )}

      {/* ── Teams table ── hidden entirely when the user has no quizzers (the nudge shows instead) ── */}
      {!hasNoQuizzers && (
      <Box sx={{ mb: 3 }}>
        <DataTableTemplate<TeamTS>
          entityLabel="Team"
          title="My Registered Teams"
          showCreateButton={false}
          showDeleteButton={false}
          columns={teamColumns}
          rows={myTeams.slice(page * pageSize, (page + 1) * pageSize)}
          totalCount={myTeams.length}
          getId={(t) => t.teamid}
          onDelete={async () => {}}
          page={page}
          pageSize={pageSize}
          onPageChange={setPage}
          onPageSizeChange={(s) => { setPage(0); setPageSize(s) }}
          getRowStyle={(t) => editingTeamId === t.teamid ? { background: '#eef2ff' } : undefined}
          headerActions={
            <Button
              size="small"
              variant="contained"
              startIcon={<AddIcon />}
              onClick={openCreate}
              disabled={formOpen && editingTeamId === null}
            >
              Register New Team
            </Button>
          }
        />
      </Box>
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
                  const quizzer = qid ? allQuizzersFull.find(u => u.id === qid) : null
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
                {rosters.length === 0 && (
                  <Button
                    size="small"
                    variant="outlined"
                    onClick={() => navigate(`/user/${userId}/my-rosters`)}
                    sx={{ textTransform: 'none' }}
                  >
                    No Rosters? Click here to create one now.
                  </Button>
                )}
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

      {/* ── No quizzers nudge (shown when the "My Quizzers" aggregate would be empty) ── */}
      {!formOpen && hasNoQuizzers && (
        <Stack spacing={2} alignItems="flex-start" sx={{ mt: 2 }}>
          <Alert severity="info" sx={{ width: '100%' }}>
            You have no quizzers yet. Add quizzers to a roster in your profile to assign them to teams here.
          </Alert>
          <Button variant="outlined" onClick={() => navigate(`/user/${userId}/my-rosters`)}>
            Go to My Rosters →
          </Button>
        </Stack>
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
