import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import UserMyTeamsTable from '../components/UserMyTeamsTable'

export const UserProfileMyTeamsPage = ({
  userId,
  isSuperUser,
  isOwnProfile = false,
}: {
  userId: string
  isSuperUser: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: "left" }}>
        This page shows the teams that this person/user has been a part of, either as a quizzer or as a coach.
      </Typography>
      <UserMyTeamsTable
        userId={userId}
        showCreateButton={isSuperUser}
        showDeleteButton={isSuperUser}
        showAuditColumns={isOwnProfile}
      />
    </Box>
  )
}
