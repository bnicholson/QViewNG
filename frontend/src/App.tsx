// import { useAuth, useAuthCheck } from './hooks/useAuth';
import React from 'react';
import './App.css';
import { useNavigate } from 'react-router-dom';
import '@mui/material/colors';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import { Backdrop } from '@mui/material';
import CssBaseline from '@mui/material/CssBaseline';
import QViewRoutes from './routes';
import DrawerGlobal from './components/DrawerGlobal';
import FooterGlobal from './components/FooterGlobal';

if (import.meta.env.NODE_ENV === 'development') import('./setupDevelopment')

const drawerWidth = 240;

export default function App() {
  const navigate = useNavigate()
  // useAuthCheck()
  // const auth = useAuth()

  const [drawerIsOpen, setDrawerIsOpen] = React.useState(false);

  const toggleDrawer = () => setDrawerIsOpen(!drawerIsOpen)

  // @ts-ignore
  return (
    <>
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
          <DrawerGlobal
            drawerWidth={drawerWidth}
            drawerIsOpen={drawerIsOpen}
            toggleDrawer={toggleDrawer}/>
        </Backdrop>
        <div style={{ margin: '0 auto', maxWidth: '1200px' }}>
          <QViewRoutes/>
        </div>
        <Box textAlign="center" pt={{ xs: 5, sm: 10 }} pb={{ xs: 5, sm: 0 }}>
        </Box>
        <FooterGlobal/>
      </div >
      <CssBaseline />
    </>
  )
}
