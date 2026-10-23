import { Children, type ReactNode } from 'react'
import Box from '@mui/material/Box'
import Link from '@mui/material/Link'
import Typography from '@mui/material/Typography'
import { Link as RouterLink } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import type { ExternalLink } from './ExternalLinks'

/**
 * A titled list of links, one per line: the look shared by the mobile About page and every column
 * of the desktop footer. Each child is one line.
 */
export function LinkSection({ heading, children }: { heading: string; children: ReactNode }) {
  return (
    <Box>
      <Typography variant="h6" component="h2" sx={{ fontWeight: 600, mb: 1 }}>
        {heading}
      </Typography>
      {Children.map(children, child => (
        <Typography variant="body1" component="div" sx={{ mb: 0.5 }}>
          {child}
        </Typography>
      ))}
    </Box>
  )
}

/**
 * A `LinkSection` of outside links, opened in a new tab. `linkColor` is the only difference between
 * its two homes: the footer's dark background needs `inherit` (white) rather than the default link
 * color.
 */
export function ExternalLinkSection({ heading, links, linkColor }: {
  heading: string
  links: ExternalLink[]
  linkColor?: 'inherit'
}) {
  return (
    <LinkSection heading={heading}>
      {links.map(p => (
        <Link key={p.url} href={p.url} target="_blank" rel="noopener noreferrer" underline="hover" color={linkColor}>
          {p.name}
        </Link>
      ))}
    </LinkSection>
  )
}

/**
 * The "Accounts" `LinkSection`: Login (Logout when signed in), Register and Forgot Password. Shown in
 * the desktop footer and on the mobile About page; `linkColor` works as in `ExternalLinkSection`.
 */
export function AccountsLinkSection({ linkColor }: { linkColor?: 'inherit' }) {
  const auth = useAuth()

  return (
    <LinkSection heading="Accounts">
      {auth.isAuthenticated ? (
        <Link component="button" onClick={() => auth.logout()} color={linkColor} underline="hover" sx={{ font: 'inherit', verticalAlign: 'baseline' }}>
          Logout
        </Link>
      ) : (
        <Link component={RouterLink} to="/login" color={linkColor} underline="hover">
          Login
        </Link>
      )}
      <Link component={RouterLink} to="/register" color={linkColor} underline="hover">
        Register
      </Link>
      <Link component={RouterLink} to="/recovery" color={linkColor} underline="hover">
        Forgot Password
      </Link>
    </LinkSection>
  )
}
