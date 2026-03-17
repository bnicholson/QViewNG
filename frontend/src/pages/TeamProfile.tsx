import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import QuizzersTable from '../components/QuizzersTable'
import { QuizzerPickerDialog } from '../components/QuizzerPickerDialog'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
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

function userLabel(u: UserTS): string {
  return [u.fname, u.mname, u.lname].filter(Boolean).join(' ');   // + ` (@${u.username})`;
}

export const TeamProfile = () => {
  const { tid, teamid } = useParams();

  const [isLoading, setIsLoading] = useState(false);
  const [notFound, setNotFound] = useState(false);
  const [tournament, setTournament] = useState<TournamentTS | undefined>(undefined);
  const [division, setDivision] = useState<DivisionTS | undefined>(undefined);
  const [team, setTeam] = useState<TeamTS | undefined>(undefined);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [slots, setSlots] = useState<(string | null)[]>([null, null, null, null, null, null]);
  const [pickerOpen, setPickerOpen] = useState(false);

  useEffect(() => {
    if (!tid || !teamid) return;
    setIsLoading(true);
    Promise.all([
      TournamentAPI.getById(tid),
      TeamAPI.getById(teamid),
      UserAPI.get(0, 200),
    ])
      .then(([t, tm, u]) => {
        setTournament(t);
        setTeam(tm);
        setUsers(u);
        setSlots(slotsFromTeam(tm));
        return DivisionAPI.getById(tm.did);
      })
      .then(setDivision)
      .catch((err) => {
        console.error('Failed to load team profile:', err);
        setNotFound(true);
      })
      .finally(() => setIsLoading(false));
  }, [tid, teamid]);

  const handlePickerConfirm = (selected: UserTS[]) => {
    setPickerOpen(false);
    setSlots(prev => {
      const next = [...prev];
      let si = 0;
      for (let i = 0; i < next.length && si < selected.length; i++) {
        if (!next[i]) next[i] = selected[si++].id;
      }
      return next;
    });
  };

  const handleRemoveQuizzer = async (user: UserTS): Promise<void> => {
    const newSlots = slots.map(s => s === user.id ? null : s);
    const changeset: TeamChangeset = {};
    SLOT_FIELDS.forEach((f, i) => { changeset[f] = newSlots[i] || null; });
    await TeamAPI.update(teamid!, changeset);
    setSlots(newSlots);
  };

  if (!tid || !teamid) return <></>;
  if (notFound) return <Navigate to="/404" replace />;
  if (isLoading || !team || !tournament || !division) return <div>Loading team...</div>;

  const coachUser = users.find(u => u.id === team.coachid);
  const coachName = coachUser ? userLabel(coachUser) : team.coachid;

  const assignedUsers = slots
    .filter(Boolean)
    .map(id => users.find(u => u.id === id))
    .filter((u): u is UserTS => u !== undefined);

  const assignedIds = slots.filter(Boolean) as string[];
  const openSlots = slots.filter(s => !s).length;

  return (
    <Stack spacing={3}>

      {/* ── Breadcrumb ── */}
      <Breadcrumbs aria-label="breadcrumb">
        <Link color="inherit" to="/">Home</Link>
        <Link color="inherit" to={`/tournament/${tid}/divisions`}>{tournament.tname}</Link>
        <Typography color="text.secondary">{division.dname}</Typography>
        <Typography color="text.primary">{team.name}</Typography>
      </Breadcrumbs>

      {/* ── General Info ── */}
      <Box>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600, mb: 2 }}>
          {team.name}
        </Typography>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Team: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />
        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Division</Typography>
            <Typography variant="body1">{division.dname}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Coach</Typography>
            <Typography variant="body1">{coachName}</Typography>
          </Grid>
        </Grid>
      </Box>

      {/* ── Quizzer Roster ── */}
      <Box>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 1 }}>
          <Typography variant="subtitle1" sx={{ fontWeight: 600 }}>
            Quizzer Roster
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {assignedUsers.length} / 6 assigned
          </Typography>
        </Box>
        <Divider sx={{ mb: 2 }} />

        <QuizzersTable
          externalRows={assignedUsers}
          onAdd={() => setPickerOpen(true)}
          onDelete={handleRemoveQuizzer}
          createLabel="Add Quizzers"
        />

      </Box>

      <QuizzerPickerDialog
        isOpen={pickerOpen}
        onCancel={() => setPickerOpen(false)}
        onConfirm={handlePickerConfirm}
        maxSelectable={openSlots}
        assignedIds={assignedIds}
      />

    </Stack>
  );
};
