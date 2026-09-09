import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Stack from "@mui/material/Stack"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import { Link } from 'react-router-dom'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { TournamentAPI, isRegistrationOpen, type TournamentTS } from '../features/TournamentAPI'
import { AdminAPI } from '../features/AdminAPI'
import { makeCancellable } from '../features/makeCancellable'
import DivisionsTable from '../components/DivisionsTable'
import RoomsTable from '../components/RoomsTable'
import RoundsTable from '../components/RoundsTable'
import TeamsTable from '../components/TeamsTable'
import AdminsTable from '../components/AdminsTable'
import QuizzersTable from '../components/QuizzersTable'
import GamesTable from '../components/GamesTable'
import RoomMonitorTable from '../components/RoomMonitorTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import ProfileLayout from '../components/ProfileLayout'
import { TournamentOverviewPage } from './TournamentOverviewPage'
import { TournamentRegisterPage } from './TournamentRegisterPage'
import { TournamentGearPage } from './TournamentGearPage'
import TournamentGroupsTable from '../components/TournamentGroupsTable'
import StatsGroupsPanel from '../components/StatsGroupsPanel'
import { useAuth } from '../hooks/useAuth'

export const TournamentProfile = (props: { childRoute?: string }) => {

  const { session, accessToken } = useAuth();

  const { tid } = useParams();
  if (tid === undefined) return (<></>)

  const [isLoading, setIsLoading] = useState<boolean>(false)
  const stillLoading = () => isLoading || tournament == null || tournament == undefined
  const [notFound, setNotFound] = useState<boolean>(false)
  const [tournament, setTournament] = useState<TournamentTS>()
  const [tournamentEditorIsOpen, setTournamentEditorIsOpen] = useState(false);
  // null = check in-flight, true/false = resolved
  const [canViewAdmins, setCanViewAdmins] = useState<boolean | null>(null);

  useEffect(() => {
    setIsLoading(true)
    const cancellable = makeCancellable(TournamentAPI.getById(tid, accessToken));
    try {
      cancellable.promise
        .then((returnedTournament: TournamentTS) => {
          setTournament(returnedTournament)
          setIsLoading(false)
        })
        .catch((error) => {
          if (error.isCancelled) {
            console.log('Info: The request to get Tournament by ID was cancelled');
          } else {
            console.error('Error: Could not load Tournament:', error);
          }
          setIsLoading(false);
          setNotFound(true)
        })
    } catch (err: any) {
      if (err instanceof Error) {
        console.error(err.message)
        setIsLoading(false);
        setNotFound(true)
      }
    }
    setIsLoading(false)
  }, [tid, accessToken])

  useEffect(() => {
    if (!tournament || !session) {
      setCanViewAdmins(false);
      return;
    }
    // Superuser bypass
    if (session.hasRole('super_user')) {
      setCanViewAdmins(true);
      return;
    }
    // Owner check — no API call needed
    if (session.userId === tournament.owner_id) {
      setCanViewAdmins(true);
      return;
    }
    // Admin check — fetch the admin list and look for the current user
    setCanViewAdmins(null);
    AdminAPI.getByTournament(String(tournament.tid), 0, 500)
      .then(admins => setCanViewAdmins(admins.some(a => a.id === session.userId)))
      .catch(() => setCanViewAdmins(false));
  }, [tournament?.tid, session?.userId])

  // true only when the current user can act as owner or admin for this tournament
  // AND holds the given resource-level permission
  const canCreate = (permission: string): boolean =>
    canViewAdmins === true && (session?.hasPermission(permission) ?? false);

  const isOwnerOrSuperUser =
    (session?.hasRole('super_user') ?? false) ||
    (session?.userId === tournament?.owner_id);

  const canViewPairingCode =
    (session?.hasRole('super_user') ?? false) ||
    (session?.hasRole('tournament_manager') ?? false) ||
    (session?.hasRole('tournament_admin') ?? false);

  // "Created" / "Last Modified" audit columns are visible only to privileged users:
  // superusers, tournament managers, tournament admins (role), plus this tournament's
  // owner and its designated admins (via canViewAdmins). Visitors and regular members
  // (logged in with no special permissions) never see them.
  const canViewAuditColumns =
    canViewAdmins === true ||
    (session?.hasRole('tournament_manager') ?? false) ||
    (session?.hasRole('tournament_admin') ?? false);

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  const registrationIsOpen = tournament ? isRegistrationOpen(tournament) : false;

  // Which registration types the tournament offers (each gates its tab). Default true if unset.
  const useTeamRegistration = tournament?.use_team_registration ?? true;
  const useGearRegistration = tournament?.use_gear_registration ?? true;
  const useVolunteerRegistration = tournament?.use_volunteer_registration ?? true;
  const anyRegistrationEnabled = useTeamRegistration || useGearRegistration || useVolunteerRegistration;

  // Shown when someone navigates directly to a registration URL while the window is closed.
  const registrationWindowText =
    tournament?.registration_open_date && tournament?.registration_close_date
      ? `${tournament.registration_open_date.format('MMM D, YYYY')} – ${tournament.registration_close_date.format('MMM D, YYYY')}`
      : null;
  const registrationClosedNotice = (
    <Alert severity="info">
      <AlertTitle>Registration is currently closed</AlertTitle>
      {registrationWindowText
        ? `Registration for ${tournament!.tname} is open ${registrationWindowText}.`
        : `A registration window has not been set for ${tournament!.tname}.`}
    </Alert>
  );

  const allNavItems: Array<{ kind: 'route'; label: string; to: string; requiredPermission?: string; visible?: boolean }> = [
    { kind: 'route', label: 'Overview',     to: `/tournament/${tid}/overview`     },
    { kind: 'route', label: 'Registration', to: `/tournament/${tid}/register`,      visible: registrationIsOpen && anyRegistrationEnabled },
    { kind: 'route', label: 'Divisions',    to: `/tournament/${tid}/divisions`    },
    { kind: 'route', label: 'Rooms',        to: `/tournament/${tid}/rooms`        },
    { kind: 'route', label: 'Teams',        to: `/tournament/${tid}/teams`        },
    { kind: 'route', label: 'Quizzers',     to: `/tournament/${tid}/quizzers`     },
    { kind: 'route', label: 'Rounds',       to: `/tournament/${tid}/rounds`       },
    { kind: 'route', label: 'Games',        to: `/tournament/${tid}/games`        },
    { kind: 'route', label: 'Gear',         to: `/tournament/${tid}/gear`,          visible: canViewAdmins === true },
    { kind: 'route', label: 'Admins',       to: `/tournament/${tid}/admins`,       visible: canViewAdmins === true },
    { kind: 'route', label: 'Tournament Groups', to: `/tournament/${tid}/tournament-groups` },
    { kind: 'route', label: 'Server Monitor', to: `/tournament/${tid}/room-monitor`, visible: isOwnerOrSuperUser || canViewAdmins === true },
    { kind: 'route', label: 'Server Stats', to: `/tournament/${tid}/stats-groups`, visible: isOwnerOrSuperUser || canViewAdmins === true },
  ]

  const navItems = allNavItems
    .filter(({ requiredPermission, visible }) =>
      (visible !== false) &&
      (!requiredPermission || (session?.hasPermission(requiredPermission) ?? false))
    )
    .map(({ requiredPermission: _rp, visible: _v, ...item }) => item)

  return (
    <ProfileLayout title={<>Tournament:<br />{tournament!.tname}</>} navItems={navItems}>
      <Stack spacing={3}>

        {/* ── Breadcrumb ── */}
        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament!.tname },
        ]} />

        {/* ── Section content ── */}
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'register/team'     && (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="team" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)}
          {props.childRoute === 'register/gear'     && (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="gear" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)}
          {props.childRoute === 'register/volunteer'&& (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="as-volunteer" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)}
          {props.childRoute === 'overview'          && <TournamentOverviewPage tournament={tournament!} isTournamentUpdate={canCreate('tournament:update')} canViewPairingCodeAndVisibility={canViewPairingCode} onEdit={() => setTournamentEditorIsOpen(true)} />}
          {props.childRoute === 'divisions'         && <DivisionsTable tid={String(tournament?.tid)} showCreateButton={canCreate('division:create')} showDeleteButton={canCreate('division:delete')} showSensitiveColumns={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'rooms'             && <RoomsTable tid={String(tournament?.tid)} showCreateButton={canCreate('room:create')} showDeleteButton={canCreate('room:delete')} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'rounds'            && <RoundsTable tid={String(tournament?.tid)} showCreateButton={canCreate('round:create')} showDeleteButton={canCreate('round:delete')} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'teams'             && <TeamsTable tid={String(tournament?.tid)} showCreateButton={canCreate('team:create')} showDeleteButton={canCreate('team:delete')} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'quizzers'          && <QuizzersTable tid={String(tournament?.tid)} showSensitiveColumns={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'games'             && <GamesTable tid={String(tournament?.tid)} showCreateButton={canCreate('game:create')} showDeleteButton={canCreate('game:delete')} showSensitiveColumns={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'gear'              && canViewAdmins === true && <TournamentGearPage tid={String(tournament?.tid)} />}
          {props.childRoute === 'admins'            && canViewAdmins === true && <AdminsTable tid={String(tournament?.tid)} showCreateButton={canViewAdmins === true} showDeleteButton={canViewAdmins === true}/>}
          {props.childRoute === 'tournament-groups'  && <TournamentGroupsTable tid={String(tournament?.tid)} showCreateButton={canCreate('tournamentgroup:create')} showDeleteButton={canCreate('tournamentgroup:delete')} canEdit={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns} />}
          {props.childRoute === 'stats-groups'      && (isOwnerOrSuperUser || canViewAdmins) && <StatsGroupsPanel tid={String(tournament?.tid)} />}
          {props.childRoute === 'room-monitor'      && (isOwnerOrSuperUser || canViewAdmins) && <RoomMonitorTable tid={String(tournament?.tid)}/>}
        </Box>

        <TournamentEditorDialog
          initialTournament={tournament}
          isOpen={tournamentEditorIsOpen}
          canViewPairingCode={canViewPairingCode}
          onCancel={() => setTournamentEditorIsOpen(false)}
          onSave={t => { setTournament(t); setTournamentEditorIsOpen(false); }}
        />

      </Stack>
    </ProfileLayout>
  )
}
