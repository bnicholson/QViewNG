import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'
import { InfoItem } from '../components/InfoItem'
import type { PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import type { DivisionTS } from '../features/DivisionAPI'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  /** The pool/bracket group being viewed. */
  group: PoolBracketGroupTS
  /** Its parent division. */
  division: DivisionTS
  /** How many pools and brackets the group holds (shown as counts). */
  poolCount: number
  bracketCount: number
  /** Opens the group's editor dialog, which the profile owns. */
  onEdit: () => void
  canEdit?: boolean
}

/**
 * The pool/bracket group's Overview. Laid out like the Tournament overview: the name centered at the
 * top with an Edit button beneath it that opens the group's editor dialog, then its details.
 */
export const PoolBracketGroupProfileOverviewPage = ({ group, division, poolCount, bracketCount, onEdit, canEdit = false }: Props) => (
  <Box>
    <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
      <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
        {group.name}
      </Typography>
      <Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
        {canEdit && (
          <Button variant="contained" size="small" onClick={onEdit}>
            Edit
          </Button>
        )}
      </Box>
    </Box>

    <Divider sx={{ mb: 2 }} />

    <Grid container spacing={{ xs: 1, sm: 2 }}>
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
      <InfoItem label="Pools" value={String(poolCount)} />
      <InfoItem label="Brackets" value={String(bracketCount)} />
      <InfoItem label="Created" value={formatDate(group.created_date)} />
      <InfoItem label="Last Modified" value={formatDate(group.last_modified_date)} />
    </Grid>

    <Typography variant="body2" color="text.secondary" sx={{ mt: 3 }}>
      A Pool/Bracket Group holds the pools and brackets that run concurrently in this division —
      every team competes in exactly one pool per group, so a later group can re-pool the same teams.
    </Typography>
  </Box>
)
