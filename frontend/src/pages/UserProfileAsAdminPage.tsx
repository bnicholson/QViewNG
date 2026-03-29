import UserTournamentsAsAdminTable from '../components/UserTournamentsAsAdminTable'

export const UserProfileAsAdminPage = ({
  userId,
  isSuperUser,
}: {
  userId: string
  isSuperUser: boolean
}) => {
  return (
    <UserTournamentsAsAdminTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
    />
  )
}
