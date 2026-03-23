import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import type { TournamentTS } from '../features/TournamentAPI'

interface TournamentOverviewPageProps {
  tournament: TournamentTS
  isUserAdmin: boolean
  onEdit: () => void
}

export const TournamentOverviewPage = ({ tournament, isUserAdmin, onEdit }: TournamentOverviewPageProps) => {
  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: 1, mb: 2 }}>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
          {tournament.tname}
        </Typography>
        &nbsp;&nbsp;
        {isUserAdmin && (
          <Button variant="outlined" size="small" onClick={onEdit}>
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
