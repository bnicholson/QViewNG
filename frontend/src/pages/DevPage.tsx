import { useNavigate } from 'react-router-dom'
import { Navigate } from 'react-router'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import List from '@mui/material/List'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemText from '@mui/material/ListItemText'
import Divider from '@mui/material/Divider'
import { useAuth } from '../hooks/useAuth'

interface DevLink {
  label: string
  path: string
  description: string
}

const devLinks: DevLink[] = [
  { label: 'CRM',                path: '/crm',                description: 'User and tournament applicant management' },
  { label: 'Tournaments Page',   path: '/tournaments-page',   description: 'Tournaments list page'                   },
  { label: 'TD Editor',          path: '/tdeditor',           description: 'Legacy tournament director editor'        },
  { label: 'Rounds In Progress', path: '/rounds-in-progress', description: 'View rounds currently in progress'       },
  { label: 'Files',              path: '/files',              description: 'File management'                         },
  { label: 'Swagger',            path: '/swagger',            description: 'API documentation and interactive tester' },
  { label: '404',                path: '/404',                description: 'Not found page'                          },
]

export function DevPage() {
  const { session } = useAuth()
  const navigate = useNavigate()

  if (!session?.hasRole('super_user')) return <Navigate to="/404" replace />

  return (
    <Box sx={{ maxWidth: 600, mx: 'auto', mt: 4, px: 2 }}>
      <Typography variant="h5" fontWeight={700} gutterBottom>
        Developer Links
      </Typography>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
        Convenience links to internal and legacy pages. Visible to superusers only.
      </Typography>
      <Divider sx={{ mb: 1 }} />
      <List disablePadding>
        {devLinks.map(({ label, path, description }) => (
          <ListItemButton key={path} onClick={() => navigate(path)} sx={{ borderRadius: 1, mb: 0.5 }}>
            <ListItemText
              primary={label}
              secondary={description}
              primaryTypographyProps={{ fontWeight: 600 }}
            />
          </ListItemButton>
        ))}
      </List>
    </Box>
  )
}
