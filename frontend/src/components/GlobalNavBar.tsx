import React from 'react'
import { useNavigate } from 'react-router-dom'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import MenuIcon from '@mui/icons-material/Menu'
import { Backdrop, Button } from '@mui/material'
import { useAuth } from '../hooks/useAuth'
import DrawerGlobal from './DrawerGlobal'

const drawerWidth = 240

export default function GlobalNavBar() {
  const navigate = useNavigate()
  const auth = useAuth()
  const [drawerIsOpen, setDrawerIsOpen] = React.useState(false)
  const toggleDrawer = () => setDrawerIsOpen(!drawerIsOpen)

  return (
    <>
      <AppBar position="fixed">
        <Toolbar>
          {/* 
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
            onClick={toggleDrawer}
          >
            <MenuIcon />
          </IconButton> */}
          <Typography variant="h6" component="div" onClick={() => navigate('/')} style={{ cursor: 'pointer' }}>
            QView
          </Typography>
          <Typography variant="h6" component="div" sx={{ flexGrow: 2 }} onClick={toggleDrawer} />
          <Box sx={{ flexGrow: 1, display: 'flex', justifyContent: 'flex-end', gap: 1 }}>
            {auth.session?.hasRole('super_user') && (
              <Button variant="contained" color="inherit" sx={{ color: 'primary.main' }} onClick={() => navigate('/dev')}>
                Dev
              </Button>
            )}
            {auth.isAuthenticated && (
              <Button variant="contained" color="inherit" sx={{ color: 'primary.main' }} onClick={() => navigate(`/user/${auth.session?.userId}/overview`)}>
                My Profile
              </Button>
            )}
            {auth.isAuthenticated && (
              <Button variant="contained" color="inherit" sx={{ color: 'primary.main' }} onClick={() => auth.logout()}>
                Logout
              </Button>
            )}
            {!auth.isAuthenticated && (
              <Button variant="contained" color="inherit" sx={{ color: 'primary.main' }} onClick={() => navigate('/login')}>
                Login / Register
              </Button>
            )}
          </Box>
        </Toolbar>
      </AppBar>
      <Backdrop onClick={toggleDrawer} open={drawerIsOpen} sx={{ zIndex: 1 }}>
        <DrawerGlobal
          drawerWidth={drawerWidth}
          drawerIsOpen={drawerIsOpen}
          toggleDrawer={toggleDrawer}
        />
      </Backdrop>
    </>
  )
}
