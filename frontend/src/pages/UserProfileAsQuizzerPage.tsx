import UserTeamsAsQuizzerTable from '../components/UserTeamsAsQuizzerTable'

export const UserProfileAsQuizzerPage = ({
  userId,
  canAdminister,
}: {
  userId: string
  canAdminister: boolean
}) => {
  return (
    <UserTeamsAsQuizzerTable
      userId={userId}
      showCreateButton={canAdminister}
      showDeleteButton={canAdminister}
    />
  )
}
