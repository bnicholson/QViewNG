import { Navigate, useParams, Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import Typography from '@mui/material/Typography'
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import DocsSidebar from '../components/DocsSidebar'
import DocsContent from '../components/DocsContent'
import {
  adjacentArticles,
  docArticle,
  docCategoryById,
  docPath,
  isKnownVersion,
  latestVersion,
  resolveVersion,
  type DocCategoryId,
} from '../data/docs'

export const DocsArticlePage = ({ categoryId }: { categoryId: DocCategoryId }) => {
  const { version, slug } = useParams<{ version: string; slug: string }>()
  const category = docCategoryById(categoryId)
  if (!category) return <Navigate to="/help" replace />

  // An unknown version in the URL falls back to the newest published one.
  if (!isKnownVersion(category, version)) {
    return <Navigate to={docPath(category, latestVersion(category).version, slug)} replace />
  }

  const docVersion = resolveVersion(category, version)
  const article = docArticle(docVersion, slug)
  const { prev, next } = article ? adjacentArticles(docVersion, article.slug) : {}
  const basePath = docPath(category, docVersion.version)

  // Previous/Next across this version's whole hierarchy, treated as one flat list.
  const navRow = prev || next ? (
    <Box sx={{ display: 'flex', justifyContent: 'space-between', gap: 2 }}>
      {prev ? (
        <Button
          component={RouterLink}
          to={`${basePath}/${prev.slug}`}
          startIcon={<ChevronLeftIcon />}
          sx={{ textTransform: 'none', textAlign: 'left', maxWidth: '48%' }}
        >
          Previous: {prev.title}
        </Button>
      ) : <Box />}
      {next ? (
        <Button
          component={RouterLink}
          to={`${basePath}/${next.slug}`}
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
      <DocsSidebar category={category} docVersion={docVersion} activeSlug={slug} />

      <Box sx={{ flex: 1, minWidth: 0, maxWidth: 760 }}>
        {article ? (
          <>
            {/* Which manual and version this page belongs to. */}
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1.5, flexWrap: 'wrap' }}>
              <Typography
                component={RouterLink}
                to={basePath}
                variant="body2"
                sx={{ color: 'text.secondary', textDecoration: 'none', '&:hover': { color: 'primary.main' } }}
              >
                {category.title}
              </Typography>
              <Chip label={`v${docVersion.version}`} size="small" variant="outlined" />
            </Box>

            {navRow && <Box sx={{ mb: 2 }}>{navRow}</Box>}

            <Typography variant="h4" sx={{ fontWeight: 600, mb: 2 }}>
              {article.title}
            </Typography>

            <DocsContent html={article.contentHtml} category={category} version={docVersion.version} />

            {navRow && <Box sx={{ mt: 4, pt: 2, borderTop: 1, borderColor: 'divider' }}>{navRow}</Box>}
          </>
        ) : (
          <Box sx={{ py: 4 }}>
            <Typography variant="h5" sx={{ fontWeight: 500, mb: 1 }}>
              Article not found
            </Typography>
            <Typography color="text.secondary">
              That page doesn’t exist in {category.title} v{docVersion.version}.{' '}
              <RouterLink to={basePath}>Browse this manual</RouterLink> or{' '}
              <RouterLink to="/help">return to the Help Center</RouterLink>.
            </Typography>
          </Box>
        )}
      </Box>
    </Box>
  )
}
