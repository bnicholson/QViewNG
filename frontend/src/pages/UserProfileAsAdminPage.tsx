import UserTournamentsAsAdminTable from '../components/UserTournamentsAsAdminTable'

export const UserProfileAsAdminPage = ({
  userId,
  canAdminister,
  isOwnProfile = false,
}: {
  userId: string
  canAdminister: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserTournamentsAsAdminTable
      userId={userId}
      showCreateButton={canAdminister}
      showDeleteButton={canAdminister}
      showAuditColumns={isOwnProfile}
    />
  )
}
