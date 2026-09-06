import UserTournamentsAsAdminTable from '../components/UserTournamentsAsAdminTable'

export const UserProfileAsAdminPage = ({
  userId,
  isSuperUser,
  isOwnProfile = false,
}: {
  userId: string
  isSuperUser: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserTournamentsAsAdminTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
      showAuditColumns={isOwnProfile}
    />
  )
}
