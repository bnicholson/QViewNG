import UserGamesAsContentJudgeTable from '../components/UserGamesAsContentJudgeTable'

export const UserProfileAsContentJudgePage = ({
  userId,
  isSuperUser,
  isOwnProfile = false,
}: {
  userId: string
  isSuperUser: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserGamesAsContentJudgeTable
      userId={userId}
      showCreateButton={isSuperUser}
      showDeleteButton={isSuperUser}
      showAuditColumns={isOwnProfile}
    />
  )
}
