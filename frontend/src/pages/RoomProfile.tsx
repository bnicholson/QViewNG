import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { RoomProfileOverviewPage } from './RoomProfileOverviewPage'
import GamesTable from '../components/GamesTable'
import { useTournamentAccess } from '../hooks/useTournamentAccess'

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

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!room || !tournament) return <div>Loading Room…</div>

  const { isOwnerOrSuperUser, canViewAuditColumns, canCreate } = access

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/room/${roomid}/overview` },
    { kind: 'route' as const, label: 'Games',    to: `/room/${roomid}/games`    },
  ]

  return (
    <ProfileLayout title={<>Room:<br />{room.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Room', name: room.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <RoomProfileOverviewPage room={room} tournament={tournament} onUpdated={setRoom} showSensitiveColumns={isOwnerOrSuperUser} />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} roomid={roomid}
              showCreateButton={canCreate('game:create')} showDeleteButton={canCreate('game:delete')}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Room']} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
