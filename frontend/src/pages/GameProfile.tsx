import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Breadcrumbs from '@mui/material/Breadcrumbs'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { GameAPI, type GameTS } from '../features/GameAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { GameProfileOverviewPage } from './GameProfileOverviewPage'
import { useAuth } from '../hooks/useAuth'

export const GameProfile = (props: { childRoute?: string }) => {
  const { gid } = useParams()
  if (!gid) return <></>

  const { session } = useAuth()
  const [game, setGame] = useState<GameTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    GameAPI.getById(gid)
      .then(g => {
        setGame(g)
        return Promise.all([
          TournamentAPI.getById(g.tournamentid),
          DivisionAPI.getById(g.divisionid),
        ])
      })
      .then(([t, d]) => {
        setTournament(t)
        setDivision(d)
      })
      .catch(() => setNotFound(true))
  }, [gid])

  if (notFound) return <Navigate to="/404" replace />
  if (!game || !tournament || !division) return <div>Loading Game…</div>

  const isOwnerOrSuperUser =
    (session?.hasRole('super_user') ?? false) ||
    (session?.userId === tournament.owner_id)

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/game/${gid}/overview` },
  ]

  return (
    <ProfileLayout title={<>Game:<br />{gid}</>} navItems={navItems}>
      <Stack spacing={3}>

        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Link color="inherit" to={`/tournament/${tournament.tid}/overview`}>{tournament.tname}</Link>
          <Link color="inherit" to={`/division/${division.did}/overview`}>{division.dname}</Link>
          <Typography color="text.primary">Game</Typography>
        </Breadcrumbs>

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <GameProfileOverviewPage game={game} tournament={tournament} onUpdated={setGame} canEdit={isOwnerOrSuperUser} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
