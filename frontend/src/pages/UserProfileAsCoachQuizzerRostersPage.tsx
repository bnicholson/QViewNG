import { useCallback, useEffect, useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import Tab from '@mui/material/Tab'
import Tabs from '@mui/material/Tabs'
import Typography from '@mui/material/Typography'
import AddIcon from '@mui/icons-material/Add'
import DeleteIcon from '@mui/icons-material/Delete'
import EditIcon from '@mui/icons-material/Edit'
import GroupIcon from '@mui/icons-material/Group'
import PersonAddIcon from '@mui/icons-material/PersonAdd'
import { RosterAPI, type RosterTS } from '../features/RosterAPI'
import { RosterEditorDialog } from '../components/RosterEditorDialog'
import { QuizzerCreatorDialog } from '../components/QuizzerCreatorDialog'
import { UserPickerDialog } from '../components/UserPickerDialog'
import { ConfirmDialog, confirmDialogDefaultState } from '../components/ConfirmDialog'
import QuizzersTable from '../components/QuizzersTable'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { Link } from 'react-router-dom'

// ── Shared hook: aggregate all quizzers across rosters ──────────────────────

function useAllQuizzers(userId: string) {
  const [allQuizzers, setAllQuizzers] = useState<UserTS[]>([]);
  const [loading, setLoading] = useState(false);

  // Single scoped call: the endpoint aggregates + de-duplicates quizzers across the coach's rosters.
  const loadAll = useCallback(async () => {
    setLoading(true);
    try {
      const { items } = await UserAPI.getRosterQuizzerRows(userId, 0, 500);
      setAllQuizzers(items.map(q => ({
        id: q.quizzer_id,
        username: '',
        email: q.email,
        fname: q.fname,
        mname: q.mname,
        lname: q.lname,
        activated: true,
        created_at: q.created_at,
        updated_at: q.updated_at,
      })));
    } catch {
      console.error('Failed to aggregate quizzers');
    } finally {
      setLoading(false);
    }
  }, [userId]);

  useEffect(() => { loadAll(); }, [loadAll]);

  return { allQuizzers, loading, reload: loadAll };
}

// ── All Quizzers Tab ────────────────────────────────────────────────────────

function AllQuizzersPanel({ allQuizzers, loading, rosterCount }: { allQuizzers: UserTS[]; loading: boolean; rosterCount: number }) {
  if (loading) return <Typography color="text.secondary" sx={{ py: 4 }}>Loading all quizzers...</Typography>;

  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: "left" }}>
        As a Coach, rosters organize your quizzers and are used when registering teams for tournaments. 
        Each tab above (except for this tab) is a roster you manage.
        Quizzers can exist in 1 or more rosters. 
        You can share rosters with your program's other coaches and co-leaders also.
        <br/>
        This tab shows all quizzers across your {rosterCount} roster{rosterCount !== 1 ? 's' : ''} (without duplicates).
      </Typography>
      <QuizzersTable externalRows={allQuizzers} />
    </Box>
  );
}

// ── Coaches Section ─────────────────────────────────────────────────────────

