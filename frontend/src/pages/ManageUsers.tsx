// import Box from '@mui/material/Box'
// import Divider from '@mui/material/Divider'
import Stack from '@mui/material/Stack'
// import Typography from '@mui/material/Typography'
import UsersTable from '../components/UsersTable'

export const ManageUsers = () => {
  return (
    <Stack spacing={3}>
      {/* <Box>
        <Typography variant="h5" >
          Manage Users
        </Typography>
        <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
          Create, edit, and deactivate user accounts.
        </Typography>
      </Box>

      <Divider /> */}

      <UsersTable />
    </Stack>
  );
};
