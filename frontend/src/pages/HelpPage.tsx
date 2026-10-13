import { Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Chip from '@mui/material/Chip'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import { DEFAULT_CATEGORY_ID, docCategories, docPath, latestVersion } from '../data/docs'
import { usePublishMobileMenu } from '../contexts/MobileMenuContext'

export const HelpPage = () => {
  // On mobile the Menu sheet browses the manuals, starting with QView's.
  usePublishMobileMenu({ kind: 'docs', categoryId: DEFAULT_CATEGORY_ID })

  return (
    <Box sx={{ maxWidth: 1040, mx: 'auto' }}>
      {/* Hero */}
      <Box sx={{ textAlign: 'center', py: { xs: 4, sm: 6 } }}>
        <Typography variant="h3" sx={{ fontWeight: 600, mb: 1 }}>
          Help Center
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ maxWidth: 640, mx: 'auto' }}>
          Documentation for the QView web app and for the QuizMachine and QMServer desktop
          programs. Choose a product below to browse its guide.
        </Typography>
      </Box>

      {/* One card per documentation category. */}
      <Box
        sx={{
          display: 'grid',
          gridTemplateColumns: { xs: '1fr', md: 'repeat(3, 1fr)' },
          gap: 2,
          pb: 6,
          textAlign: 'left',
        }}
      >
        {docCategories.map((category) => {
          const newest = latestVersion(category)
          return (
            <Paper
              key={category.id}
              component={RouterLink}
              to={docPath(category, newest.version)}
              variant="outlined"
              sx={{
                p: 2.5,
                borderRadius: 2,
                display: 'flex',
                flexDirection: 'column',
                textDecoration: 'none',
                color: 'inherit',
                transition: 'border-color 120ms, box-shadow 120ms',
                '&:hover': { borderColor: 'primary.main', boxShadow: 2 },
              }}
            >
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1, flexWrap: 'wrap' }}>
                <Typography variant="h6" sx={{ fontWeight: 600 }}>
                  {category.title}
                </Typography>
                <Chip label={`v${newest.version}`} size="small" variant="outlined" />
              </Box>

              <Typography variant="body2" color="text.secondary" sx={{ flex: 1, mb: 2 }}>
                {category.description}
              </Typography>

              <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, color: 'primary.main' }}>
                <Typography variant="body2" sx={{ fontWeight: 600 }}>
                  Browse documentation
                </Typography>
                <ChevronRightIcon fontSize="small" />
              </Box>
            </Paper>
          )
        })}
      </Box>
    </Box>
  )
}