function CoachesSection({ rosterId, createdByUserId, currentUserId, onSelfRemoved }: { rosterId: string; createdByUserId: string; currentUserId: string; onSelfRemoved: () => void }) {
  const [coaches, setCoaches] = useState<UserTS[]>([]);
  const [pickerOpen, setPickerOpen] = useState(false);
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const loadCoaches = useCallback(async () => {
    try {
      setCoaches(await RosterAPI.getCoaches(rosterId));
    } catch {
      console.error('Failed to load coaches');
    }
  }, [rosterId]);

  useEffect(() => { loadCoaches(); }, [loadCoaches]);

  const handleAddCoach = async (user: UserTS) => {
    await RosterAPI.addCoach(rosterId, user.id);
    await loadCoaches();
  };

  const handleRemoveCoach = (coach: UserTS) => {
    if (coach.id === createdByUserId) return; // Can't remove the creator
    const removingSelf = coach.id === currentUserId;
    setConfirmDialog({
      isOpen: true,
      title: removingSelf ? 'Remove yourself from this roster?' : 'Remove coach access?',
      message: removingSelf
        ? 'You will no longer be able to view or manage this roster, and it will be removed from your rosters.'
        : `${coach.fname} ${coach.lname} will no longer be able to view or manage this roster.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: async () => {
        setConfirmDialog(confirmDialogDefaultState);
        await RosterAPI.removeCoach(rosterId, coach.id);
        if (removingSelf) {
          // Losing my own access — drop this roster's tab and return to All Quizzers.
          onSelfRemoved();
        } else {
          await loadCoaches();
        }
      },
    });
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1.5 }}>
        <GroupIcon fontSize="small" color="action" />
        <Typography variant="subtitle1" fontWeight={600}>Shared With (Coaches)</Typography>
        <Button
          size="small"
          startIcon={<PersonAddIcon />}
          onClick={() => setPickerOpen(true)}
          sx={{ ml: 'auto', textTransform: 'none' }}
        >
          Add Coach
        </Button>
      </Box>

      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
        {coaches.map(c => (
          <Chip
            key={c.id}
            label={
              <Link
                to={`/user/${c.id}/overview`}
                style={{ color: 'inherit', textDecoration: 'none' }}
                onClick={e => e.stopPropagation()}
              >
                {`${c.fname} ${c.lname}`}{c.id === createdByUserId ? ' (owner)' : ''}
              </Link>
            }
            variant={c.id === createdByUserId ? 'filled' : 'outlined'}
            color={c.id === createdByUserId ? 'primary' : 'default'}
            onDelete={c.id === createdByUserId ? undefined : () => handleRemoveCoach(c)}
            deleteIcon={<DeleteIcon fontSize="small" />}
          />
        ))}
        {coaches.length === 0 && (
          <Typography variant="body2" color="text.secondary">No coaches assigned.</Typography>
        )}
      </Box>

      <UserPickerDialog
        isOpen={pickerOpen}
        title="Add Coach to Roster"
        note="Note: Coaches added to this (or any) roster can add the quizzers of this roster to other rosters that they create."
        excludeIds={coaches.map(c => c.id)}
        minSearchChars={3}
        onCancel={() => setPickerOpen(false)}
        onPick={async (user) => {
          await handleAddCoach(user);
          setPickerOpen(false);
        }}
      />

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
        title={confirmDialog.title}
      />
    </Box>
  );
}

// ── Single Roster Tab ───────────────────────────────────────────────────────

function RosterPanel({
  roster,
  allQuizzers,
  currentUserId,
  onEditRoster,
  onDeleteRoster,
  onLeaveRoster,
  onQuizzersChanged,
}: {
  roster: RosterTS;
  allQuizzers: UserTS[];
  currentUserId: string;
  onEditRoster: () => void;
  onDeleteRoster: () => void;
  onLeaveRoster: () => void;
  onQuizzersChanged: () => void;
}) {
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [creatorOpen, setCreatorOpen] = useState(false);
  const [pickerOpen, setPickerOpen] = useState(false);
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const loadQuizzers = useCallback(async () => {
    try {
      setQuizzers(await RosterAPI.getQuizzers(roster.rosterid));
    } catch {
      console.error('Failed to load quizzers');
      setQuizzers([]);
    }
  }, [roster.rosterid]);

  useEffect(() => { loadQuizzers(); }, [loadQuizzers]);

  const handleRemoveQuizzer = async (user: UserTS) => {
    await RosterAPI.removeQuizzer(roster.rosterid, user.id);
    setQuizzers(prev => prev.filter(q => q.id !== user.id));
    onQuizzersChanged();
  };

  const handleAddExisting = async (user: UserTS) => {
    await RosterAPI.addQuizzer(roster.rosterid, user.id);
    await loadQuizzers();
    onQuizzersChanged();
  };

  return (
    <Box>
      {/* Roster Header */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', mb: 2, flexWrap: 'wrap', gap: 1 }}>
        <Box>
          {roster.description && (
            <Typography variant="body2" color="text.secondary">
              {roster.description}
            </Typography>
          )}
          <Typography variant="caption" color="text.disabled">
            Created {new Date(roster.created_at).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })}
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', gap: 1 }}>
          <Button size="small" variant="outlined" startIcon={<EditIcon />} onClick={onEditRoster} sx={{ textTransform: 'none' }}>
            Edit Roster
          </Button>
          <Button
            size="small"
            variant="outlined"
            color="error"
            startIcon={<DeleteIcon />}
            onClick={() => setConfirmDialog({
              isOpen: true,
              title: 'Delete this roster?',
              message: `"${roster.name}" and all its quizzer/coach associations will be permanently removed. The users themselves will not be deleted.`,
              onCancel: () => setConfirmDialog(confirmDialogDefaultState),
              onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); onDeleteRoster(); },
            })}
            sx={{ textTransform: 'none' }}
          >
            Delete Roster
          </Button>
        </Box>
      </Box>

      <Divider sx={{ mb: 2 }} />

      {/* Coaches Section */}
      <CoachesSection
        rosterId={roster.rosterid}
        createdByUserId={roster.created_by_userid}
        currentUserId={currentUserId}
        onSelfRemoved={onLeaveRoster}
      />

      <Divider sx={{ my: 3 }} />

      {/* Quizzers Section — the table renders its own "Quizzers" title; the actions sit on that line. */}
      <QuizzersTable
        externalRows={quizzers}
        onDelete={handleRemoveQuizzer}
        headerActions={
          <>
            <Button
              size="small"
              variant="contained"
              startIcon={<AddIcon />}
              onClick={() => setCreatorOpen(true)}
              sx={{ textTransform: 'none' }}
            >
              Create New Quizzer
            </Button>
            <Button
              size="small"
              variant="outlined"
              startIcon={<PersonAddIcon />}
              onClick={() => setPickerOpen(true)}
              sx={{ textTransform: 'none' }}
            >
              Add Existing
            </Button>
          </>
        }
      />

      {/* Dialogs */}
      <QuizzerCreatorDialog
        rosterId={roster.rosterid}
        rosterName={roster.name}
        isOpen={creatorOpen}
        onCancel={() => setCreatorOpen(false)}
        onSave={() => { setCreatorOpen(false); loadQuizzers(); onQuizzersChanged(); }}
      />

      <UserPickerDialog
        isOpen={pickerOpen}
        title={`Add Existing Quizzer to "${roster.name}"`}
        note="Select one of your existing quizzers to add to this roster."
        excludeIds={quizzers.map(q => q.id)}
        availableUsers={allQuizzers}
        onCancel={() => setPickerOpen(false)}
        onPick={async (user) => {
          await handleAddExisting(user);
          setPickerOpen(false);
        }}
      />

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
        title={confirmDialog.title}
      />
    </Box>
  );
}

// ── Main Page ───────────────────────────────────────────────────────────────

export const UserProfileAsCoachQuizzerRostersPage = (props: { userId: string; isSuperUser: boolean }) => {
  const { userId } = props;

  const [rosters, setRosters] = useState<RosterTS[]>([]);
  const [tabIndex, setTabIndex] = useState(0); // 0 = All Quizzers, 1..N = roster tabs
  const { allQuizzers, loading: allQuizzersLoading, reload: reloadAllQuizzers } = useAllQuizzers(userId);

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);

  const loadRosters = useCallback(async () => {
    try {
      const items = await RosterAPI.getByCoach(userId);
      setRosters(items);
    } catch {
      console.error('Failed to load rosters');
    }
  }, [userId]);

  useEffect(() => { loadRosters(); }, [loadRosters]);

  const selectedRoster = tabIndex > 0 && tabIndex <= rosters.length ? rosters[tabIndex - 1] : null;

  const handleCreateSave = (roster: RosterTS) => {
    setCreateDialogOpen(false);
    loadRosters().then(() => {
      // Switch to the newly created roster's tab
      // It will be appended at the end of the list
      setRosters(prev => {
        const idx = prev.findIndex(r => r.rosterid === roster.rosterid);
        if (idx >= 0) setTabIndex(idx + 1);
        return prev;
      });
    });
  };

  const handleEditSave = (roster: RosterTS) => {
    setEditDialogOpen(false);
    setRosters(prev => prev.map(r => r.rosterid === roster.rosterid ? roster : r));
  };

  const handleDeleteRoster = async () => {
    if (!selectedRoster) return;
    try {
      await RosterAPI.delete(selectedRoster.rosterid);
      // Bump back to the All Quizzers tab, drop the deleted roster's tab, and refresh the
      // aggregated quizzer list so it no longer reflects the removed roster.
      setTabIndex(0);
      await Promise.all([loadRosters(), reloadAllQuizzers()]);
    } catch (err: any) {
      console.error('Failed to delete roster:', err.message);
    }
  };

  // A shared coach removed their own access: the roster is no longer theirs to see, so drop its
  // tab and return them to All Quizzers.
  const handleLeaveRoster = async () => {
    setTabIndex(0);
    await loadRosters();
  };

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    // Last tab is the "+" button — open create dialog instead of switching
    if (newValue === rosters.length + 1) {
      setCreateDialogOpen(true);
      return;
    }
    setTabIndex(newValue);
  };

  return (
    <Box>
      {/* Tab Bar */}
      <Box sx={{ borderBottom: 1, borderColor: 'divider', mb: 2 }}>
        <Tabs
          value={tabIndex}
          onChange={handleTabChange}
          variant="scrollable"
          scrollButtons="auto"
          sx={{
            '& .MuiTab-root': { textTransform: 'none', minHeight: 48, fontWeight: 500 },
          }}
        >
          <Tab label="My Quizzers" />
          {rosters.map(r => (
            <Tab key={r.rosterid} label={r.name} />
          ))}
          <Tab
            icon={<AddIcon fontSize="small" />}
            aria-label="Create new roster"
            sx={{ minWidth: 48 }}
          />
        </Tabs>
      </Box>

      {/* Tab Content */}
      {tabIndex === 0 && (
        <AllQuizzersPanel allQuizzers={allQuizzers} loading={allQuizzersLoading} rosterCount={rosters.length} />
      )}

      {selectedRoster && (
        <RosterPanel
          key={selectedRoster.rosterid}
          roster={selectedRoster}
          allQuizzers={allQuizzers}
          currentUserId={userId}
          onEditRoster={() => setEditDialogOpen(true)}
          onDeleteRoster={handleDeleteRoster}
          onLeaveRoster={handleLeaveRoster}
          onQuizzersChanged={reloadAllQuizzers}
        />
      )}

      {rosters.length === 0 && tabIndex === 0 && (
        <Box sx={{ textAlign: 'center', py: 6 }}>
          <Typography variant="body1" color="text.secondary" sx={{ mb: 2 }}>
            No rosters yet. Create your first roster to start managing quizzers.
          </Typography>
          <Button variant="contained" startIcon={<AddIcon />} onClick={() => setCreateDialogOpen(true)}>
            Create Roster
          </Button>
        </Box>
      )}

      {/* Dialogs */}
      <RosterEditorDialog
        coachId={userId}
        isOpen={createDialogOpen}
        onCancel={() => setCreateDialogOpen(false)}
        onSave={handleCreateSave}
      />

      {selectedRoster && (
        <RosterEditorDialog
          coachId={userId}
          isOpen={editDialogOpen}
          onCancel={() => setEditDialogOpen(false)}
          onSave={handleEditSave}
          editingRoster={selectedRoster}
        />
      )}
    </Box>
  );
};
