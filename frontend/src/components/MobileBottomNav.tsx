import { useRef, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import BottomNavigation from '@mui/material/BottomNavigation'
import BottomNavigationAction from '@mui/material/BottomNavigationAction'
import Divider from '@mui/material/Divider'
import Drawer from '@mui/material/Drawer'
import Paper from '@mui/material/Paper'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faBars, faCircleQuestion, faHomeLg, faRightToBracket, faUser } from '@fortawesome/free-solid-svg-icons'
import { useAuth } from '../hooks/useAuth'
import { useMobileMenu } from '../contexts/MobileMenuContext'
import DocsMenuPanel from './DocsMenuPanel'
import MenuSheetHeader from './MenuSheetHeader'
import ProfileNavList from './ProfileNavList'
import { isRouteActive } from '../utils/profileNav'

/** BottomNavigation's own height; the spacer reserves the same. */
const BAR_HEIGHT = 56

/** Shown below `md` only — the breakpoint where the profile sidebar disappears. */
const mobileOnly = { display: { xs: 'flex', md: 'none' } } as const

const iconStyle = { fontSize: '1.25rem' }

/**
 * Mobile-only navigation fixed to the bottom of the screen: Profile (Login when signed out), Home,
 * Help and Menu. Menu opens a sheet that slides up with whatever the page offers (see
 * `usePublishMobileMenu`) — a profile's sections, or the Help Center's manuals — and is disabled on
 * pages that offer nothing.
 */
export default function MobileBottomNav() {
  const location = useLocation()
  const navigate = useNavigate()
  const auth = useAuth()
  const menu = useMobileMenu()
  // The path the sheet was opened on. Any navigation, from the sheet or elsewhere, leaves that path
  // and so dismisses it.
  const [menuOpenedOn, setMenuOpenedOn] = useState<string | null>(null)
  const menuOpen = menuOpenedOn === location.pathname
  // Work to run once the sheet has finished sliding away, e.g. scrolling to an in-page section,
  // which the sheet's scroll lock would otherwise swallow.
  const afterCloseRef = useRef<(() => void) | undefined>(undefined)

  const myProfileBase = auth.isAuthenticated ? `/user/${auth.session?.userId}` : null

  const pathname = location.pathname
  const value =
    pathname === '/' ? 'home'
    // The manuals live outside /help (/qview-docs/…), but publish a docs menu.
    : isRouteActive(pathname, '/help') || menu?.kind === 'docs' ? 'help'
    : myProfileBase && isRouteActive(pathname, myProfileBase) ? 'profile'
    : !myProfileBase && pathname === '/login' ? 'profile'
    : null

  const closeMenu = (then?: () => void) => {
    afterCloseRef.current = then
    setMenuOpenedOn(null)
  }

  return (
    <>
      {/* Reserves room at the end of the page so the fixed bar never covers it. */}
      <Box sx={{ ...mobileOnly, height: `calc(${BAR_HEIGHT}px + env(safe-area-inset-bottom))` }} />
      <Paper
        elevation={8}
        square
        sx={{
          ...mobileOnly,
          position: 'fixed',
          left: 0,
          right: 0,
          bottom: 0,
          zIndex: 'appBar',
          // Keep the buttons clear of the home indicator on notched phones.
          pb: 'env(safe-area-inset-bottom)',
        }}
      >
        <BottomNavigation showLabels value={value} sx={{ width: '100%', height: BAR_HEIGHT }}>
          {myProfileBase ? (
            <BottomNavigationAction
              label="Profile"
              value="profile"
              icon={<FontAwesomeIcon icon={faUser} style={iconStyle} />}
              onClick={() => navigate(`${myProfileBase}/overview`)}
            />
          ) : (
            <BottomNavigationAction
              label="Login"
              value="profile"
              icon={<FontAwesomeIcon icon={faRightToBracket} style={iconStyle} />}
              onClick={() => navigate('/login')}
            />
          )}
          <BottomNavigationAction
            label="Home"
            value="home"
            icon={<FontAwesomeIcon icon={faHomeLg} style={iconStyle} />}
            onClick={() => navigate('/')}
          />
          <BottomNavigationAction
            label="Help"
            value="help"
            icon={<FontAwesomeIcon icon={faCircleQuestion} style={iconStyle} />}
            onClick={() => navigate('/help')}
          />
          <BottomNavigationAction
            label="Menu"
            value="menu"
            icon={<FontAwesomeIcon icon={faBars} style={iconStyle} />}
            disabled={!menu}
            aria-haspopup="dialog"
            aria-expanded={menuOpen}
            onClick={() => setMenuOpenedOn(location.pathname)}
            sx={{ '&.Mui-disabled': { opacity: 0.38 } }}
          />
        </BottomNavigation>
      </Paper>

      <Drawer
        anchor="bottom"
        open={menuOpen && !!menu}
        onClose={() => closeMenu()}
        slotProps={{
          transition: {
            onExited: () => {
              afterCloseRef.current?.()
              afterCloseRef.current = undefined
            },
          },
          paper: {
            sx: {
              borderTopLeftRadius: 16,
              borderTopRightRadius: 16,
              maxHeight: '80vh',
              // The docs sheet's content changes size as manuals are switched and folders opened;
              // a fixed height keeps it from jumping under the reader's thumb.
              height: menu?.kind === 'docs' ? '80vh' : undefined,
              pb: 'env(safe-area-inset-bottom)',
            },
          },
        }}
      >
        {/* Grab handle — a visual cue that this is a sheet. */}
        <Box sx={{ width: 36, height: 4, borderRadius: 2, bgcolor: 'divider', mx: 'auto', mt: 1, flexShrink: 0 }} />
        {menu?.kind === 'profile' && (
          <>
            <MenuSheetHeader
              overline={`${menu.entityType} Profile${menu.entityName !== undefined ? ':' : ''}`}
              title={menu.entityName}
              onClose={() => closeMenu()}
            />
            <Divider />
            <Box sx={{ overflowY: 'auto', py: 0.5 }}>
              <ProfileNavList navItems={menu.navItems} onSelect={closeMenu} touch />
            </Box>
          </>
        )}
        {menu?.kind === 'docs' && <DocsMenuPanel menu={menu} onClose={() => closeMenu()} />}
      </Drawer>
    </>
  )
}
