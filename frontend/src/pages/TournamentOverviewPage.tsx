import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import IconButton from '@mui/material/IconButton'
import Visibility from '@mui/icons-material/Visibility'
import VisibilityOff from '@mui/icons-material/VisibilityOff'
import { useNavigate } from 'react-router-dom'
import { isRegistrationOpen, type TournamentTS } from '../features/TournamentAPI'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import React, { useState } from 'react'
import { InfoItem } from '../components/InfoItem'

// Pairing code is hidden by default (shown as 8 "#") and revealed via a toggle button.
const PairingCodeInfoItem = ({ code }: { code: string }) => {
  const [visible, setVisible] = useState(false)
  return (
    <Grid size={{ xs: 12, sm: 6, md: 4 }}>
      <Typography variant="body2" color="text.secondary">Pairing Code</Typography>
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 0.5 }}>
        <Typography variant="body1" sx={{ fontFamily: visible ? 'monospace' : undefined }}>
          {visible ? code : '########'}
        </Typography>
        <IconButton
          size="small"
          onClick={() => setVisible(v => !v)}
          aria-label={visible ? 'Hide pairing code' : 'Show pairing code'}
        >
          {visible ? <VisibilityOff fontSize="small" /> : <Visibility fontSize="small" />}
        </IconButton>
      </Box>
    </Grid>
  )
}

interface TournamentOverviewPageProps {
  tournament: TournamentTS
  isTournamentUpdate: boolean
  canViewPairingCodeAndVisibility: boolean
  onEdit: () => void
}

export const TournamentOverviewPage = ({ tournament, isTournamentUpdate, canViewPairingCodeAndVisibility, onEdit }: TournamentOverviewPageProps) => {
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
          {isRegistrationOpen(tournament) && (() => {
            // Land on the first registration type this tournament actually offers; hide the button
            // entirely if none are offered.
            const firstEnabledTab =
              (tournament.use_team_registration ?? true) ? 'team'
              : (tournament.use_gear_registration ?? true) ? 'gear'
              : (tournament.use_volunteer_registration ?? true) ? 'as-volunteer'
              : null;
            return firstEnabledTab && (
              <Button variant="contained" size="small" onClick={() => navigate(`/tournament/${tournament.tid}/register/${firstEnabledTab}`)}>
                Register
              </Button>
            );
          })()}
        </Box>
      </Box>

      <Divider sx={{ mb: 2 }} />

      <Grid container spacing={{ xs: 1, sm: 2 }}>
        {canViewPairingCodeAndVisibility && (
          <InfoItem label="Visibility" value={tournament.is_public ? 'Public' : 'Private'}/>
        )}
        <InfoItem
          label="Dates"
          value={
            tournament.fromdate && tournament.todate
              ? `${tournament.fromdate.format('MMM D, YYYY')} – ${tournament.todate.format('MMM D, YYYY')}`
              : ''
          }
        />
        {(() => {
          const anyRegistrationEnabled =
            (tournament.use_team_registration ?? true) ||
            (tournament.use_gear_registration ?? true) ||
            (tournament.use_volunteer_registration ?? true);
          // With no registration types enabled there's no window to speak of — show "Registration: None".
          return anyRegistrationEnabled ? (
            <InfoItem
              label={`Registration Window (${isRegistrationOpen(tournament) ? 'OPEN' : 'CLOSED'})`}
              value={
                tournament.registration_open_date && tournament.registration_close_date
                  ? `${tournament.registration_open_date.format('MMM D, YYYY')} – ${tournament.registration_close_date.format('MMM D, YYYY')}`
                  : 'Not set'
              }
            />
          ) : (
            <InfoItem label="Registration" value="None" />
          );
        })()}
        <InfoItem label="Venue" value={tournament.venue}/>
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
        <InfoItem label="Contact" value={tournament.contact}/>
        <InfoItem label="Contact Email" value={tournament.contactemail}/>
        <InfoItem label="Organization" value={tournament.organization}/>
        {canViewPairingCodeAndVisibility && (
          <PairingCodeInfoItem code={tournament.pairing_code ?? ''}/>
        )}
        {canViewPairingCodeAndVisibility && (
          <InfoItem
            label="Registration Types Being Used"
            value={
              [
                (tournament.use_team_registration ?? true) && 'Team',
                (tournament.use_gear_registration ?? true) && 'Gear',
                (tournament.use_volunteer_registration ?? true) && 'Volunteer',
              ].filter(Boolean).join(', ') || 'None'
            }
          />
        )}
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
