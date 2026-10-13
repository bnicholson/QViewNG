import { useEffect, useState } from 'react'
import { Link, useLocation } from 'react-router-dom'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import List from '@mui/material/List'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemText from '@mui/material/ListItemText'
import { useProfileHistory } from '../contexts/ProfileHistoryContext'
import { isRouteActive, type AnchorNavItem, type NavItem } from '../utils/profileNav'

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

interface ProfileNavListProps {
  navItems: NavItem[]
  /**
   * Called when an item (or Exit Profile) is chosen. An anchor item passes its scroll as `then`, so
   * a container that is closing — the mobile sheet — can run it once it is out of the way. Without
   * this prop the scroll runs immediately.
   */
  onSelect?: (then?: () => void) => void
  /** Larger touch targets, for the mobile sheet. */
  touch?: boolean
}

/**
 * A profile's section links, with Exit Profile above them when there is somewhere to exit to.
 * Shared by the desktop sidebar and the mobile menu sheet.
 */
export default function ProfileNavList({ navItems, onSelect, touch = false }: ProfileNavListProps) {
  const location = useLocation()
  const { exitUrl, exit } = useProfileHistory()
  const anchors = navItems.filter((i): i is AnchorNavItem => i.kind === 'anchor').map((i) => i.anchor)
  const activeAnchor = useActiveAnchor(anchors)

  const select = (then?: () => void) => {
    if (onSelect) onSelect(then)
    else then?.()
  }

  const scrollToAnchor = (anchor: string) => {
    const el = document.getElementById(anchor)
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }

  const fontSize = touch ? '1rem' : '0.875rem'
  const py = touch ? '10px' : '3px'

  return (
    <>
      {exitUrl && (
        <>
          <Button
            size="small"
            onClick={() => { select(); exit() }}
            sx={{ px: '10px', py, justifyContent: 'flex-start', color: 'text.secondary', fontSize, textTransform: 'none', width: '100%' }}
          >
            &laquo; Exit Profile
          </Button>
          <Divider sx={{ mb: 0.5 }} />
        </>
      )}
      <List dense disablePadding>
        {navItems.map((item) => {
          const isActive =
            item.kind === 'route'
              ? isRouteActive(location.pathname, item.to)
              : activeAnchor === item.anchor

          const commonSx = {
            pl: isActive ? '7px' : '10px',
            py,
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
                onClick={() => select()}
                sx={commonSx}
                selected={false}
                aria-current={isActive ? 'page' : undefined}
              >
                <ListItemText
                  primary={item.label}
                  primaryTypographyProps={{ fontSize, fontWeight: touch && isActive ? 600 : undefined }}
                />
              </ListItemButton>
            )
          }

          return (
            <ListItemButton
              key={item.anchor}
              onClick={() => select(() => scrollToAnchor(item.anchor))}
              sx={commonSx}
              selected={false}
            >
              <ListItemText
                primary={item.label}
                primaryTypographyProps={{ fontSize, fontWeight: touch && isActive ? 600 : undefined, sx: { whiteSpace: 'pre-line' } }}
              />
            </ListItemButton>
          )
        })}
      </List>
    </>
  )
}
