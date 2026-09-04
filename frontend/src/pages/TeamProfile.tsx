import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import { TeamProfileOverviewPage } from './TeamProfileOverviewPage'
import { TeamProfileQuizzersPage } from './TeamProfileQuizzersPage'

type ChildRoute = 'overview' | 'quizzers'

export const TeamProfile = (props: { childRoute?: ChildRoute }) => {
  const { teamid } = useParams<{ teamid: string }>();
  const [team, setTeam] = useState<TeamTS | null>(null);
  // Division and tournament are resolved via team → division → tournament so this profile can
  // render the breadcrumb (once, here) and honour the same table permissions as elsewhere.
  const [division, setDivision] = useState<DivisionTS | null>(null);
  const [tournament, setTournament] = useState<TournamentTS | null>(null);

  useEffect(() => {
    if (!teamid) return;
    TeamAPI.getById(teamid)
      .then(t => {
        setTeam(t);
        return DivisionAPI.getById(t.did);
      })
      .then(div => {
        setDivision(div);
        return TournamentAPI.getById(div.tid);
      })
      .then(setTournament)
      .catch(() => {});
  }, [teamid]);

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id);

  if (!teamid) return <Navigate to="/404" replace />;

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview', to: `/team/${teamid}/overview` },
    { kind: 'route', label: 'Quizzers', to: `/team/${teamid}/quizzers` },
  ];

  return (
    <ProfileLayout title={<>Team:<br />{team?.name ?? ''}</>} navItems={navItems}>
      <Stack spacing={3}>
        {team && division && tournament && (
          <ProfileBreadcrumbs crumbs={[
            { name: 'Home', to: '/' },
            { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
            { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
            { label: 'Team', name: team.name },
          ]} />
        )}

        {props.childRoute === 'overview' && <TeamProfileOverviewPage teamid={teamid} />}
        {props.childRoute === 'quizzers' && (
          <TeamProfileQuizzersPage teamid={teamid}
            showSensitiveColumns={access.isOwnerOrSuperUser}
            showAuditColumns={access.canViewAuditColumns} />
        )}
      </Stack>
    </ProfileLayout>
  );
};
