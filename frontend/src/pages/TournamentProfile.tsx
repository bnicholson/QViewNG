import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Card from "@mui/material/Card"
import CardContent from "@mui/material/CardContent"
import Divider from "@mui/material/Divider"
import Grid from "@mui/material/Grid"
import Stack from "@mui/material/Stack"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
import { Link } from 'react-router-dom'
import Button from '@mui/material/Button';
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { makeCancellable } from '../features/makeCancellable'
import DivisionsTable from '../components/DivisionsTable'
import TournamentTabBar from '../components/TournamentTabBar'
import RoomsTable from '../components/RoomsTable'
import RoundsTable from '../components/RoundsTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'

export const TournamentProfile = (props: { tab: string }) => {

  const isUserAdmin = true;

  const { tid } = useParams();
  if (tid === undefined) return (<></>)

  const validTabs = ['divisions', 'rooms', 'teams', 'rounds', 'quizzers', 'games', 'admins', 'stats-groups'];
  if (props.tab === undefined) { props.tab = validTabs[0] }

  if (!validTabs.includes(props.tab)) {
    return <Navigate to={`/tournament/${tid}`} replace />;
  }

  const [isLoading, setIsLoading] = useState<boolean>(false)
  const stillLoading = () => isLoading || tournament == null || tournament == undefined
  const [notFound, setNotFound] = useState<boolean>(false)
  const [tournament, setTournament] = useState<TournamentTS>()
  const [tournamentEditorIsOpen, setTournamentEditorIsOpen] = useState(false);

  useEffect(() => {
    setIsLoading(true)
    const cancellable = makeCancellable(TournamentAPI.getById(tid));
    try {
      cancellable.promise
        .then((returnedTournament: TournamentTS) => {
          setTournament(returnedTournament)
          setIsLoading(false)
        })
        .catch((error) => {
          if (error.isCancelled) {
            console.log('Info: The request to get Tournament by ID was cancelled');
          } else {
            console.error('Error: Could not load Tournament:', error);
          }
          setIsLoading(false);
          setNotFound(true)
        })
    } catch (err: any) {
      if (err instanceof Error) {
        console.error(err.message)
        setIsLoading(false);
        setNotFound(true)
      }
    }
    setIsLoading(false)
  }, [tid])

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  return (
    <Stack spacing={3}>

      {/* ── Breadcrumb ── */}
      <Breadcrumbs aria-label="breadcrumb">
        <Link color="inherit" to="/">Home</Link>
        <Typography color="text.primary">{tournament?.tname}</Typography>
      </Breadcrumbs>

      {/* ── Header + General Info ── */}
      <Box>
        <Box sx={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: 1, mb: 2 }}>
          <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
            {tournament!.tname}
          </Typography>
          &nbsp;&nbsp;
          {isUserAdmin && (
            <Button variant="outlined" size="small" onClick={() => setTournamentEditorIsOpen(true)}>
              Edit
            </Button>
          )}
        </Box>

        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Tournament: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Organization</Typography>
            <Typography variant="body1">{tournament!.organization}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Visibility</Typography>
            <Typography variant="body1">{tournament!.is_public ? 'Public' : 'Private'}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Venue</Typography>
            <Typography variant="body1">{tournament!.venue}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Location</Typography>
            <Typography variant="body1">
              {[tournament!.city, tournament!.region, tournament!.country].filter(Boolean).join(', ')}
            </Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Contact</Typography>
            <Typography variant="body1">{tournament!.contact}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Contact Email</Typography>
            <Typography variant="body1">{tournament!.contactemail}</Typography>
          </Grid>
          <Grid item xs={12} sm={6}>
            <Typography variant="body2" color="text.secondary">Short Info</Typography>
            <Typography variant="body1">{tournament!.shortinfo}</Typography>
          </Grid>
          {tournament!.info && (
            <Grid item xs={12}>
              <Typography variant="body2" color="text.secondary">More Info</Typography>
              <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>{tournament!.info}</Typography>
            </Grid>
          )}
        </Grid>
      </Box>

      <br/>

      {/* ── Tab card ── */}
      <Card>
        <TournamentTabBar tid={String(tournament?.tid)}/>
        <CardContent sx={{ p: { xs: 1, sm: 2, md: 3 }, overflowX: 'auto' }}>
          {props.tab === 'divisions'    && <DivisionsTable tid={String(tournament?.tid)}/>}
          {props.tab === 'rooms'        && <RoomsTable tid={String(tournament?.tid)}/>}
          {props.tab === 'rounds'       && <RoundsTable tid={String(tournament?.tid)}/>}
          {props.tab === 'teams'        && <Typography color="text.secondary">Teams coming soon.</Typography>}
          {props.tab === 'quizzers'     && <Typography color="text.secondary">Quizzers coming soon.</Typography>}
          {props.tab === 'games'        && <Typography color="text.secondary">Games coming soon.</Typography>}
          {props.tab === 'admins'       && <Typography color="text.secondary">Admins coming soon.</Typography>}
          {props.tab === 'stats-groups' && <Typography color="text.secondary">Stats Groups coming soon.</Typography>}
        </CardContent>
      </Card>

      <TournamentEditorDialog
        initialTournament={tournament}
        isOpen={tournamentEditorIsOpen}
        onCancel={() => setTournamentEditorIsOpen(false)}
        onSave={t => { setTournament(t); setTournamentEditorIsOpen(false); }}
      />

    </Stack>
  )
}
