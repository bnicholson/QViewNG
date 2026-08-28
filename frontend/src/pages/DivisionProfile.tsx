import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Breadcrumbs from '@mui/material/Breadcrumbs'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { AdminAPI } from '../features/AdminAPI'
import { useAuth } from '../hooks/useAuth'
import TeamsTable from '../components/TeamsTable'
import RoundsTable from '../components/RoundsTable'
import GamesTable from '../components/GamesTable'
import { DivisionProfileOverviewPage } from './DivisionProfileOverviewPage'
import { DivisionProfileQuizzersPage } from './DivisionProfileQuizzersPage'

export const DivisionProfile = (props: { childRoute?: string }) => {
  const { did } = useParams()
  if (!did) return <></>

  const { session } = useAuth()
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)
  // Whether the current user is one of this tournament's admins (null while loading).
  const [canViewAdmins, setCanViewAdmins] = useState<boolean | null>(null)

  useEffect(() => {
    DivisionAPI.getById(did)
      .then(div => {
        setDivision(div)
        return TournamentAPI.getById(div.tid)
      })
      .then(setTournament)
      .catch(() => setNotFound(true))
  }, [did])

  // Resolve tournament-admin membership for the "Stats Groups" gate.
  useEffect(() => {
    if (!tournament || !session) { setCanViewAdmins(false); return }
    if (session.hasRole('super_user') || session.userId === tournament.owner_id) {
      setCanViewAdmins(true); return
    }
    setCanViewAdmins(null)
    AdminAPI.getByTournament(String(tournament.tid), 0, 500)
      .then(admins => setCanViewAdmins(admins.some(a => a.id === session.userId)))
      .catch(() => setCanViewAdmins(false))
  }, [tournament?.tid, session?.userId])

  if (notFound) return <Navigate to="/404" replace />
  if (!division || !tournament) return <div>Loading Division…</div>

  const isOwnerOrSuperUser =
    (session?.hasRole('super_user') ?? false) ||
    (session?.userId === tournament.owner_id)

  // Only super users, the tournament owner, and tournament admins may view Stats Groups.
  const canViewStatsGroups = isOwnerOrSuperUser || canViewAdmins === true

  const navItems = [
    { kind: 'route' as const, label: 'Overview',     to: `/division/${did}/overview`     },
    { kind: 'route' as const, label: 'Teams',        to: `/division/${did}/teams`        },
    { kind: 'route' as const, label: 'Quizzers',     to: `/division/${did}/quizzers`     },
    { kind: 'route' as const, label: 'Rounds',       to: `/division/${did}/rounds`       },
    { kind: 'route' as const, label: 'Games',        to: `/division/${did}/games`        },
    ...(canViewStatsGroups
      ? [{ kind: 'route' as const, label: 'Stats Groups', to: `/division/${did}/stats-groups` }]
      : []),
  ]

  return (
    <ProfileLayout title={<>Division:<br />{division.dname}</>} navItems={navItems}>
      <Stack spacing={3}>

        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Link color="inherit" to={`/tournament/${tournament.tid}/overview`}>{tournament.tname}</Link>
          <Typography color="text.primary">{division.dname}</Typography>
        </Breadcrumbs>

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <DivisionProfileOverviewPage division={division} tournament={tournament} onUpdated={setDivision} canEdit={isOwnerOrSuperUser} />
          )}
          {props.childRoute === 'teams' && (
            <TeamsTable tid={tournament.tid} did={did} />
          )}
          {props.childRoute === 'quizzers' && (
            <DivisionProfileQuizzersPage did={did} />
          )}
          {props.childRoute === 'rounds' && (
            <RoundsTable tid={tournament.tid} did={did} />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} did={did} />
          )}
          {props.childRoute === 'stats-groups' && canViewStatsGroups && (
            <Typography color="text.secondary">
              Stats Groups coming soon. For now, navigate to{' '}
              <Link to={`/tournament/${tournament.tid}/stats-groups`} style={{ color: '#2563eb' }}>
                Tournament &gt; Server: Stats
              </Link>
              {' '}and set Division = &quot;{division.dname}&quot;.
            </Typography>
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
