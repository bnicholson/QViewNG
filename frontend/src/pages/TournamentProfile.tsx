import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Stack from "@mui/material/Stack"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
import { Link } from 'react-router-dom'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
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
import { useAuth } from '../hooks/useAuth'

export const TournamentProfile = (props: { childRoute?: string }) => {

  const { session } = useAuth();

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
    const cancellable = makeCancellable(TournamentAPI.getById(tid));
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
  }, [tid])

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

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  const allNavItems: Array<{ kind: 'route'; label: string; to: string; requiredPermission?: string; visible?: boolean }> = [
    { kind: 'route', label: 'Overview',     to: `/tournament/${tid}/overview`     },
    { kind: 'route', label: 'Divisions',    to: `/tournament/${tid}/divisions`    },
    { kind: 'route', label: 'Rooms',        to: `/tournament/${tid}/rooms`        },
    { kind: 'route', label: 'Room Monitor', to: `/tournament/${tid}/room-monitor`, requiredPermission: 'roommonitor:read' },
    { kind: 'route', label: 'Teams',        to: `/tournament/${tid}/teams`        },
    { kind: 'route', label: 'Quizzers',     to: `/tournament/${tid}/quizzers`     },
    { kind: 'route', label: 'Rounds',       to: `/tournament/${tid}/rounds`       },
    { kind: 'route', label: 'Games',        to: `/tournament/${tid}/games`        },
    { kind: 'route', label: 'Admins',       to: `/tournament/${tid}/admins`,       visible: canViewAdmins === true },
    { kind: 'route', label: 'Stats Groups', to: `/tournament/${tid}/stats-groups` },
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
        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Typography color="text.primary">{tournament?.tname}</Typography>
        </Breadcrumbs>

        {/* ── Section content ── */}
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview'     && <TournamentOverviewPage tournament={tournament!} isTournamentUpdate={canCreate('tournament:update')} onEdit={() => setTournamentEditorIsOpen(true)} />}
          {props.childRoute === 'divisions'    && <DivisionsTable tid={String(tournament?.tid)} showCreateButton={canCreate('division:create')} showDeleteButton={canCreate('division:delete')}/>}
          {props.childRoute === 'rooms'        && <RoomsTable tid={String(tournament?.tid)} showCreateButton={canCreate('room:create')} showDeleteButton={canCreate('room:delete')}/>}
          {props.childRoute === 'rounds'       && <RoundsTable tid={String(tournament?.tid)} showCreateButton={canCreate('round:create')} showDeleteButton={canCreate('round:delete')}/>}
          {props.childRoute === 'teams'        && <TeamsTable tid={String(tournament?.tid)} showCreateButton={canCreate('team:create')} showDeleteButton={canCreate('team:delete')}/>}
          {props.childRoute === 'quizzers'     && <QuizzersTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'games'        && <GamesTable tid={String(tournament?.tid)} showCreateButton={canCreate('game:create')} showDeleteButton={canCreate('game:delete')}/>}
          {props.childRoute === 'admins'       && canViewAdmins === true && <AdminsTable tid={String(tournament?.tid)} showCreateButton={canViewAdmins === true} showDeleteButton={canViewAdmins === true}/>}
          {props.childRoute === 'stats-groups'  && <Typography color="text.secondary">Stats Groups coming soon.</Typography>}
          {props.childRoute === 'room-monitor'  && <RoomMonitorTable tid={String(tournament?.tid)}/>}
        </Box>

        <TournamentEditorDialog
          initialTournament={tournament}
          isOpen={tournamentEditorIsOpen}
          onCancel={() => setTournamentEditorIsOpen(false)}
          onSave={t => { setTournament(t); setTournamentEditorIsOpen(false); }}
        />

      </Stack>
    </ProfileLayout>
  )
}
