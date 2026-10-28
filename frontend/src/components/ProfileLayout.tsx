import { useState } from 'react'
import { Navigate, useLocation } from 'react-router-dom'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import Divider from '@mui/material/Divider'
import { ProfileSidebarProvider } from '../contexts/ProfileSidebarContext'
import { usePublishMobileMenu } from '../contexts/MobileMenuContext'
import ProfileNavList from './ProfileNavList'
import { isRouteActive, isRouteNavItem, type NavItem } from '../utils/profileNav'

export type { NavItem }

const SIDEBAR_WIDTH = 184
/** Below the app bar (74px) plus a gap. */
const SIDEBAR_TOP = 'calc(74px + 16px)'
/**
 * Remembered per viewer: navigating between a profile's sections re-renders this layout, and a
 * collapse that reset on every click would be useless. Browser-only, so a failure to read or write
 * it just means the sidebar starts expanded.
 */
const SIDEBAR_COLLAPSED_KEY = 'qview_profile_sidebar_collapsed'

function readCollapsedPreference(): boolean {
  try {
    return localStorage.getItem(SIDEBAR_COLLAPSED_KEY) === '1'
  } catch {
    return false
  }
}

interface ProfileLayoutProps {
  /** The kind of domain entity, e.g. "Team". */
  entityType: string
  /** The entity's name. Omitted for profiles that are not of a single entity (the Superuser Console). */
  entityName?: string
  subtitle?: string
  navItems: NavItem[]
  children: React.ReactNode
}

export default function ProfileLayout({ entityType, entityName, subtitle, navItems, children }: ProfileLayoutProps) {
  const location = useLocation()
  usePublishMobileMenu({ kind: 'profile', entityType, entityName, navItems })
  const [collapsed, setCollapsed] = useState<boolean>(readCollapsedPreference)

  const toggleCollapsed = () => {
    setCollapsed((prev) => {
      const next = !prev
      try {
        localStorage.setItem(SIDEBAR_COLLAPSED_KEY, next ? '1' : '0')
      } catch {
        // Not remembering the preference is not worth failing the toggle over.
      }
      return next
    })
  }

  const routeItems = navItems.filter(isRouteNavItem)
  if (routeItems.length > 0 && !routeItems.some((i) => isRouteActive(location.pathname, i.to))) {
    return <Navigate to={routeItems[0].to} replace />
  }

  return (
    <ProfileSidebarProvider value={{ collapsed, toggle: toggleCollapsed }}>
      <Box sx={{ display: 'flex', gap: 3, alignItems: 'flex-start' }}>
        {/* Sidebar — omitted entirely while collapsed, so it takes no width and no flex gap. Its
            toggle lives beside the breadcrumb path, not in here, so it does not move when the
            sidebar closes. */}
        {!collapsed && (
          <Box
            component="nav"
            sx={{
              width: SIDEBAR_WIDTH,
              flexShrink: 0,
              position: 'sticky',
              top: SIDEBAR_TOP,
              display: { xs: 'none', md: 'block' },
              // Fits within the viewport and scrolls on its own, only while the mouse is over it.
              // `contain` keeps reaching its end from scrolling the page instead.
              maxHeight: `calc(100vh - ${SIDEBAR_TOP} - 16px)`,
              overflowY: 'auto',
              overscrollBehavior: 'contain',
            }}
          >
            <Typography variant="subtitle2" sx={{ fontWeight: 700, mb: 1, px: '14px', lineHeight: 1.5, color: 'text.secondary', textTransform: 'uppercase', fontSize: '0.68rem', letterSpacing: '0.08em' }}>
              {entityName === undefined ? entityType : <>{entityType}:<br />{entityName}</>}
            </Typography>
            {subtitle && (
              <Typography variant="caption" sx={{ px: '14px', color: 'text.secondary', display: 'block', mb: 1 }}>
                {subtitle}
              </Typography>
            )}
            <Divider sx={{ mb: 1 }} />
            <ProfileNavList navItems={navItems.filter((i) => !i.mobileOnly)} />
          </Box>
        )}

        {/* Main content */}
        <Box sx={{ flex: 1, minWidth: 0 }}>
          {children}
        </Box>
      </Box>
    </ProfileSidebarProvider>
  )
}
