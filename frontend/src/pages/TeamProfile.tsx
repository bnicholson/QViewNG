import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { TeamProfileOverviewPage } from './TeamProfileOverviewPage'
import { TeamProfileQuizzersPage } from './TeamProfileQuizzersPage'

type ChildRoute = 'overview' | 'quizzers'

export const TeamProfile = (props: { childRoute?: ChildRoute }) => {
  const { teamid } = useParams<{ teamid: string }>();
  const [team, setTeam] = useState<TeamTS | null>(null);

  useEffect(() => {
    if (!teamid) return;
    TeamAPI.getById(teamid)
      .then(setTeam)
      .catch(() => {});
  }, [teamid]);

  if (!teamid) return <Navigate to="/404" replace />;

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview', to: `/team/${teamid}/overview` },
    { kind: 'route', label: 'Quizzers', to: `/team/${teamid}/quizzers` },
  ];

  return (
    <ProfileLayout title={<>Team:<br />{team?.name ?? ''}</>} navItems={navItems}>
      {props.childRoute === 'overview' && <TeamProfileOverviewPage teamid={teamid} />}
      {props.childRoute === 'quizzers' && <TeamProfileQuizzersPage teamid={teamid} />}
    </ProfileLayout>
  );
};
