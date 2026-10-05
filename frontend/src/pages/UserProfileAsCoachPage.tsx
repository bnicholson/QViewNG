import UserTeamsAsCoachTable from '../components/UserTeamsAsCoachTable'

export const UserProfileAsCoachPage = ({
  userId,
  canAdminister,
}: {
  userId: string
  canAdminister: boolean
}) => {
  return (
    <UserTeamsAsCoachTable
      userId={userId}
      showCreateButton={canAdminister}
      showDeleteButton={canAdminister}
    />
  )
}
