import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import Divider from '@mui/material/Divider'
import Typography from '@mui/material/Typography'
import TeamQuizzersTable from '../components/TeamQuizzersTable'
import { QuizzerPickerDialog } from '../components/QuizzerPickerDialog'
import { TeamAPI, type TeamTS, type TeamChangeset } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'

const SLOT_FIELDS = [
  'quizzer_one_id',
  'quizzer_two_id',
  'quizzer_three_id',
  'quizzer_four_id',
  'quizzer_five_id',
  'quizzer_six_id',
] as const;

function slotsFromTeam(team: TeamTS): (string | null)[] {
  return SLOT_FIELDS.map(f => team[f] ?? null);
}

export const TeamProfileQuizzersPage = ({ teamid }: { teamid: string }) => {
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
      const updatedTeam = await TeamAPI.update(teamid, changeset);
      setTeam(updatedTeam);
      setSlots(slotsFromTeam(updatedTeam));
    } catch (err) {
      console.error('Failed to save quizzer assignments:', err);
    }
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

      <TeamQuizzersTable
        team={team}
        teamId={teamid}
        assignedUsers={assignedUsers}
        onAdd={() => setPickerOpen(true)}
        onRemoved={(updatedTeam) => {
          setTeam(updatedTeam);
          setSlots(slotsFromTeam(updatedTeam));
        }}
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
