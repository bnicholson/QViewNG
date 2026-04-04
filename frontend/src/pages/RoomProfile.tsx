import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Breadcrumbs from '@mui/material/Breadcrumbs'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { RoomProfileOverviewPage } from './RoomProfileOverviewPage'
import GamesTable from '../components/GamesTable'

export const RoomProfile = (props: { childRoute?: string }) => {
  const { roomid } = useParams()
  if (!roomid) return <></>

  const [room, setRoom] = useState<RoomTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    RoomAPI.getById(roomid)
      .then(r => {
        setRoom(r)
        return TournamentAPI.getById(r.tid)
      })
      .then(setTournament)
      .catch(() => setNotFound(true))
  }, [roomid])

  if (notFound) return <Navigate to="/404" replace />
  if (!room || !tournament) return <div>Loading Room…</div>

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/room/${roomid}/overview` },
    { kind: 'route' as const, label: 'Games',    to: `/room/${roomid}/games`    },
  ]

  return (
    <ProfileLayout title={<>Room:<br />{room.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Link color="inherit" to={`/tournament/${tournament.tid}/overview`}>{tournament.tname}</Link>
          <Typography color="text.primary">{room.name}</Typography>
        </Breadcrumbs>

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <RoomProfileOverviewPage room={room} tournament={tournament} onUpdated={setRoom} />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} roomid={roomid} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
