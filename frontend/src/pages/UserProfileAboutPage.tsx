import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import QViewAttribution from '../components/QViewAttribution'

/**
 * About QView: the attribution and version line the footer carries. A mobile-only profile section,
 * since the footer is hidden on mobile and this is otherwise the only place to see which version
 * of QView is running.
 */
export const UserProfileAboutPage = () => {
  return (
    <Box>
      <Typography variant="h5" component="h1" sx={{ fontWeight: 600, mb: 1.5 }}>
        About QView
      </Typography>
      <Typography variant="body1" color="text.secondary">
        <QViewAttribution />
      </Typography>
    </Box>
  )
}
