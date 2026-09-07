import { useParams, Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import HelpSidebar from '../components/HelpSidebar'
import { kbArticleBySlug } from '../data/knowledgeBase'

export const HelpArticlePage = () => {
  const { slug } = useParams<{ slug: string }>()
  const article = slug ? kbArticleBySlug(slug) : undefined

  return (
    <Box sx={{ display: 'flex', gap: 4, alignItems: 'flex-start', textAlign: 'left' }}>
      <HelpSidebar activeSlug={slug} />

      <Box sx={{ flex: 1, minWidth: 0, maxWidth: 760 }}>
        {article ? (
          <>
            <Typography variant="h4" sx={{ fontWeight: 600, mb: 2 }}>
              {article.title}
            </Typography>
            {/* Content is trusted placeholder HTML for now (authored HTML later). */}
            <Box
              sx={{
                color: 'text.primary',
                lineHeight: 1.7,
                '& h2': { fontSize: '1.25rem', fontWeight: 600, mt: 3, mb: 1 },
                '& h3': { fontSize: '1.05rem', fontWeight: 600, mt: 2, mb: 1 },
                '& p': { mb: 1.5 },
                '& ul, & ol': { pl: 3, mb: 1.5 },
                '& li': { mb: 0.5 },
                '& a': { color: 'primary.main' },
                '& code': { fontFamily: 'monospace', bgcolor: 'action.hover', px: 0.5, borderRadius: 0.5 },
              }}
              dangerouslySetInnerHTML={{ __html: article.contentHtml }}
            />
          </>
        ) : (
          <Box sx={{ py: 4 }}>
            <Typography variant="h5" sx={{ fontWeight: 500, mb: 1 }}>
              Article not found
            </Typography>
            <Typography color="text.secondary">
              That help article doesn’t exist. <RouterLink to="/help">Return to the Help Center</RouterLink>.
            </Typography>
          </Box>
        )}
      </Box>
    </Box>
  )
}
