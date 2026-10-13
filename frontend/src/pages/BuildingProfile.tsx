import { useState, useEffect } from 'react'
import { PERMISSION } from '../auth/permissions'
import { Navigate, useParams } from 'react-router'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { RoomGroupAPI, type RoomGroupTS } from '../features/RoomGroupAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import RoomsTable from '../components/RoomsTable'
import { BuildingProfileOverviewPage } from './BuildingProfileOverviewPage'

export const BuildingProfile = (props: { childRoute?: string }) => {
  const { buildingid } = useParams()
  if (!buildingid) return <></>

  const [building, setBuilding] = useState<RoomGroupTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    let cancelled = false
    RoomGroupAPI.getById(buildingid)
      .then(async b => {
        if (cancelled) return
        setBuilding(b)
        const tour = await TournamentAPI.getById(b.tournamentid)
        if (cancelled) return
        setTournament(tour)
      })
      .catch(() => { if (!cancelled) setNotFound(true) })
    return () => { cancelled = true }
  }, [buildingid])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!building || !tournament) return <div>Loading Building…</div>

  const { canViewAuditColumns, canCreate, canRead } = access

  const navItems = [
    ...(canRead(PERMISSION.ROOMGROUP_READ) ? [{ kind: 'route' as const, label: 'Overview', to: `/building/${buildingid}/overview` }] : []),
    ...(canRead(PERMISSION.ROOM_READ)      ? [{ kind: 'route' as const, label: 'Rooms',    to: `/building/${buildingid}/rooms`    }] : []),
  ]

  return (
    <ProfileLayout entityType="Building" entityName={building.name} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Building', name: building.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <BuildingProfileOverviewPage
              building={building}
              tid={tournament.tid}
              onUpdated={setBuilding}
              showAuditColumns={canViewAuditColumns}
              canEdit={canCreate(PERMISSION.ROOMGROUP_UPDATE)} />
          )}
          {props.childRoute === 'rooms' && (
            // The tournament's Rooms table, scoped to this building.
            <RoomsTable tid={tournament.tid} roomgroupid={buildingid}
              showCreateButton={canCreate(PERMISSION.ROOM_CREATE)}
              showDeleteButton={canCreate(PERMISSION.ROOM_DELETE)}
              showAuditColumns={canViewAuditColumns} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
