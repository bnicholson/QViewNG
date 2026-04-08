import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ManageUsers } from './ManageUsers'
import { ManageCreateTournamentApplicants } from './ManageCreateTournamentApplicants'

const navItems = [
  { kind: 'route' as const, label: 'Users',                  to: '/crm/users'                          },
  { kind: 'route' as const, label: 'Tournament Applicants',  to: '/crm/create-tournament-applicants'   },
]

export const CRMProfile = (props: { childRoute?: string }) => {
  return (
    <ProfileLayout title="CRM" navItems={navItems}>
      <Stack spacing={3}>
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'users' && <ManageUsers />}
          {props.childRoute === 'create-tournament-applicants' && <ManageCreateTournamentApplicants />}
        </Box>
      </Stack>
    </ProfileLayout>
  )
}
