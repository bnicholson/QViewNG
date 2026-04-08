import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import CreateTournamentApplicantsTable from '../components/CreateTournamentApplicantsTable'
import { useAuth } from '../hooks/useAuth'

export const ManageCreateTournamentApplicants = () => {
  const { session } = useAuth();
  const userId = session?.userId ?? '';

  return (
    <Stack spacing={2}>
      <Typography variant="body2" color="text.secondary">
        Note: To 'Approve' or 'Decline' an application, click the status label, change the status and save.
        <br/>
        Once status is no longer 'Pending' it cannot be updated; the user must submit a new application.
      </Typography>
      <CreateTournamentApplicantsTable currentUserId={userId ?? ''} />
    </Stack>
  );
};
