import { Breadcrumbs } from '@mui/material'
import { Link } from 'react-router-dom'
import Typography from '@mui/material/Typography'

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
 */
export function ProfileBreadcrumbs({ crumbs }: { crumbs: Crumb[] }) {
  return (
    <Breadcrumbs aria-label="breadcrumb">
      {crumbs.map((c, i) => (
        <span key={i} style={{ display: 'inline-flex', alignItems: 'baseline', gap: 4 }}>
          <Link
            to={c.to}
            style={{ color: '#2563eb', textDecoration: 'none' }}
            onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
            onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
          >
            {c.label && (
              <Typography component="span" color="text.secondary">
                {c.label}:
              </Typography>
            )}
            &nbsp;
            {c.to ? (
              <>
                {c.name}
              </>
            ) : (
              <Typography component="span" color="text.primary">{c.name}</Typography>
            )}
          </Link>
        </span>
      ))}
    </Breadcrumbs>
  )
}
