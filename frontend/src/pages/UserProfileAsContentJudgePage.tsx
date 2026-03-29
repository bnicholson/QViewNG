import UserGamesAsContentJudgeTable from '../components/UserGamesAsContentJudgeTable'

export const UserProfileAsContentJudgePage = ({
  userId,
  isSuperUser,
}: {
  userId: string
  isSuperUser: boolean
}) => {
  return (
    <UserGamesAsContentJudgeTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
    />
  )
}
