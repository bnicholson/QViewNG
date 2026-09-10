import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import GamesTable from '../components/GamesTable'
import { RoundProfileOverviewPage } from './RoundProfileOverviewPage'

export const RoundProfile = (props: { childRoute?: string }) => {
  const { roundid } = useParams()
  if (!roundid) return <></>

  const [round, setRound] = useState<RoundTS | null>(null)
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    RoundAPI.getById(roundid)
      .then(r => {
        setRound(r)
        return DivisionAPI.getById(r.did)
      })
      .then(div => {
        setDivision(div)
        return TournamentAPI.getById(div.tid)
      })
      .then(setTournament)
      .catch(() => setNotFound(true))
  }, [roundid])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!round || !division || !tournament) return <div>Loading Round…</div>

  const { isOwnerOrSuperUser, canViewAuditColumns, canCreate } = access

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/round/${roundid}/overview` },
    { kind: 'route' as const, label: 'Games',    to: `/round/${roundid}/games`    },
  ]

  return (
    <ProfileLayout title={<>Round:<br />{round.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
          { label: 'Round', name: round.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <RoundProfileOverviewPage
              round={round}
              division={division}
              tournament={tournament}
              onUpdated={setRound}
              canEdit={isOwnerOrSuperUser}
            />
          )}
          {props.childRoute === 'games' && (
            <GamesTable tid={tournament.tid} roundid={roundid}
              showCreateButton={canCreate('game:create')} showDeleteButton={canCreate('game:delete')}
              showAuditColumns={canViewAuditColumns}
              // Breadcrumb here is Home / Tournament / Division / Round, so both Division and Round are fixed context.
              hiddenColumns={['Division', 'Round']} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
