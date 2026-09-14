import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { GameAPI, type GameTS } from '../features/GameAPI'
import { type TournamentTS } from '../features/TournamentAPI'
import { type DivisionTS } from '../features/DivisionAPI'
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
    // The game's tournament and division are derived server-side from its pool bracket.
    Promise.all([GameAPI.getById(gid), GameAPI.getContext(gid)])
      .then(([g, ctx]) => {
        setGame(g)
        setTournament(ctx.tournament)
        setDivision(ctx.division)
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

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
          { name: 'Game' },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <GameProfileOverviewPage game={game} tournament={tournament} onUpdated={setGame} canEdit={isOwnerOrSuperUser} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
