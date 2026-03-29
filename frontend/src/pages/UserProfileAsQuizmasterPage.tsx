import UserGamesAsQuizmasterTable from '../components/UserGamesAsQuizmasterTable'

export const UserProfileAsQuizmasterPage = ({
  userId,
  isSuperUser,
}: {
  userId: string
  isSuperUser: boolean
}) => {
  return (
    <UserGamesAsQuizmasterTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
    />
  )
}
