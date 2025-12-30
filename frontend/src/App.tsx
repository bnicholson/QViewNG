// import { useAuth, useAuthCheck } from './hooks/useAuth';
import { AccountPage } from './pages/AccountPage';
import { LoginPage } from './pages/LoginPage';
import { ActivationPage } from './pages/ActivationPage';
import { RegistrationPage } from './pages/RegistrationPage';
import { RecoveryPage } from './pages/RecoveryPage';
import { ResetPage } from './containers/ResetPage';
import { TournamentPage } from './pages/TournamentPage';
// import { Divisions } from './pages/DivisionPage';
import { TDEditor } from './containers/TDEditor';
import React from 'react';
import './App.css';
import { Files } from './containers/Files';
import { Route, useNavigate, Routes, Navigate } from 'react-router-dom';
import '@mui/material/colors';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import IconButton from '@mui/material/IconButton';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import MenuIcon from '@mui/icons-material/Menu';
import Drawer from '@mui/material/Drawer';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import styled from '@emotion/styled';
import { ChevronLeft, Inbox, Mail, AccountCircle } from '@mui/icons-material';
import { Backdrop, Divider, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { createTheme, ThemeProvider } from '@mui/material';
import CssBaseline from '@mui/material/CssBaseline';
import { RoundsInProgress } from './containers/RoundsInProgress';
import { Swagger } from './containers/Swagger'
import { TournamentFinder } from './pages/TournamentFinder'
import { TournamentProfile } from './pages/TournamentProfile';
import TournamentRedirect from './pages/TournamentRedirect';

if (import.meta.env.NODE_ENV === 'development') import('./setupDevelopment')

const drawerWidth = 240;

const theme = createTheme({
  palette: {
    primary: {
      main: "#008080"
    }
  },
  typography: {
    button: {
      textTransform: 'none'
    }
  }
})

export default function App() {
  const navigate = useNavigate()
  // useAuthCheck()
  // const auth = useAuth()


  // const theme = useTheme();  

  // const DrawerHeader = styled('div')(({ theme }) => ({
  const DrawerHeader = styled('div')(() => ({
    display: 'flex',
    alignItems: 'center',
    //padding: theme.spacing(0, 1),
    // necessary for content to be below app bar
    //    ...theme.mixins.toolbar,
    justifyContent: 'flex-end',
  }));

  const [drawerIsOpen, setDrawerIsOpen] = React.useState(false);

  const toggleDrawer = () => setDrawerIsOpen(!drawerIsOpen)

  // @ts-ignore
  return (
    <ThemeProvider theme={theme}>
      <div className="App">
        <Box sx={{ flexGrow: 1 }}>
          <AppBar position="static">
            <Toolbar>
              <IconButton
                size="large"
                edge="start"
                color="inherit"
                aria-label="menu"
                sx={{ mr: 2 }}
                onClick={() => { toggleDrawer() }}
              >
                <MenuIcon />
              </IconButton>
              <Typography variant="h6" component="div" onClick={() => { navigate("/") }} style={{ cursor: 'pointer' }}>QView</Typography>
              <Typography variant="h6" component="div" sx={{ flexGrow: 2 }} onClick={() => toggleDrawer()}>

              </Typography>
              {/* <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                {/* CRA: right-aligned nav buttons *
                {auth.isAuthenticated && <a onClick={() => { auth.logout(); apollo.resetStore(); }}>Logout</a>}
                {!auth.isAuthenticated && <button onClick={() => navigate('/login')}>Login/Register</button>}
              </Typography> */}
              {/* {auth.isAuthenticated && <IconButton onClick={() => navigate('/account')}> <AccountCircle /></IconButton>} */}
            </Toolbar>
          </AppBar>

        </Box>
        <Backdrop
          onClick={toggleDrawer}
          open={drawerIsOpen}
          sx={{ zIndex: 1 }}
        >
          <Drawer
            sx={{
              width: drawerWidth,
              flexShrink: 0,
              '& .MuiDrawer-paper': {
                width: drawerWidth,
                boxSizing: 'border-box',
              },
            }}
            variant="persistent"
            anchor="left"
            open={drawerIsOpen}
          >
            <DrawerHeader>
              <IconButton onClick={toggleDrawer}>
                <ChevronLeft />
              </IconButton>
            </DrawerHeader>
            <Divider />
            <List>
              {['Tournament', 'Division', 'Room', 'Round'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 4) {
                      case 0:
                        navigate("/tournament");
                        break;
                      case 1:
                        navigate("/division");
                        break;
                      case 2:
                        alert("room");
                        break;
                      case 3:
                        alert('round');
                    }
                  }}>
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
            <Divider />
            <List>
              {['Quizzes', 'Team', 'Individual'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 3) {
                      case 0:
                        alert("quizzes");
                        break;
                      case 1:
                        alert("team");
                        break;
                      case 2:
                        alert("individual");
                        break;
                    }
                  }}>
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
            <Divider />
            <List>
              {['Files'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 3) {
                      case 0:
                        navigate("/files");
                        break;
                      // case 1:
                      //   navigate("/todos");
                      //   break;
                      // case 2:
                      //   navigate("/gql");
                      //   break;
                    }
                  }}
                >
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
          </Drawer>
        </Backdrop>
        <div style={{ margin: '0 auto', maxWidth: '1200px' }}>
          <Routes>
            <Route path="/" element={<TournamentFinder />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/recovery" element={<RecoveryPage />} />
            <Route path="/reset" element={<ResetPage />} />
            <Route path="/activate" element={<ActivationPage />} />
            <Route path="/register" element={<RegistrationPage />} />
            <Route path="/account" element={<AccountPage />} />
            {/* <Route path="/tournament" element={<Tournaments />} /> */}
            <Route path="/tournaments" element={<TournamentPage />} />
            <Route path="/tournament/:tid_str" element={<TournamentRedirect />} />
            <Route path="/tournament/:tid_str/divisions" element={<TournamentProfile tab="divisions" />} />
            {/* <Route path="/tournament/:tid_str/division/:did_str" element={<DivisionProfile tab="" />} />  // <- future */}
            <Route path="/tournament/:tid_str/rooms" element={<TournamentProfile tab="rooms" />} />
            <Route path="/tournament/:tid_str/teams" element={<TournamentProfile tab="teams" />} />
            <Route path="/tournament/:tid_str/rounds" element={<TournamentProfile tab="rounds" />} />
            <Route path="/tournament/:tid_str/quizzers" element={<TournamentProfile tab="quizzers" />} />
            <Route path="/tournament/:tid_str/games" element={<TournamentProfile tab="games" />} />
            <Route path="/tournament/:tid_str/admins" element={<TournamentProfile tab="admins" />} />
            <Route path="/tournament/:tid_str/stats-groups" element={<TournamentProfile tab="stats-groups" />} />
            {/* <Route path="/division" element={<Divisions />} /> */}
            <Route path="/tdeditor" element={<TDEditor />} />
            <Route path="/roundsinprogress" element={<RoundsInProgress />} />
            <Route path="/files" element={<Files />} />
            <Route path="/swagger" element={<Swagger />} />
          </Routes>
        </div>
        <Box textAlign="center" pt={{ xs: 5, sm: 10 }} pb={{ xs: 5, sm: 0 }}>
        </Box>
        <Box px={{ xs: 3, sm: 10 }} py={{ xs: 5, sm: 10 }} bgcolor="text.secondary" color="white">
          <Typography variant="h6">
            <Container maxWidth='lg'>
              <Grid container spacing={5}>
                <Grid>
                  <Box borderBottom={1}>Help</Box>
                  <Box>Contact</Box>
                  <Box>Support</Box>
                  <Box>Privacy</Box>
                </Grid>
                <Grid>
                  <Box borderBottom={1}>Help</Box>
                  <Box>Login</Box>
                  <Box>Register</Box>
                </Grid>
                <Grid>
                  <Box borderBottom={1}>Messages</Box>
                  <Box>Backup</Box>
                  <Box>History</Box>
                  <Box>Roll</Box>
                </Grid>
              </Grid>
            </Container>
            <Box textAlign="center" pt={{ xs: 5, sm: 10 }} pb={{ xs: 5, sm: 0 }}>
              QView by QuizStuff &reg; 2022-{new Date().getFullYear()}
            </Box>
          </Typography>
        </Box>
      </div >
      <CssBaseline />
    </ThemeProvider>
  )
}
