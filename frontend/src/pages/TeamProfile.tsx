import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { DivisionAPI } from '../features/DivisionAPI'
import { TournamentAPI } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import { TeamProfileOverviewPage } from './TeamProfileOverviewPage'
import { TeamProfileQuizzersPage } from './TeamProfileQuizzersPage'

type ChildRoute = 'overview' | 'quizzers'

export const TeamProfile = (props: { childRoute?: ChildRoute }) => {
  const { teamid } = useParams<{ teamid: string }>();
  const [team, setTeam] = useState<TeamTS | null>(null);
  // Tournament context (owner id) resolved via team → division → tournament, so the
  // shared tables here honour the same permissions as on the Tournament profile.
  const [tid, setTid] = useState<string | undefined>(undefined);
  const [ownerId, setOwnerId] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (!teamid) return;
    TeamAPI.getById(teamid)
      .then(t => {
        setTeam(t);
        return DivisionAPI.getById(t.did);
      })
      .then(div => {
        setTid(div.tid);
        return TournamentAPI.getById(div.tid);
      })
      .then(tournament => setOwnerId(tournament.owner_id))
      .catch(() => {});
  }, [teamid]);

  const access = useTournamentAccess(tid, ownerId);

  if (!teamid) return <Navigate to="/404" replace />;

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview', to: `/team/${teamid}/overview` },
    { kind: 'route', label: 'Quizzers', to: `/team/${teamid}/quizzers` },
  ];

  return (
    <ProfileLayout title={<>Team:<br />{team?.name ?? ''}</>} navItems={navItems}>
      {props.childRoute === 'overview' && <TeamProfileOverviewPage teamid={teamid} />}
      {props.childRoute === 'quizzers' && (
        <TeamProfileQuizzersPage teamid={teamid}
          showSensitiveColumns={access.isOwnerOrSuperUser}
          showAuditColumns={access.canViewAuditColumns} />
      )}
    </ProfileLayout>
  );
};
