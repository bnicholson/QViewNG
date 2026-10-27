import { useEffect, useState } from 'react'
import { Navigate, useLocation, useParams, Link as RouterLink } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import CircularProgress from '@mui/material/CircularProgress'
import Typography from '@mui/material/Typography'
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import LockOutlinedIcon from '@mui/icons-material/LockOutlined'
import DocsSidebar from '../components/DocsSidebar'
import DocsContent from '../components/DocsContent'
import { useAuth } from '../hooks/useAuth'
import { HelpDocAPI } from '../features/HelpDocAPI'
import {
  adjacentArticles,
  articleRequiresLogin,
  docArticle,
  docCategoryById,
  docPath,
  isKnownVersion,
  latestVersion,
  resolveVersion,
  type DocCategory,
  type DocCategoryId,
} from '../data/docs'

const CheckingAuth = ({ label }: { label: string }) => (
  <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 3 }}>
    <CircularProgress size={20} />
    <Typography color="text.secondary">{label}</Typography>
  </Box>
)

/** A members-only article whose content the backend serves to logged-in users only. */
const MembersOnlyContent = ({ category, version, slug }: { category: DocCategory; version: string; slug: string }) => {
  const { accessToken } = useAuth()
  const [loaded, setLoaded] = useState<{ key: string; html?: string; error?: string } | null>(null)
  const key = `${category.id}/${version}/${slug}`

  useEffect(() => {
    if (!accessToken) return
    let cancelled = false
    HelpDocAPI.getArticle(category.id, version, slug, accessToken)
      .then((html) => { if (!cancelled) setLoaded({ key, html }) })
      .catch((e: Error) => { if (!cancelled) setLoaded({ key, error: e.message }) })
    return () => { cancelled = true }
  }, [category.id, version, slug, key, accessToken])

  // Ignore a result left over from the previous article while the next one loads.
  if (!loaded || loaded.key !== key) return <CheckingAuth label="Loading article…" />
  if (loaded.error) {
    return (
      <Alert severity="error" sx={{ my: 2 }}>
        This article couldn’t be loaded. Please refresh the page to try again.
      </Alert>
    )
  }
  return <DocsContent html={loaded.html ?? ''} category={category} version={version} />
}

export const DocsArticlePage = ({ categoryId }: { categoryId: DocCategoryId }) => {
  const { version, slug } = useParams<{ version: string; slug: string }>()
  const { isAuthenticated, isCheckingAuth } = useAuth()
  const location = useLocation()
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
  const locked = !!article && !isAuthenticated && articleRequiresLogin(docVersion, article.slug)

  // Previous/Next across this version's whole hierarchy, treated as one flat list. The copy at the
  // bottom of the article also scrolls back to the top, so the next article starts at its title.
  const navRow = (scrollToTop: boolean) => prev || next ? (
    <Box sx={{ display: 'flex', justifyContent: 'space-between', gap: 2 }}>
      {prev ? (
        <Button
          component={RouterLink}
          to={`${basePath}/${prev.slug}`}
          onClick={scrollToTop ? () => window.scrollTo(0, 0) : undefined}
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
          onClick={scrollToTop ? () => window.scrollTo(0, 0) : undefined}
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

            {(prev || next) && <Box sx={{ mb: 2 }}>{navRow(false)}</Box>}

            <Typography variant="h4" sx={{ fontWeight: 600, mb: 2 }}>
              {article.title}
            </Typography>

            {!locked ? (
              article.contentHtml !== undefined ? (
                <DocsContent html={article.contentHtml} category={category} version={docVersion.version} />
              ) : (
                <MembersOnlyContent category={category} version={docVersion.version} slug={article.slug} />
              )
            ) : isCheckingAuth ? (
              <CheckingAuth label="Checking authentication…" />
            ) : (
              // Members-only article: the title stays visible, the content needs any QView login.
              <Alert severity="info" icon={<LockOutlinedIcon />} sx={{ my: 2 }}>
                <Typography sx={{ mb: 1.5 }}>
                  You need to be logged in to read this article. Any QView account will do, and creating one is
                  free.
                </Typography>
                <Box sx={{ display: 'flex', gap: 1.5, flexWrap: 'wrap' }}>
                  <Button variant="contained" component={RouterLink} to="/login" state={{ from: location.pathname }}>
                    Log In
                  </Button>
                  <Button variant="outlined" component={RouterLink} to="/register">
                    Create an Account
                  </Button>
                </Box>
              </Alert>
            )}

            {(prev || next) && <Box sx={{ mt: 4, pt: 2, borderTop: 1, borderColor: 'divider' }}>{navRow(true)}</Box>}
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
