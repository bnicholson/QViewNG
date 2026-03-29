import UserTeamsAsQuizzerTable from '../components/UserTeamsAsQuizzerTable'

export const UserProfileAsQuizzerPage = ({
  userId,
  isSuperUser,
}: {
  userId: string
  isSuperUser: boolean
}) => {
  return (
    <UserTeamsAsQuizzerTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
    />
  )
}
