import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Breadcrumbs from '@mui/material/Breadcrumbs'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ProfileLayout from '../components/ProfileLayout'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { RoundProfileOverviewPage } from './RoundProfileOverviewPage'
import { RoundProfileGamesPage } from './RoundProfileGamesPage'

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return 'Round'
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: 'numeric', minute: '2-digit',
  })
}

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

  if (notFound) return <Navigate to="/404" replace />
  if (!round || !division || !tournament) return <div>Loading Round…</div>

  const navItems = [
    { kind: 'route' as const, label: 'Overview', to: `/round/${roundid}/overview` },
    { kind: 'route' as const, label: 'Games',    to: `/round/${roundid}/games`    },
  ]

  return (
    <ProfileLayout title={<>Round:<br />{formatDateTime(round.scheduled_start_time)}</>} navItems={navItems}>
      <Stack spacing={3}>

        <Breadcrumbs aria-label="breadcrumb">
          <Link color="inherit" to="/">Home</Link>
          <Link color="inherit" to={`/tournament/${tournament.tid}/overview`}>{tournament.tname}</Link>
          <Link color="inherit" to={`/division/${division.did}/overview`}>{division.dname}</Link>
          <Typography color="text.primary">{formatDateTime(round.scheduled_start_time)}</Typography>
        </Breadcrumbs>

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <RoundProfileOverviewPage
              round={round}
              division={division}
              tournament={tournament}
              onUpdated={setRound}
            />
          )}
          {props.childRoute === 'games' && (
            <RoundProfileGamesPage tid={tournament.tid} roundid={roundid} />
          )}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
