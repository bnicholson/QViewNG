// import Box from '@mui/material/Box'
// import Divider from '@mui/material/Divider'
import Stack from '@mui/material/Stack'
// import Typography from '@mui/material/Typography'
import CreateTournamentApplicantsTable from '../components/CreateTournamentApplicantsTable'
import { useAuth } from '../hooks/useAuth'

export const ManageCreateTournamentApplicants = () => {
  const { session } = useAuth();
  const userId = session?.userId ?? '';

  return (
    <Stack spacing={3}>
      {/* <Box>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600 }}>
          Tournament Applicants
        </Typography>
        <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
          Review and update the status of tournament creation applications.
        </Typography>
      </Box>

      <Divider /> */}

      <CreateTournamentApplicantsTable currentUserId={userId ?? ''} />
    </Stack>
  );
};
