import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import { useNavigate } from 'react-router-dom'
import type { TournamentTS } from '../features/TournamentAPI'

interface TournamentOverviewPageProps {
  tournament: TournamentTS
  isTournamentUpdate: boolean
  onEdit: () => void
}

export const TournamentOverviewPage = ({ tournament, isTournamentUpdate, onEdit }: TournamentOverviewPageProps) => {
  const navigate = useNavigate()
  return (
    <Box>
      <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
          {tournament.tname}
        </Typography>
        <Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
          {isTournamentUpdate && (
            <Button variant="contained" size="small" onClick={onEdit}>
              Edit
            </Button>
          )}
          <Button variant="contained" size="small" onClick={() => navigate(`/tournament/${tournament.tid}/register/team`)}>
            Register
          </Button>
        </Box>
      </Box>
      <br/>
      <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
        Tournament: General Info
      </Typography>
      <Divider sx={{ mb: 2 }} />

      <Grid container spacing={{ xs: 1, sm: 2 }}>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Organization</Typography>
          <Typography variant="body1">{tournament.organization}</Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Visibility</Typography>
          <Typography variant="body1">{tournament.is_public ? 'Public' : 'Private'}</Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Venue</Typography>
          <Typography variant="body1">{tournament.venue}</Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Location</Typography>
          <Typography variant="body1">
            {[tournament.city, tournament.region, tournament.country].filter(Boolean).join(', ')}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Contact</Typography>
          <Typography variant="body1">{tournament.contact}</Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Contact Email</Typography>
          <Typography variant="body1">{tournament.contactemail}</Typography>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Typography variant="body2" color="text.secondary">Pairing Code</Typography>
          <Typography variant="body1" sx={{ fontFamily: 'monospace', letterSpacing: 1 }}>
            {tournament.pairing_code}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={6}>
          <Typography variant="body2" color="text.secondary">Short Info</Typography>
          <Typography variant="body1">{tournament.shortinfo}</Typography>
        </Grid>
        {tournament.info && (
          <Grid item xs={12}>
            <Typography variant="body2" color="text.secondary">More Info</Typography>
            <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>{tournament.info}</Typography>
          </Grid>
        )}
      </Grid>
    </Box>
  )
}
