import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'

function userLabel(u: UserTS): string {
  return [u.fname, u.mname, u.lname].filter(Boolean).join(' ');
}

export const TeamProfileOverviewPage = ({ teamid }: { teamid: string }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [notFound, setNotFound] = useState(false);
  const [tournament, setTournament] = useState<TournamentTS | undefined>(undefined);
  const [division, setDivision] = useState<DivisionTS | undefined>(undefined);
  const [team, setTeam] = useState<TeamTS | undefined>(undefined);
  const [users, setUsers] = useState<UserTS[]>([]);

  useEffect(() => {
    setIsLoading(true);
    Promise.all([TeamAPI.getById(teamid), UserAPI.get(0, 200)])
      .then(([tm, u]) => {
        setTeam(tm);
        setUsers(u.items);
        return DivisionAPI.getById(tm.did);
      })
      .then(div => {
        setDivision(div);
        return TournamentAPI.getById(div.tid);
      })
      .then(setTournament)
      .catch((err) => {
        console.error('Failed to load team overview:', err);
        setNotFound(true);
      })
      .finally(() => setIsLoading(false));
  }, [teamid]);

  if (notFound) return <div>Team not found.</div>;
  if (isLoading || !team || !tournament || !division) return <div>Loading...</div>;

  const coachUser = users.find(u => u.id === team.coachid);
  const coachName = coachUser ? userLabel(coachUser) : team.coachid;

  return (
    <Stack spacing={3}>
      <Breadcrumbs aria-label="breadcrumb">
        <Link color="inherit" to="/">Home</Link>
        <Link color="inherit" to={`/tournament/${tournament.tid}/divisions`}>{tournament.tname}</Link>
        <Link color="inherit" to={`/division/${division.did}/overview`}>{division.dname}</Link>
        <Typography color="text.primary">{team.name}</Typography>
      </Breadcrumbs>

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
            <Typography variant="body2" color="text.secondary">Tournament</Typography>
            <Typography variant="body1">
              <Link
                to={`/tournament/${tournament.tid}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {tournament.tname}
              </Link>
            </Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Division</Typography>
            <Typography variant="body1">
              <Link
                to={`/division/${division.did}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {division.dname}
              </Link>
            </Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Coach</Typography>
            <Typography variant="body1">
              {coachUser ? (
                <Link
                  to={`/user/${coachUser.id}/overview`}
                  style={{ color: '#2563eb', textDecoration: 'none' }}
                  onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                  onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
                >
                  {coachName}
                </Link>
              ) : coachName}
            </Typography>
          </Grid>
        </Grid>
      </Box>
    </Stack>
  );
};
