import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import UserMyTeamsTable from '../components/UserMyTeamsTable'

export const UserProfileMyTeamsPage = ({
  userId,
  isSuperUser,
}: {
  userId: string
  isSuperUser: boolean
}) => {
  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: "left" }}>
        The "My Teams" section shows the teams this person/user has been a part of, whether as a quizzer or as a coach.
      </Typography>
      <UserMyTeamsTable
        userId={userId}
        showCreateButton={isSuperUser}
        showDeleteButton={isSuperUser}
      />
    </Box>
  )
}
