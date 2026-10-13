import type { Dayjs } from 'dayjs'
import Box from '@mui/material/Box'
import Chip from '@mui/material/Chip'
import Typography from '@mui/material/Typography'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import { isRegistrationOpen, type TournamentTS } from '../features/TournamentAPI'
import { getTournamentStatus } from '../utils/tournamentStatus'

/** "Oct 12 – 17, 2026", "Oct 30 – Nov 2, 2026" or "Dec 30, 2026 – Jan 2, 2027". */
function formatDateRange(from: Dayjs, to: Dayjs): string {
  if (from.isSame(to, 'day')) return from.format('MMM D, YYYY')
  if (from.isSame(to, 'month')) return `${from.format('MMM D')} – ${to.format('D, YYYY')}`
  if (from.isSame(to, 'year')) return `${from.format('MMM D')} – ${to.format('MMM D, YYYY')}`
  return `${from.format('MMM D, YYYY')} – ${to.format('MMM D, YYYY')}`
}

const oneLine = { whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' } as const

/**
 * A tournament in the finder on mobile: full width and three lines tall, so several fit on screen.
 * Keeps what decides whether a tournament is the one you want — name, status, dates, place, venue,
 * organization, and whether registration is open — and drops the free-text info and the country.
 */
export const TournamentCardCompact = ({ tournament }: { tournament: TournamentTS }) => {
  const { fromdate, todate } = tournament
  const status = fromdate && todate ? getTournamentStatus(fromdate, todate) : null
  const place = [tournament.city, tournament.state].filter(Boolean).join(', ')
  const when = fromdate && todate ? formatDateRange(fromdate, todate) : null
  const where = [tournament.venue, tournament.organization].filter(Boolean).join(' · ')
  const registrationOpen = isRegistrationOpen(tournament)

  return (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, px: 1.5, py: 1.25, textAlign: 'left' }}>
      <Box sx={{ flex: 1, minWidth: 0 }}>
        <Box sx={{ display: 'flex', alignItems: 'baseline', gap: 1 }}>
          <Typography
            component="h3"
            sx={{
              flex: 1, minWidth: 0, fontWeight: 600, fontSize: '1rem', lineHeight: 1.3,
              // Long names get two lines before truncating, rather than one.
              display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', overflow: 'hidden',
            }}
          >
            {tournament.tname}
          </Typography>
          {status && (
            <Typography component="span" sx={{ flexShrink: 0, fontSize: '0.75rem', fontWeight: 600, color: status.color }}>
              {status.label}
            </Typography>
          )}
        </Box>

        {(when || place) && (
          <Typography variant="body2" sx={{ ...oneLine, mt: 0.25 }}>
            {[when, place].filter(Boolean).join(' · ')}
          </Typography>
        )}

        {(where || registrationOpen) && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 0.25 }}>
            <Typography variant="body2" color="text.secondary" sx={{ ...oneLine, flex: 1, minWidth: 0 }}>
              {where}
            </Typography>
            {registrationOpen && (
              <Chip label="Registration open" size="small" color="success" variant="outlined" sx={{ height: 20, fontSize: '0.6875rem', flexShrink: 0 }} />
            )}
          </Box>
        )}
      </Box>
      <ChevronRightIcon sx={{ color: 'text.disabled', flexShrink: 0 }} />
    </Box>
  )
}
