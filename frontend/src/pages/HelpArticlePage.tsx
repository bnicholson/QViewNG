import { useParams, Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import HelpSidebar from '../components/HelpSidebar'
import { kbArticleBySlug, kbAdjacentArticles } from '../data/knowledgeBase'

export const HelpArticlePage = () => {
  const { slug } = useParams<{ slug: string }>()
  const article = slug ? kbArticleBySlug(slug) : undefined
  const { prev, next } = article ? kbAdjacentArticles(article.slug) : {}

  // Previous/Next across the whole article hierarchy, treated as one flat list. Rendered above the
  // title and below the content so readers can page through without returning to the sidebar.
  const navRow = (prev || next) ? (
    <Box sx={{ display: 'flex', justifyContent: 'space-between', gap: 2 }}>
      {prev ? (
        <Button
          component={RouterLink}
          to={`/help/${prev.slug}`}
          startIcon={<ChevronLeftIcon />}
          sx={{ textTransform: 'none', textAlign: 'left', maxWidth: '48%' }}
        >
          Previous: {prev.title}
        </Button>
      ) : <Box />}
      {next ? (
        <Button
          component={RouterLink}
          to={`/help/${next.slug}`}
          endIcon={<ChevronRightIcon />}
          sx={{ textTransform: 'none', textAlign: 'right', maxWidth: '48%', ml: 'auto' }}
        >
          Next: {next.title}
        </Button>
      ) : <Box />}
    </Box>
  ) : null

  return (
    <Box sx={{ display: 'flex', gap: 4, alignItems: 'flex-start', textAlign: 'left' }}>
      <HelpSidebar activeSlug={slug} />

      <Box sx={{ flex: 1, minWidth: 0, maxWidth: 760 }}>
        {article ? (
          <>
            {navRow && <Box sx={{ mb: 2 }}>{navRow}</Box>}

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

            {navRow && (
              <Box sx={{ mt: 4, pt: 2, borderTop: 1, borderColor: 'divider' }}>{navRow}</Box>
            )}
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
