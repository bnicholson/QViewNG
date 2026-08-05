import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ManageUsers } from './ManageUsers'
import { ManageCreateTournamentApplicants } from './ManageCreateTournamentApplicants'
import { ManageGameevents } from './ManageGameevents'

const navItems = [
  { kind: 'route' as const, label: 'Users',                         to: '/crm/users'                          },
  { kind: 'route' as const, label: 'Create Tournament Applicants',  to: '/crm/create-tournament-applicants'   },
  { kind: 'route' as const, label: 'Game Events',                   to: '/crm/gameevents'                     },
]

export const CRMProfile = (props: { childRoute?: string }) => {
  return (
    <ProfileLayout title="CRM" navItems={navItems}>
      <Stack spacing={3}>
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'users' && <ManageUsers />}
          {props.childRoute === 'create-tournament-applicants' && <ManageCreateTournamentApplicants />}
          {props.childRoute === 'gameevents' && <ManageGameevents />}
        </Box>
      </Stack>
    </ProfileLayout>
  )
}
