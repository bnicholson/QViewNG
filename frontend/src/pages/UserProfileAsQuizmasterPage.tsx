import UserGamesAsQuizmasterTable from '../components/UserGamesAsQuizmasterTable'

export const UserProfileAsQuizmasterPage = ({
  userId,
  canAdminister,
  isOwnProfile = false,
}: {
  userId: string
  canAdminister: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserGamesAsQuizmasterTable
      userId={userId}
      showCreateButton={canAdminister}
      showDeleteButton={canAdminister}
      showAuditColumns={isOwnProfile}
    />
  )
}
