import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import Divider from '@mui/material/Divider'
import Typography from '@mui/material/Typography'
import QuizzersTable from '../components/QuizzersTable'
import { QuizzerPickerDialog } from '../components/QuizzerPickerDialog'
import { TeamAPI, type TeamTS, type TeamChangeset } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { useAuth } from '../hooks/useAuth'

const SLOT_FIELDS = [
  'quizzer_one_id',
  'quizzer_two_id',
  'quizzer_three_id',
  'quizzer_four_id',
  'quizzer_five_id',
  'quizzer_six_id',
] as const;

type SlotField = typeof SLOT_FIELDS[number];

function slotsFromTeam(team: TeamTS): (string | null)[] {
  return SLOT_FIELDS.map(f => team[f] ?? null);
}

interface Props {
  teamid: string;
  showSensitiveColumns?: boolean;
  showAuditColumns?: boolean;
}

export const TeamProfileQuizzersPage = ({ teamid, showSensitiveColumns, showAuditColumns }: Props) => {
  const { accessToken } = useAuth();
  const [isLoading, setIsLoading] = useState(false);
  const [notFound, setNotFound] = useState(false);
  const [team, setTeam] = useState<TeamTS | undefined>(undefined);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [slots, setSlots] = useState<(string | null)[]>([null, null, null, null, null, null]);
  const [pickerOpen, setPickerOpen] = useState(false);

  useEffect(() => {
    setIsLoading(true);
    Promise.all([TeamAPI.getById(teamid), UserAPI.get(0, 200)])
      .then(([tm, u]) => {
        setTeam(tm);
        setUsers(u.items);
        setSlots(slotsFromTeam(tm));
      })
      .catch((err) => {
        console.error('Failed to load team quizzers:', err);
        setNotFound(true);
      })
      .finally(() => setIsLoading(false));
  }, [teamid]);

  const handlePickerConfirm = async (selected: UserTS[]) => {
    setPickerOpen(false);
    const next = [...slots];
    let si = 0;
    for (let i = 0; i < next.length && si < selected.length; i++) {
      if (!next[i]) next[i] = selected[si++].id;
    }
    const changeset: TeamChangeset = {};
    SLOT_FIELDS.forEach((f, i) => { changeset[f] = next[i] ?? null; });
    try {
      const updatedTeam = await TeamAPI.update(teamid, changeset, accessToken);
      setTeam(updatedTeam);
      setSlots(slotsFromTeam(updatedTeam));
    } catch (err) {
      console.error('Failed to save quizzer assignments:', err);
    }
  };

  // Removes a quizzer from whichever roster slot holds them.
  const handleRemove = async (user: UserTS) => {
    if (!team) return;
    const slotField = SLOT_FIELDS.find((f): f is SlotField => team[f] === user.id);
    if (!slotField) return;
    const updatedTeam = await TeamAPI.update(teamid, { [slotField]: null }, accessToken);
    setTeam(updatedTeam);
    setSlots(slotsFromTeam(updatedTeam));
  };

  if (notFound) return <div>Team not found.</div>;
  if (isLoading || !team) return <div>Loading...</div>;

  const assignedUsers = slots
    .filter(Boolean)
    .map(id => users.find(u => u.id === id))
    .filter((u): u is UserTS => u !== undefined);

  const assignedIds = slots.filter(Boolean) as string[];
  const openSlots = slots.filter(s => !s).length;

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 1 }}>
        <Typography variant="subtitle1" sx={{ fontWeight: 600 }}>
          Rosters
        </Typography>
        <Typography variant="body2" color="text.secondary">
          {assignedUsers.length} / 6 assigned
        </Typography>
      </Box>
      <Divider sx={{ mb: 2 }} />

      <QuizzersTable
        externalRows={assignedUsers}
        onAdd={() => setPickerOpen(true)}
        onDelete={handleRemove}
        createLabel="Add Quizzers"
        showSensitiveColumns={showSensitiveColumns}
        showAuditColumns={showAuditColumns}
      />

      <QuizzerPickerDialog
        isOpen={pickerOpen}
        onCancel={() => setPickerOpen(false)}
        onConfirm={handlePickerConfirm}
        maxSelectable={openSlots}
        assignedIds={assignedIds}
      />
    </Box>
  );
};
