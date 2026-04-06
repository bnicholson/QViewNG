import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Breadcrumbs from '@mui/material/Breadcrumbs'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'
import { TournamentGroupOverviewPage } from './TournamentGroupOverviewPage'
import { TournamentGroupTournamentsPage } from './TournamentGroupTournamentsPage'
import { useAuth } from '../hooks/useAuth'

export const TournamentGroupProfile = (props: { childRoute?: string }) => {
  const { tgid } = useParams()
  if (!tgid) return <></>

  const { session } = useAuth()

  const [group, setGroup] = useState<TournamentGroupTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    TournamentGroupAPI.getById(tgid)
      .then(setGroup)
      .catch(() => setNotFound(true))
  }, [tgid])

  if (notFound) return <Navigate to="/404" replace />
  if (!group) return <div>Loading Tournament Group…</div>

  const isOwner = session?.userId === group.owner_id || session?.hasRole('super_user')
  const canEdit = isOwner ?? false

  const navItems = [
    { kind: 'route' as const, label: 'Overview',      to: `/tournament-group/${tgid}/overview`    },
    { kind: 'route' as const, label: 'Tournaments',   to: `/tournament-group/${tgid}/tournaments` },
    { kind: 'route' as const, label: 'Stats Groups',  to: `/tournament-group/${tgid}/stats-groups`},
  ]

  return (
    <ProfileLayout title={<>Tournament Group:<br />{group.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Typography color="text.primary">{group.name}</Typography>
        </Breadcrumbs>

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <TournamentGroupOverviewPage
              group={group}
              canEdit={canEdit}
              onUpdated={setGroup}
            />
          )}
          {props.childRoute === 'tournaments' && (
            <TournamentGroupTournamentsPage
              tgid={tgid}
              canEdit={canEdit}
              canCreate={session?.hasPermission('tournament:create') ?? false}
            />
          )}
          {props.childRoute === 'stats-groups' && (
            <Typography color="text.secondary">Stats Groups coming soon.</Typography>
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
