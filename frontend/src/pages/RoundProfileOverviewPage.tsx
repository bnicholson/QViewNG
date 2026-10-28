import { useState } from 'react'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import type { RoundTS } from '../features/RoundAPI'
import type { DivisionTS } from '../features/DivisionAPI'
import type { TournamentTS } from '../features/TournamentAPI'
import { RoundEditorDialog } from '../components/RoundEditorDialog'

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: 'numeric', minute: '2-digit',
  })
}

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  round: RoundTS
  division: DivisionTS
  tournament: TournamentTS
  onUpdated: (round: RoundTS) => void
  canEdit?: boolean
}

export const RoundProfileOverviewPage = ({ round, division, tournament, onUpdated, canEdit = false }: Props) => {
  const [editorOpen, setEditorOpen] = useState(false)

  const handleSave = (updated: RoundTS) => {
    setEditorOpen(false)
    onUpdated(updated)
  }

  return (
    <Stack spacing={3}>
      <Box>
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
          <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
            Round {round.name}
          </Typography>
          {canEdit && (
            <Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
              <Button variant="contained" size="small" onClick={() => setEditorOpen(true)}>
                Edit
              </Button>
            </Box>
          )}
        </Box>

        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Round: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Round Name</Typography>
            <Typography variant="body1">{round.name}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Tournament</Typography>
            <Typography variant="body1">
              <Link
                to={`/tournament/${tournament.tid}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {tournament.tname}
              </Link>
            </Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Division</Typography>
            <Typography variant="body1">
              <Link
                to={`/division/${division.did}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {division.dname}
              </Link>
            </Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Scheduled Start Time</Typography>
            <Typography variant="body1">{formatDateTime(round.scheduled_start_time)}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(round.created_at)}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(round.updated_at)}</Typography>
          </Grid>
        </Grid>

      </Box>

      <RoundEditorDialog
        tid={tournament.tid}
        lockedDivisionId={division.did}
        round={round}
        isOpen={editorOpen}
        onCancel={() => setEditorOpen(false)}
        onSave={handleSave}
      />
    </Stack>
  )
}
