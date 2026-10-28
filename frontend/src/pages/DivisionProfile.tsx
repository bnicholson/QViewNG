import { useState, useEffect } from 'react'
import { PERMISSION } from '../auth/permissions'
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
import SetQuestionTypesButton from '../components/SetQuestionTypesButton'
import PoolBracketsTable from '../components/PoolBracketsTable'
import PoolBracketGroupsTable from '../components/PoolBracketGroupsTable'
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

  const { isOwnerOrSystemAdmin, isSystemAdmin, canViewAdmins, canViewAuditColumns, canCreate, canRead } = access

  // Only super users, the tournament owner, and tournament admins may view Stats Groups.
  const canViewStatsGroups = isOwnerOrSystemAdmin || canViewAdmins

  // Pool/Bracket Groups, Pools and Brackets are infrastructure that scheduling creates and manages on
  // the user's behalf, so only super users (who support every tournament) see their sections.
  const showScheduleInternals = isSystemAdmin

  const navItems = [
    ...(canRead(PERMISSION.DIVISION_READ)    ? [{ kind: 'route' as const, label: 'Overview', to: `/division/${did}/overview` }] : []),
    ...(canRead(PERMISSION.TEAM_READ)        ? [{ kind: 'route' as const, label: 'Teams',    to: `/division/${did}/teams`    }] : []),
    ...(canRead(PERMISSION.USER_READ)        ? [{ kind: 'route' as const, label: 'Quizzers', to: `/division/${did}/quizzers` }] : []),
    ...(showScheduleInternals && canRead(PERMISSION.POOLBRACKETGROUP_READ) ? [{ kind: 'route' as const, label: 'Pool/Bracket Groups', to: `/division/${did}/pool-bracket-groups` }] : []),
    ...(showScheduleInternals && canRead(PERMISSION.POOLBRACKET_READ) ? [{ kind: 'route' as const, label: 'Pools',    to: `/division/${did}/pools`    }] : []),
    ...(showScheduleInternals && canRead(PERMISSION.POOLBRACKET_READ) ? [{ kind: 'route' as const, label: 'Brackets', to: `/division/${did}/brackets` }] : []),
    ...(canRead(PERMISSION.ROUND_READ)       ? [{ kind: 'route' as const, label: 'Rounds',   to: `/division/${did}/rounds`   }] : []),
    ...(canRead(PERMISSION.GAME_READ)        ? [{ kind: 'route' as const, label: 'Games',    to: `/division/${did}/games`    }] : []),
    ...(canViewStatsGroups && canRead(PERMISSION.STATSGROUP_READ)
      ? [{ kind: 'route' as const, label: 'Stats Groups', to: `/division/${did}/stats-groups` }]
      : []),
  ]

  return (
    <ProfileLayout entityType="Division" entityName={division.dname} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <DivisionProfileOverviewPage division={division} tournament={tournament} onUpdated={setDivision} canEdit={canCreate(PERMISSION.DIVISION_UPDATE)} />
          )}
          {props.childRoute === 'teams' && (
            <TeamsTable tid={tournament.tid} did={did}
              showCreateButton={canCreate(PERMISSION.TEAM_CREATE)} showDeleteButton={canCreate(PERMISSION.TEAM_DELETE)}
              showAuditColumns={canViewAuditColumns}
              // In the Division profile every team is in this division, so the Division column is redundant.
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'quizzers' && (
            <QuizzersTable did={did}
              showSensitiveColumns={isOwnerOrSystemAdmin} showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'pool-bracket-groups' && !showScheduleInternals && <Navigate to={`/division/${did}/overview`} replace />}
          {props.childRoute === 'pool-bracket-groups' && showScheduleInternals && (
            <PoolBracketGroupsTable tid={tournament.tid} did={did}
              showCreateButton={canCreate(PERMISSION.POOLBRACKETGROUP_CREATE)}
              showEditButton={canCreate(PERMISSION.POOLBRACKETGROUP_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.POOLBRACKETGROUP_DELETE)}
              showAuditColumns={canViewAuditColumns}
              // In the Division profile every group is in this division, so the Division column is redundant.
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'pools' && !showScheduleInternals && <Navigate to={`/division/${did}/overview`} replace />}
          {props.childRoute === 'pools' && showScheduleInternals && (
            <PoolBracketsTable tid={tournament.tid} did={did}
              type="pool" entityLabel="Pool" title="Pools"
              showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)}
              showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)}
              showAuditColumns={canViewAuditColumns}
              // In the Division profile every pool is in this division, so the Division column is redundant.
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'brackets' && !showScheduleInternals && <Navigate to={`/division/${did}/overview`} replace />}
          {props.childRoute === 'brackets' && showScheduleInternals && (
            <PoolBracketsTable tid={tournament.tid} did={did}
              type="bracket" entityLabel="Bracket" title="Brackets"
              showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)}
              showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)}
              showAuditColumns={canViewAuditColumns}
              // In the Division profile every bracket is in this division, so the Division column is redundant.
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'rounds' && (
            <>
              {canCreate(PERMISSION.ROUND_UPLOAD_QUESTION_TYPES) && (
                <Box sx={{ mb: 1 }}>
                  <SetQuestionTypesButton did={did} />
                </Box>
              )}
              <RoundsTable tid={tournament.tid} did={did}
                showCreateButton={canCreate(PERMISSION.ROUND_CREATE)} showDeleteButton={canCreate(PERMISSION.ROUND_DELETE)}
                showAuditColumns={canViewAuditColumns}
                hiddenColumns={['Division']} />
            </>
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} did={did}
              showCreateButton={canCreate(PERMISSION.GAME_CREATE)} showDeleteButton={canCreate(PERMISSION.GAME_DELETE)}
              showAuditColumns={canViewAuditColumns}
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
