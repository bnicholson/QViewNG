import { useState, useEffect } from 'react'
import { PERMISSION } from '../auth/permissions'
import { Navigate, useParams } from 'react-router'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import TeamsTable from '../components/TeamsTable'
import GamesTable from '../components/GamesTable'
import { PoolBracketProfileOverviewPage } from './PoolBracketProfileOverviewPage'

/** Human label for a pool_brackets `type` ("pool" → "Pool", "bracket" → "Bracket"). */
function typeLabel(type: string | undefined): string {
  if (!type) return 'Pool Bracket'
  return type.charAt(0).toUpperCase() + type.slice(1)
}

export const PoolBracketProfile = (props: { childRoute?: string }) => {
  const { bracketid } = useParams()
  if (!bracketid) return <></>

  const [bracket, setBracket] = useState<PoolBracketTS | null>(null)
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    let cancelled = false
    PoolBracketAPI.getById(bracketid)
      .then(async b => {
        if (cancelled) return
        setBracket(b)
        const div = await DivisionAPI.getById(b.divisionid)
        if (cancelled) return
        setDivision(div)
        const tour = await TournamentAPI.getById(div.tid)
        if (cancelled) return
        setTournament(tour)
      })
      .catch(() => { if (!cancelled) setNotFound(true) })
    return () => { cancelled = true }
  }, [bracketid])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!bracket || !division || !tournament) return <div>Loading…</div>

  const { canViewAuditColumns, canCreate, canRead } = access
  const label = typeLabel(bracket.type)

  const navItems = [
    ...(canRead(PERMISSION.POOLBRACKET_READ) ? [{ kind: 'route' as const, label: 'Overview', to: `/pool-bracket/${bracketid}/overview` }] : []),
    ...(canRead(PERMISSION.TEAM_READ)        ? [{ kind: 'route' as const, label: 'Teams',    to: `/pool-bracket/${bracketid}/teams`    }] : []),
    ...(canRead(PERMISSION.GAME_READ)        ? [{ kind: 'route' as const, label: 'Games',    to: `/pool-bracket/${bracketid}/games`    }] : []),
  ]

  return (
    <ProfileLayout title={<>{label}:<br />{bracket.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
          { label: label, name: bracket.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <PoolBracketProfileOverviewPage
              bracket={bracket} division={division}
              entityLabel={label} onUpdated={setBracket} canEdit={canCreate(PERMISSION.POOLBRACKET_UPDATE)} />
          )}
          {props.childRoute === 'teams' && (
            // Teams associated with this pool bracket via its teamgroup. Creating a team here also
            // adds it to this bracket; the row delete removes it from the bracket (not the team).
            <TeamsTable tid={tournament.tid} did={division.did} poolBracketId={bracketid}
              showCreateButton={canCreate(PERMISSION.TEAM_CREATE)}
              showEditButton={canCreate(PERMISSION.TEAM_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.TEAM_DELETE)}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} poolbracketid={bracketid}
              showCreateButton={false} showDeleteButton={canCreate(PERMISSION.GAME_DELETE)}
              showAuditColumns={canViewAuditColumns} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
