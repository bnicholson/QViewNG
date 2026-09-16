import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { RoundGroupAPI, type RoundGroupTS } from '../features/RoundGroupAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import GamesTable from '../components/GamesTable'
import RoundsTable from '../components/RoundsTable'
import { RoundGroupProfileOverviewPage } from './RoundGroupProfileOverviewPage'

export const RoundGroupProfile = (props: { childRoute?: string }) => {
  const { sessionid } = useParams()
  if (!sessionid) return <></>

  const [session, setSession] = useState<RoundGroupTS | null>(null)
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    let cancelled = false
    RoundGroupAPI.getById(sessionid)
      .then(async sess => {
        if (cancelled) return
        setSession(sess)
        const div = await DivisionAPI.getById(sess.did)
        if (cancelled) return
        setDivision(div)
        const tour = await TournamentAPI.getById(div.tid)
        if (cancelled) return
        setTournament(tour)
      })
      .catch(() => { if (!cancelled) setNotFound(true) })
    return () => { cancelled = true }
  }, [sessionid])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!session || !division || !tournament) return <div>Loading Session…</div>

  const { isOwnerOrSuperUser, canViewAuditColumns, canCreate } = access

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/division-session/${sessionid}/overview` },
    { kind: 'route' as const, label: 'Rounds',   to: `/division-session/${sessionid}/rounds`   },
    { kind: 'route' as const, label: 'Games',    to: `/division-session/${sessionid}/games`    },
  ]

  return (
    <ProfileLayout title={<>Session:<br />{session.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
          { label: 'Session', name: session.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <RoundGroupProfileOverviewPage session={session} division={division} onUpdated={setSession} canEdit={isOwnerOrSuperUser} />
          )}
          {props.childRoute === 'rounds' && (
            // Rounds belong to a division session; Division and Session are fixed context here.
            <RoundsTable tid={tournament.tid} did={division.did} roundgroupId={sessionid}
              showCreateButton={canCreate('division:create')}
              showDeleteButton={canCreate('division:delete')}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division', 'Session']} />
          )}
          {props.childRoute === 'games' && (
            // Games whose round belongs to this session. Division is fixed context here.
            <GamesTable tid={tournament.tid} roundgroupId={sessionid}
              showCreateButton={false} showDeleteButton={canCreate('game:delete')}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
