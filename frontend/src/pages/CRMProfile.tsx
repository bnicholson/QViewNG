import { Navigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileSidebarToggle } from '../contexts/ProfileSidebarContext'
import { ManageUsers } from './ManageUsers'
import { ManageCreateTournamentApplicants } from './ManageCreateTournamentApplicants'
import { ManageGameevents } from './ManageGameevents'
import { useAuth } from '../hooks/useAuth'
import { PERMISSION } from '../auth/permissions'

const navItems = [
  { kind: 'route' as const, label: 'Users',                         to: '/su-console/users'                          },
  { kind: 'route' as const, label: 'Create Tournament Applicants',  to: '/su-console/create-tournament-applicants'   },
  { kind: 'route' as const, label: 'Game Events',                   to: '/su-console/gameevents'                     },
]

export const CRMProfile = (props: { childRoute?: string }) => {
  const { session, isCheckingAuth } = useAuth()

  // Superusers only (`system:access_superuser_console`, or `system:administer`): everyone else, signed in or not, gets the 404 page, so the console's existence
  // isn't revealed. Wait only for the first sign-in check; the periodic re-checks keep the session,
  // so the page (and any open dialog) stays mounted through them.
  if (!session) return isCheckingAuth ? null : <Navigate to="/404" replace />
  if (!session.can(PERMISSION.SYSTEM_ACCESS_SUPERUSER_CONSOLE)) return <Navigate to="/404" replace />

  return (
    <ProfileLayout entityType="Superuser Console" navItems={navItems}>
      <Stack spacing={3}>
        {/* The Superuser Console has no breadcrumb row, so it hosts the sidebar toggle itself. */}
        <Box sx={{ display: 'flex' }}><ProfileSidebarToggle /></Box>
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'users' && <ManageUsers />}
          {props.childRoute === 'create-tournament-applicants' && <ManageCreateTournamentApplicants />}
          {props.childRoute === 'gameevents' && <ManageGameevents />}
        </Box>
      </Stack>
    </ProfileLayout>
  )
}
