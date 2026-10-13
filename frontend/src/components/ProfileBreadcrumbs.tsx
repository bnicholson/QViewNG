import { Breadcrumbs } from '@mui/material'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import { ProfileSidebarToggle } from '../contexts/ProfileSidebarContext'

/** One segment of a profile breadcrumb: "{label}: {name}" where only the name is a link. */
export interface Crumb {
  /** Domain entity type shown before the name, e.g. "Tournament". Omit for "Home". */
  label?: string;
  /** The entity's name (or "Home"). */
  name: string;
  /** Link target for the name. Omit to render the name as plain text (the current page). */
  to?: string;
}

/**
 * Renders a profile breadcrumb like:
 *   Home / [Tournament] {name} / [Division] {name} / [Team] {name}
 * The type indicator ("[Tournament]") is plain text; only the name is clickable, and only when
 * a `to` is given (the current/last entity is plain text).
 *
 * This row also carries the profile sidebar's collapse/expand control, immediately left of the
 * path — the only place it appears, in either state.
 *
 * `actions` sit at the right end of the row, pinned to its top: a long path wraps beside them
 * rather than pushing them down.
 */
export function ProfileBreadcrumbs({ crumbs, actions }: { crumbs: Crumb[]; actions?: React.ReactNode }) {
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, minWidth: 0 }}>
      <ProfileSidebarToggle />
      <Breadcrumbs aria-label="breadcrumb" sx={{ flex: 1, minWidth: 0 }}>
      {crumbs.map((c, i) => (
        <span key={i} style={{ display: 'inline-flex', alignItems: 'baseline', gap: 4 }}>
          {c.label && (
            <Typography component="span" color="text.secondary">
              {c.label}:
            </Typography>
          )}
          &nbsp;
          {c.to ? (
            <Link
              to={c.to}
              style={{ color: '#2563eb', textDecoration: 'none' }}
              onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
              onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
            >
              {c.name}
            </Link>
          ) : (
            <Typography component="span" color="text.primary">{c.name}</Typography>
          )}
        </span>
      ))}
      </Breadcrumbs>
      {actions && (
        <Box sx={{ alignSelf: 'flex-start', flexShrink: 0, display: 'flex', gap: 0.5 }}>
          {actions}
        </Box>
      )}
    </Box>
  )
}
