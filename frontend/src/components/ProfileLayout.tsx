import { useEffect, useState } from 'react'
import { Link, Navigate, useLocation } from 'react-router-dom'
import Box from '@mui/material/Box'
import List from '@mui/material/List'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemText from '@mui/material/ListItemText'
import Typography from '@mui/material/Typography'
import Divider from '@mui/material/Divider'

type RouteNavItem = {
  kind: 'route'
  label: string
  to: string
}

type AnchorNavItem = {
  kind: 'anchor'
  label: string
  anchor: string
}

export type NavItem = RouteNavItem | AnchorNavItem

interface ProfileLayoutProps {
  title: string
  subtitle?: string
  navItems: NavItem[]
  children: React.ReactNode
}

function useActiveAnchor(anchors: string[]): string | null {
  const [activeAnchor, setActiveAnchor] = useState<string | null>(anchors[0] ?? null)

  useEffect(() => {
    if (anchors.length === 0) return

    const observers: IntersectionObserver[] = []

    anchors.forEach((anchor) => {
      const el = document.getElementById(anchor)
      if (!el) return

      const observer = new IntersectionObserver(
        ([entry]) => {
          if (entry.isIntersecting) {
            setActiveAnchor(anchor)
          }
        },
        { rootMargin: '-10% 0px -60% 0px', threshold: 0 }
      )

      observer.observe(el)
      observers.push(observer)
    })

    return () => observers.forEach((o) => o.disconnect())
  }, [anchors.join(',')])

  return activeAnchor
}

export default function ProfileLayout({ title, subtitle, navItems, children }: ProfileLayoutProps) {
  const location = useLocation()
  const anchors = navItems.filter((i): i is AnchorNavItem => i.kind === 'anchor').map((i) => i.anchor)
  const activeAnchor = useActiveAnchor(anchors)

  const isRouteActive = (to: string) => {
    return location.pathname === to || location.pathname.startsWith(to + '/')
  }

  const routeItems = navItems.filter((i): i is RouteNavItem => i.kind === 'route')
  if (routeItems.length > 0 && !routeItems.some((i) => isRouteActive(i.to))) {
    return <Navigate to={routeItems[0].to} replace />
  }

  const scrollToAnchor = (anchor: string) => {
    const el = document.getElementById(anchor)
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }

  return (
    <Box sx={{ display: 'flex', gap: 2, alignItems: 'flex-start' }}>
      {/* Sidebar */}
      <Box
        component="nav"
        sx={{
          width: 148,
          flexShrink: 0,
          position: 'sticky',
          top: 'calc(74px + 16px)',
          display: { xs: 'none', md: 'block' },
        }}
      >
        <Typography variant="subtitle2" sx={{ fontWeight: 700, mb: 0.5, px: '10px', color: 'text.secondary', textTransform: 'uppercase', fontSize: '0.68rem', letterSpacing: '0.08em' }}>
          {title}
        </Typography>
        {subtitle && (
          <Typography variant="caption" sx={{ px: '10px', color: 'text.secondary', display: 'block', mb: 0.5 }}>
            {subtitle}
          </Typography>
        )}
        <Divider sx={{ mb: 0.5 }} />
        <List dense disablePadding>
          {navItems.map((item) => {
            const isActive =
              item.kind === 'route'
                ? isRouteActive(item.to)
                : activeAnchor === item.anchor

            const commonSx = {
              pl: isActive ? '7px' : '10px',
              py: '3px',
              borderLeft: isActive ? '3px solid' : '3px solid transparent',
              borderColor: isActive ? 'primary.main' : 'transparent',
              borderRadius: 0,
              '&:hover': { bgcolor: 'action.hover' },
            }

            if (item.kind === 'route') {
              return (
                <ListItemButton
                  key={item.to}
                  component={Link}
                  to={item.to}
                  sx={commonSx}
                  selected={false}
                >
                  <ListItemText
                    primary={item.label}
                    primaryTypographyProps={{ fontSize: '0.875rem' }}
                  />
                </ListItemButton>
              )
            }

            return (
              <ListItemButton
                key={item.anchor}
                onClick={() => scrollToAnchor(item.anchor)}
                sx={commonSx}
                selected={false}
              >
                <ListItemText
                  primary={item.label}
                  primaryTypographyProps={{ fontSize: '0.875rem' }}
                />
              </ListItemButton>
            )
          })}
        </List>
      </Box>

      {/* Main content */}
      <Box sx={{ flex: 1, minWidth: 0 }}>
        {children}
      </Box>
    </Box>
  )
}
