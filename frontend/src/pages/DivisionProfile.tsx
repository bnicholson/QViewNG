import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import TeamsTable from '../components/TeamsTable'
import RoundsTable from '../components/RoundsTable'
import GamesTable from '../components/GamesTable'
import QuizzersTable from '../components/QuizzersTable'
import { DivisionProfileOverviewPage } from './DivisionProfileOverviewPage'

export const DivisionProfile = (props: { childRoute?: string }) => {
  const { did } = useParams()
  if (!did) return <></>

  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    DivisionAPI.getById(did)
      .then(div => {
        setDivision(div)
        return TournamentAPI.getById(div.tid)
      })
      .then(setTournament)
      .catch(() => setNotFound(true))
  }, [did])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!division || !tournament) return <div>Loading Division…</div>

  const { isOwnerOrSuperUser, canViewAdmins, canViewAuditColumns, canCreate } = access

  // Only super users, the tournament owner, and tournament admins may view Stats Groups.
  const canViewStatsGroups = isOwnerOrSuperUser || canViewAdmins

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

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <DivisionProfileOverviewPage division={division} tournament={tournament} onUpdated={setDivision} canEdit={isOwnerOrSuperUser} />
          )}
          {props.childRoute === 'teams' && (
            <TeamsTable tid={tournament.tid} did={did}
              showCreateButton={canCreate('team:create')} showDeleteButton={canCreate('team:delete')}
              showAuditColumns={canViewAuditColumns}
              // In the Division profile every team is in this division, so the Division column is redundant.
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'quizzers' && (
            <QuizzersTable did={did}
              showSensitiveColumns={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'rounds' && (
            <RoundsTable tid={tournament.tid} did={did}
              showCreateButton={canCreate('round:create')} showDeleteButton={canCreate('round:delete')}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} did={did}
              showCreateButton={canCreate('game:create')} showDeleteButton={canCreate('game:delete')}
              showSensitiveColumns={isOwnerOrSuperUser} showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
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
