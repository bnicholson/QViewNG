import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import { useNavigate } from 'react-router-dom'
import type { TournamentTS } from '../features/TournamentAPI'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import React from 'react'

interface TournamentOverviewPageProps {
  tournament: TournamentTS
  isTournamentUpdate: boolean
  canViewPairingCode: boolean
  onEdit: () => void
}

export const TournamentOverviewPage = ({ tournament, isTournamentUpdate, canViewPairingCode, onEdit }: TournamentOverviewPageProps) => {
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

      <Divider sx={{ mb: 2 }} />

      <Grid container spacing={{ xs: 1, sm: 2 }}>
        {canViewPairingCode && (
          <>
            <Grid size={{ xs: 12, sm: 6, md: 4 }}>
              <Typography variant="body2" color="text.secondary">Visibility</Typography>
              <Typography variant="body1">{tournament.is_public ? 'Public' : 'Private'}</Typography>
            </Grid>
            <Grid size={{ xs: 12, sm: 6, md: 4 }}>
              <Typography variant="body2" color="text.secondary">Pairing Code</Typography>
              <Typography variant="body1" sx={{ fontFamily: 'monospace', letterSpacing: 1 }}>
                {tournament.pairing_code}
              </Typography>
            </Grid>
          </>
        )}
        <Grid size={{ xs: 12, sm: 6, md: 4 }}>
          <Typography variant="body2" color="text.secondary">Organization</Typography>
          <Typography variant="body1">{tournament.organization}</Typography>
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 4 }}>
          <Typography variant="body2" color="text.secondary">Venue</Typography>
          <Typography variant="body1">{tournament.venue}</Typography>
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 4 }}>
          <Typography variant="body2" color="text.secondary">Address</Typography>
          <Typography variant="body1" component="div">
            {[
              tournament.address_line_1,
              tournament.address_line_2,
              [
                [tournament.city, tournament.state].filter(Boolean).join(', '),
                tournament.zip_code,
              ].filter(Boolean).join(' '),
              tournament.country,
            ]
              .filter((line) => line && line.trim())
              .map((line, i) => <div key={i}>{line}</div>)}
          </Typography>
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 4 }}>
          <Typography variant="body2" color="text.secondary">Contact</Typography>
          <Typography variant="body1">{tournament.contact}</Typography>
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 4 }}>
          <Typography variant="body2" color="text.secondary">Contact Email</Typography>
          <Typography variant="body1">{tournament.contactemail}</Typography>
        </Grid>
      </Grid>
      <br/>
      <Divider sx={{ mb: 2 }} />

      {tournament.info && (
        <div style={{textAlign:'left'}}>
          <Markdown remarkPlugins={[remarkGfm]}>{tournament.info}</Markdown>
        </div>
      )}
    </Box>
  )
}
