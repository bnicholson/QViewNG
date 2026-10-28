import { useState, useEffect } from 'react'
import { PERMISSION } from '../auth/permissions'
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
import BuildingsTable from '../components/BuildingsTable'
import RoundsTable from '../components/RoundsTable'
import PoolBracketsTable from '../components/PoolBracketsTable'
import PoolBracketGroupsTable from '../components/PoolBracketGroupsTable'
import TeamsTable from '../components/TeamsTable'
import AdminsTable from '../components/AdminsTable'
import QuizzersTable from '../components/QuizzersTable'
import GamesTable from '../components/GamesTable'
import RoomMonitorTable from '../components/RoomMonitorTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import ProfileLayout from '../components/ProfileLayout'
import { TournamentSchedule } from '../components/TournamentSchedule'
import { TournamentOverviewPage } from './TournamentOverviewPage'
import { TournamentRegisterPage } from './TournamentRegisterPage'
import { TournamentGearPage } from './TournamentGearPage'
import TournamentGroupsTable from '../components/TournamentGroupsTable'
import StatsGroupsPanel from '../components/StatsGroupsPanel'
import { LiveStats } from '../containers/LiveStats'
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
    // Unrestricted-access bypass
    if (session.isSystemAdministrator) {
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
    canViewAdmins === true && (session?.can(permission) ?? false);

  // Mirrors the API's read gate: a signed-out visitor keeps the public read access they have
  // always had, while a signed-in user must hold the entity's `:read` permission.
  const canRead = (permission: string): boolean =>
    !session || session.can(permission);

  const isOwnerOrSystemAdmin =
    (session?.isSystemAdministrator ?? false) ||
    (session?.userId === tournament?.owner_id);

  // Pool/Bracket Groups, Pools and Brackets are infrastructure that scheduling creates and manages
  // on the user's behalf — there is nothing there for the user to manage, and naming them only
  // raises questions. Only super users (who support every tournament) see their sections.
  const showScheduleInternals = session?.isSystemAdministrator ?? false;

  const canViewPairingCode =
    session?.can(PERMISSION.TOURNAMENT_READ_PAIRING_CODE) ?? false;

  // Without this permission the Read/Edit toggle is hidden and only the read-only schedule view
  // is reachable.
  const canEditSchedule =
    session?.can(PERMISSION.TOURNAMENT_UPDATE_SCHEDULE) ?? false;

  // "Created" / "Last Modified" audit columns are visible to holders of `tournament:read_audit`,
  // plus this tournament's owner and its designated admins (via canViewAdmins). Visitors and
  // users with no such grant never see them.
  const canViewAuditColumns =
    canViewAdmins === true ||
    (session?.can(PERMISSION.TOURNAMENT_READ_AUDIT) ?? false);

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  const registrationIsOpen = tournament ? isRegistrationOpen(tournament) : false;

  // Which registration types the tournament offers (each gates its tab). Default true if unset.
  const useTeamRegistration = tournament?.use_team_registration ?? true;
  const useGearRegistration = tournament?.use_gear_registration ?? true;
  const useVolunteerRegistration = tournament?.use_volunteer_registration ?? true;
  const anyRegistrationEnabled = useTeamRegistration || useGearRegistration || useVolunteerRegistration;
  // Where the Registration section should land: the first enabled type (Team → Gear → Volunteer),
  // or the Overview page when no registration type is being used.
  const firstEnabledRegisterPath =
    useTeamRegistration ? `/tournament/${tid}/register/team`
    : useGearRegistration ? `/tournament/${tid}/register/gear`
    : useVolunteerRegistration ? `/tournament/${tid}/register/as-volunteer`
    : `/tournament/${tid}/overview`;

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

  // Each item names the read permission for the domain entity it lists, so a section disappears
  // from the sidebar when the user cannot read that entity. `visible` carries the extra
  // ownership-based conditions that a permission cannot express.
  const allNavItems: Array<{ kind: 'route'; label: string; to: string; requiredPermission?: string; visible?: boolean }> = [
    { kind: 'route', label: 'Overview',     to: `/tournament/${tid}/overview`,     requiredPermission: PERMISSION.TOURNAMENT_READ },
    { kind: 'route', label: 'Registration', to: `/tournament/${tid}/register`,     requiredPermission: PERMISSION.TOURNAMENT_READ, visible: registrationIsOpen && anyRegistrationEnabled },
    { kind: 'route', label: 'Schedule',     to: `/tournament/${tid}/schedule`,     requiredPermission: PERMISSION.TOURNAMENT_READ },
    { kind: 'route', label: 'LIVE Stats',   to: `/tournament/${tid}/live-stats`,   requiredPermission: PERMISSION.ROOM_READ },
    { kind: 'route', label: 'Divisions',    to: `/tournament/${tid}/divisions`,    requiredPermission: PERMISSION.DIVISION_READ },
    { kind: 'route', label: 'Pool/Bracket Groups', to: `/tournament/${tid}/pool-bracket-groups`, requiredPermission: PERMISSION.POOLBRACKETGROUP_READ, visible: showScheduleInternals },
    { kind: 'route', label: 'Pools',        to: `/tournament/${tid}/pools`,        requiredPermission: PERMISSION.POOLBRACKET_READ, visible: showScheduleInternals },
    { kind: 'route', label: 'Brackets',     to: `/tournament/${tid}/brackets`,     requiredPermission: PERMISSION.POOLBRACKET_READ, visible: showScheduleInternals },
    { kind: 'route', label: 'Buildings',    to: `/tournament/${tid}/buildings`,    requiredPermission: PERMISSION.ROOMGROUP_READ },
    { kind: 'route', label: 'Rooms',        to: `/tournament/${tid}/rooms`,        requiredPermission: PERMISSION.ROOM_READ },
    { kind: 'route', label: 'Teams',        to: `/tournament/${tid}/teams`,        requiredPermission: PERMISSION.TEAM_READ },
    { kind: 'route', label: 'Quizzers',     to: `/tournament/${tid}/quizzers`,     requiredPermission: PERMISSION.USER_READ },
    { kind: 'route', label: 'Rounds',       to: `/tournament/${tid}/rounds`,       requiredPermission: PERMISSION.ROUND_READ },
    { kind: 'route', label: 'Games',        to: `/tournament/${tid}/games`,        requiredPermission: PERMISSION.GAME_READ },
    { kind: 'route', label: 'Gear',         to: `/tournament/${tid}/gear`,         requiredPermission: PERMISSION.EQUIPMENT_READ,       visible: canViewAdmins === true },
    { kind: 'route', label: 'Admins',       to: `/tournament/${tid}/admins`,       requiredPermission: PERMISSION.TOURNAMENTADMIN_READ, visible: canViewAdmins === true },
    { kind: 'route', label: 'Tournament Groups', to: `/tournament/${tid}/tournament-groups`, requiredPermission: PERMISSION.TOURNAMENTGROUP_READ },
    { kind: 'route', label: 'Server Monitor', to: `/tournament/${tid}/room-monitor`, requiredPermission: PERMISSION.ROOMMONITOR_READ, visible: isOwnerOrSystemAdmin || canViewAdmins === true },
    { kind: 'route', label: 'Server Stats', to: `/tournament/${tid}/stats-groups`, requiredPermission: PERMISSION.STATSGROUP_READ,  visible: isOwnerOrSystemAdmin || canViewAdmins === true },
  ]

  const navItems = allNavItems
    .filter(({ requiredPermission, visible }) =>
      (visible !== false) &&
      (!requiredPermission || canRead(requiredPermission))
    )
    .map(({ requiredPermission: _rp, visible: _v, ...item }) => item)

  return (
    <ProfileLayout entityType="Tournament" entityName={tournament!.tname} navItems={navItems}>
      <Stack spacing={3}>

        {/* ── Breadcrumb ── */}
        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament!.tname },
        ]} />

        {/* ── Section content ── */}
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'register/team'     && (
            useTeamRegistration
              ? (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="team" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)
              : <Navigate to={firstEnabledRegisterPath} replace />
          )}
          {props.childRoute === 'register/gear'     && (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="gear" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)}
          {props.childRoute === 'register/volunteer'&& (registrationIsOpen ? <TournamentRegisterPage tid={String(tournament?.tid)} tname={tournament!.tname} initialTab="as-volunteer" useTeamRegistration={useTeamRegistration} useGearRegistration={useGearRegistration} useVolunteerRegistration={useVolunteerRegistration} /> : registrationClosedNotice)}
          {props.childRoute === 'overview'          && <TournamentOverviewPage tournament={tournament!} isTournamentUpdate={canCreate(PERMISSION.TOURNAMENT_UPDATE)} canViewPairingCodeAndVisibility={canViewPairingCode} onEdit={() => setTournamentEditorIsOpen(true)} />}
          {props.childRoute === 'schedule'          && <TournamentSchedule tid={String(tournament?.tid)} canEdit={canEditSchedule} />}
          {/* LiveStats reads the tournament id (and its tab and division) from the URL, so it needs no props. */}
          {props.childRoute === 'live-stats'        && <LiveStats />}
          {props.childRoute === 'divisions'         && <DivisionsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.DIVISION_CREATE)} showDeleteButton={canCreate(PERMISSION.DIVISION_DELETE)} showSensitiveColumns={isOwnerOrSystemAdmin} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'pool-bracket-groups' && (showScheduleInternals
            ? <PoolBracketGroupsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.POOLBRACKETGROUP_CREATE)} showEditButton={canCreate(PERMISSION.POOLBRACKETGROUP_UPDATE)} showDeleteButton={canCreate(PERMISSION.POOLBRACKETGROUP_DELETE)} showAuditColumns={canViewAuditColumns}/>
            : <Navigate to={`/tournament/${tid}/overview`} replace />)}
          {props.childRoute === 'pools'             && (showScheduleInternals
            ? <PoolBracketsTable tid={String(tournament?.tid)} type="pool" entityLabel="Pool" title="Pools" showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)} showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)} showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)} showAuditColumns={canViewAuditColumns}/>
            : <Navigate to={`/tournament/${tid}/overview`} replace />)}
          {props.childRoute === 'brackets'          && (showScheduleInternals
            ? <PoolBracketsTable tid={String(tournament?.tid)} type="bracket" entityLabel="Bracket" title="Brackets" showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)} showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)} showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)} showAuditColumns={canViewAuditColumns}/>
            : <Navigate to={`/tournament/${tid}/overview`} replace />)}
          {props.childRoute === 'buildings'         && <BuildingsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.ROOMGROUP_CREATE)} showEditButton={canCreate(PERMISSION.ROOMGROUP_UPDATE)} showDeleteButton={canCreate(PERMISSION.ROOMGROUP_DELETE)} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'rooms'             && <RoomsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.ROOM_CREATE)} showDeleteButton={canCreate(PERMISSION.ROOM_DELETE)} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'rounds'            && <RoundsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.ROUND_CREATE)} showDeleteButton={canCreate(PERMISSION.ROUND_DELETE)} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'teams'             && <TeamsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.TEAM_CREATE)} showDeleteButton={canCreate(PERMISSION.TEAM_DELETE)} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'quizzers'          && <QuizzersTable tid={String(tournament?.tid)} showSensitiveColumns={isOwnerOrSystemAdmin} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'games'             && <GamesTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.GAME_CREATE)} showDeleteButton={canCreate(PERMISSION.GAME_DELETE)} showAuditColumns={canViewAuditColumns}/>}
          {props.childRoute === 'gear'              && canViewAdmins === true && <TournamentGearPage tid={String(tournament?.tid)} />}
          {props.childRoute === 'admins'            && canViewAdmins === true && <AdminsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.TOURNAMENTADMIN_CREATE)} showDeleteButton={canCreate(PERMISSION.TOURNAMENTADMIN_DELETE)}/>}
          {props.childRoute === 'tournament-groups'  && <TournamentGroupsTable tid={String(tournament?.tid)} showCreateButton={canCreate(PERMISSION.TOURNAMENTGROUP_CREATE)} showDeleteButton={canCreate(PERMISSION.TOURNAMENTGROUP_DELETE)} canEdit={isOwnerOrSystemAdmin} showAuditColumns={canViewAuditColumns} />}
          {props.childRoute === 'stats-groups'      && (isOwnerOrSystemAdmin || canViewAdmins) && <StatsGroupsPanel tid={String(tournament?.tid)} tournamentName={tournament?.tname} canImport={canCreate(PERMISSION.GAME_UPDATE)} canEditTieBreakers={canCreate(PERMISSION.STATSGROUP_UPDATE)} canSetQuestionTypes={canCreate(PERMISSION.ROUND_UPLOAD_QUESTION_TYPES)} />}
          {props.childRoute === 'room-monitor'      && (isOwnerOrSystemAdmin || canViewAdmins) && <RoomMonitorTable tid={String(tournament?.tid)} canClear={canCreate(PERMISSION.GAME_UPDATE)}/>}
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
