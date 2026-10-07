import { createContext, useContext } from 'react'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import ViewSidebarOutlinedIcon from '@mui/icons-material/ViewSidebarOutlined'

interface ProfileSidebarContextValue {
  collapsed: boolean
  toggle: () => void
}

/** Defaults let the button render harmlessly outside a ProfileLayout. */
const Context = createContext<ProfileSidebarContextValue>({ collapsed: false, toggle: () => {} })

export const ProfileSidebarProvider = Context.Provider

export const useProfileSidebar = () => useContext(Context)

/**
 * Collapses and expands a profile's sidebar.
 *
 * It lives in one place only — left of the breadcrumb path (see `ProfileBreadcrumbs`) — so it stays
 * put as the sidebar opens and closes, and a collapsed sidebar costs no column of its own.
 */
export function ProfileSidebarToggle() {
  const { collapsed, toggle } = useProfileSidebar()
  const label = collapsed ? 'Expand sidebar' : 'Collapse sidebar'

  return (
    <Tooltip title={label} placement="bottom">
      <IconButton
        size="small"
        onClick={toggle}
        aria-label={label}
        aria-expanded={!collapsed}
        sx={{
          flexShrink: 0,
          color: 'text.secondary',
          // A rounded rectangle rather than IconButton's circle, so it reads as a button.
          border: '1px solid',
          borderColor: 'divider',
          borderRadius: 1,
          p: '4px',
          bgcolor: 'background.paper',
          '&:hover': { bgcolor: 'action.hover', borderColor: 'text.disabled', color: 'text.primary' },
        }}
      >
        {/* MUI draws the panel on the right; mirror it so it reads as the left-hand sidebar. */}
        <ViewSidebarOutlinedIcon fontSize="small" sx={{ transform: 'scaleX(-1)' }} />
      </IconButton>
    </Tooltip>
  )
}
