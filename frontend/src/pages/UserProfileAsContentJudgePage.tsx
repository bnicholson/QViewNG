import UserGamesAsContentJudgeTable from '../components/UserGamesAsContentJudgeTable'

export const UserProfileAsContentJudgePage = ({
  userId,
  canAdminister,
  isOwnProfile = false,
}: {
  userId: string
  canAdminister: boolean
  isOwnProfile?: boolean
}) => {
  return (
    <UserGamesAsContentJudgeTable
      userId={userId}
      showCreateButton={canAdminister}
      showDeleteButton={canAdminister}
      showAuditColumns={isOwnProfile}
    />
  )
}
