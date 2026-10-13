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

// Shown from `md` up only. On mobile the bottom nav carries Home, Help and Profile/Login, and the
// user's own profile carries Logout.
const desktopOnly = { display: { xs: 'none', md: 'inline-flex' } } as const

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
          {/* Left: brand. Centered below `md`, where it is the only thing in the bar. */}
          <Box sx={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: { xs: 'center', md: 'flex-start' } }}>
            <Link component={RouterLink} to="/" color="inherit" underline="none" sx={{ display: 'inline-flex', alignItems: 'center', gap: 1 }}>
              <Box component="img" src="/qview-logo.png" alt="" sx={{ height: 32, width: 32 }} />
              <Typography variant="h6" component="span">
                QView
              </Typography>
            </Link>
          </Box>

          {/* Center: primary navigation, centered across the full width of the bar. Below `md` the
              bottom nav (MobileBottomNav) carries Home, Help and Profile/Login instead. */}
          <Box sx={{ display: { xs: 'none', md: 'flex' }, gap: { xs: 3, sm: 5 }, justifyContent: 'center', alignItems: 'center' }}>
            <Link component={RouterLink} to="/" variant="h6" color="inherit" underline="none" sx={navLinkSx}>
              {/* <FontAwesomeIcon icon={faHomeLg} style={{ fontSize: '0.8em' }} /> */}
              Home
            </Link>
            <Link component={RouterLink} to="/help" variant="h6" color="inherit" underline="none" sx={navLinkSx}>
              {/* <FontAwesomeIcon icon={faCircleQuestion} style={{ fontSize: '0.85em' }} /> */}
              Help
            </Link>
          </Box>

          {/* Right: session actions. Every one is desktop-only, so the group is dropped below `md`
              rather than left empty, where it would still hold half the bar and push the brand
              off centre. */}
          <Box sx={{ flex: 1, display: { xs: 'none', md: 'flex' }, justifyContent: 'flex-end', alignItems: 'center', gap: 0.5 }}>
            {auth.isAuthenticated && (
              <Tooltip title="My Profile">
                <IconButton size="small" color="inherit" aria-label="My Profile" sx={desktopOnly} onClick={() => navigate(`/user/${auth.session?.userId}/overview`)}>
                  <FontAwesomeIcon icon={faUser} style={{ fontSize: '1rem' }} />
                </IconButton>
              </Tooltip>
            )}
            {auth.isAuthenticated && (
              <Tooltip title="Logout">
                <IconButton size="small" color="inherit" aria-label="Logout" sx={desktopOnly} onClick={() => auth.logout()}>
                  <FontAwesomeIcon icon={faRightFromBracket} style={{ fontSize: '1rem' }} />
                </IconButton>
              </Tooltip>
            )}
            {!auth.isAuthenticated && (
              <Tooltip title="Login">
                <IconButton size="small" color="inherit" aria-label="Login" sx={desktopOnly} onClick={() => navigate('/login')}>
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
