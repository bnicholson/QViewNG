import React from 'react'
import { Link as RouterLink, useNavigate } from 'react-router-dom'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import MenuIcon from '@mui/icons-material/Menu'
import { Backdrop, Link, Tooltip } from '@mui/material'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCircleQuestion, faHomeLg, faRightFromBracket, faRightToBracket, faUser } from '@fortawesome/free-solid-svg-icons'
import { useAuth } from '../hooks/useAuth'
import DrawerGlobal from './DrawerGlobal'

const drawerWidth = 240

// Modern text-style nav link (not a button): matches the "QView" brand typography (h6,
// normal case) with an animated underline that reveals on hover.
const navLinkSx = {
  color: 'inherit',
  cursor: 'pointer',
  position: 'relative',
  display: 'inline-flex',
  alignItems: 'center',
  gap: 0.75,
  py: 0.5,
  '&::after': {
    content: '""',
    position: 'absolute',
    left: 0,
    bottom: 0,
    height: '2px',
    width: '100%',
    backgroundColor: 'currentColor',
    transform: 'scaleX(0)',
    transformOrigin: 'center',
    transition: 'transform .18s ease',
  },
  '&:hover::after': { transform: 'scaleX(1)' },
} as const

export default function GlobalNavBar() {
  const navigate = useNavigate()
  const auth = useAuth()
  const [drawerIsOpen, setDrawerIsOpen] = React.useState(false)
  const toggleDrawer = () => setDrawerIsOpen(!drawerIsOpen)

  return (
    <>
      <AppBar position="fixed" sx={{ width: '100%' }}>
        <Toolbar sx={{ width: '100%' }}>
          {/* Left: brand */}
          <Box sx={{ flex: 1, display: 'flex', alignItems: 'center' }}>
            <Typography variant="h6" component="div" onClick={() => navigate('/')} style={{ cursor: 'pointer' }}>
              QView
            </Typography>
          </Box>

          {/* Center: primary navigation, centered across the full width of the bar */}
          <Box sx={{ display: 'flex', gap: { xs: 3, sm: 5 }, justifyContent: 'center', alignItems: 'center' }}>
            <Link component={RouterLink} to="/" variant="h6" color="inherit" underline="none" sx={navLinkSx}>
              {/* <FontAwesomeIcon icon={faHomeLg} style={{ fontSize: '0.8em' }} /> */}
              Home
            </Link>
            <Link component={RouterLink} to="/help" variant="h6" color="inherit" underline="none" sx={navLinkSx}>
              {/* <FontAwesomeIcon icon={faCircleQuestion} style={{ fontSize: '0.85em' }} /> */}
              Help
            </Link>
          </Box>

          {/* Right: session actions */}
          <Box sx={{ flex: 1, display: 'flex', justifyContent: 'flex-end', alignItems: 'center', gap: 0.5 }}>
            {auth.isAuthenticated && (
              <Tooltip title="My Profile">
                <IconButton size="small" color="inherit" aria-label="My Profile" onClick={() => navigate(`/user/${auth.session?.userId}/overview`)}>
                  <FontAwesomeIcon icon={faUser} style={{ fontSize: '1rem' }} />
                </IconButton>
              </Tooltip>
            )}
            {auth.isAuthenticated && (
              <Tooltip title="Logout">
                <IconButton size="small" color="inherit" aria-label="Logout" onClick={() => auth.logout()}>
                  <FontAwesomeIcon icon={faRightFromBracket} style={{ fontSize: '1rem' }} />
                </IconButton>
              </Tooltip>
            )}
            {!auth.isAuthenticated && (
              <Tooltip title="Login">
                <IconButton size="small" color="inherit" aria-label="Login" onClick={() => navigate('/login')}>
                  <FontAwesomeIcon icon={faRightToBracket} style={{ fontSize: '1rem' }} />
                </IconButton>
              </Tooltip>
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
