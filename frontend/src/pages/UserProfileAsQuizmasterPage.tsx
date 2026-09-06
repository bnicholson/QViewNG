import UserGamesAsQuizmasterTable from '../components/UserGamesAsQuizmasterTable'

export const UserProfileAsQuizmasterPage = ({
  userId,
  isSuperUser,
  isOwnProfile = false,
}: {
  userId: string
  isSuperUser: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserGamesAsQuizmasterTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
      showAuditColumns={isOwnProfile}
    />
  )
}
