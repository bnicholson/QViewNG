import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import QViewAttribution from '../components/QViewAttribution'
import { AccountsLinkSection, ExternalLinkSection } from '../components/LinkSections'
import { EXTERNAL_LINK_SECTIONS } from '../components/ExternalLinks'

/**
 * About QView: the attribution, version line, outside links and account links the footer carries. A mobile-only profile section,
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

      {EXTERNAL_LINK_SECTIONS.map(section => (
        <Box key={section.heading} sx={{ mt: 3 }}>
          <ExternalLinkSection heading={section.heading} links={section.links} />
        </Box>
      ))}
      <Box sx={{ mt: 3 }}>
        <AccountsLinkSection />
      </Box>
    </Box>
  )
}
