import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Stack from "@mui/material/Stack"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
import { Link } from 'react-router-dom'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { makeCancellable } from '../features/makeCancellable'
import DivisionsTable from '../components/DivisionsTable'
import RoomsTable from '../components/RoomsTable'
import RoundsTable from '../components/RoundsTable'
import TeamsTable from '../components/TeamsTable'
import AdminsTable from '../components/AdminsTable'
import QuizzersTable from '../components/QuizzersTable'
import GamesTable from '../components/GamesTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import ProfileLayout from '../components/ProfileLayout'
import { TournamentOverviewPage } from './TournamentOverviewPage'

export const TournamentProfile = (props: { childRoute?: string }) => {

  const isUserAdmin = true;

  const { tid } = useParams();
  if (tid === undefined) return (<></>)

  const [isLoading, setIsLoading] = useState<boolean>(false)
  const stillLoading = () => isLoading || tournament == null || tournament == undefined
  const [notFound, setNotFound] = useState<boolean>(false)
  const [tournament, setTournament] = useState<TournamentTS>()
  const [tournamentEditorIsOpen, setTournamentEditorIsOpen] = useState(false);

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

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  const navItems = [
    { kind: 'route' as const, label: 'Overview',     to: `/tournament/${tid}/overview`     },
    { kind: 'route' as const, label: 'Divisions',    to: `/tournament/${tid}/divisions`    },
    { kind: 'route' as const, label: 'Rooms',        to: `/tournament/${tid}/rooms`        },
    { kind: 'route' as const, label: 'Teams',        to: `/tournament/${tid}/teams`        },
    { kind: 'route' as const, label: 'Rounds',       to: `/tournament/${tid}/rounds`       },
    { kind: 'route' as const, label: 'Quizzers',     to: `/tournament/${tid}/quizzers`     },
    { kind: 'route' as const, label: 'Games',        to: `/tournament/${tid}/games`        },
    { kind: 'route' as const, label: 'Admins',       to: `/tournament/${tid}/admins`       },
    { kind: 'route' as const, label: 'Stats Groups', to: `/tournament/${tid}/stats-groups` },
  ]

  return (
    <ProfileLayout title={tournament!.tname} navItems={navItems}>
      <Stack spacing={3}>

        {/* ── Breadcrumb ── */}
        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Typography color="text.primary">{tournament?.tname}</Typography>
        </Breadcrumbs>

        {/* ── Section content ── */}
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview'     && <TournamentOverviewPage tournament={tournament!} isUserAdmin={isUserAdmin} onEdit={() => setTournamentEditorIsOpen(true)} />}
          {props.childRoute === 'divisions'    && <DivisionsTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'rooms'        && <RoomsTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'rounds'       && <RoundsTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'teams'        && <TeamsTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'quizzers'     && <QuizzersTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'games'        && <GamesTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'admins'       && <AdminsTable tid={String(tournament?.tid)}/>}
          {props.childRoute === 'stats-groups' && <Typography color="text.secondary">Stats Groups coming soon.</Typography>}
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
