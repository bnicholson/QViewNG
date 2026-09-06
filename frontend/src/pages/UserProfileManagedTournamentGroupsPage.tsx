import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import TournamentGroupsTable from '../components/TournamentGroupsTable'

interface Props {
  userId: string
  canCreate: boolean
  canDelete: boolean
  isSuperUser: boolean
  isOwnProfile: boolean
  targetIsTournamentManager: boolean
}

export const UserProfileManagedTournamentGroupsPage = ({ userId, canCreate, canDelete, isSuperUser, isOwnProfile, targetIsTournamentManager }: Props) => {
  return (
    <Box>
      {isSuperUser && !targetIsTournamentManager && (
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: "left" }}>
          Note: <em>This user does not have the appropriate permissions to create or manage Tournaments and Tournament Groups.</em>
        </Typography>
      )}
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: 'left' }}>
        This page shows all tournament groups that you are the owner and manager of.
        <br/><br/>
        Use Tournament Groups to get multi-Tournament stats results and create linked Tournament histories (for example: for a district's season's Tournaments, for recurring annual/semi-annual Tournaments, etc.)
      </Typography>
      {/* Same shared table as the Tournament profile, in owner-scoped mode (audit columns own-profile only). */}
      <TournamentGroupsTable
        ownerId={userId}
        showCreateButton={canCreate}
        showDeleteButton={canDelete}
        showAuditColumns={isOwnProfile}
      />
    </Box>
  )
}
