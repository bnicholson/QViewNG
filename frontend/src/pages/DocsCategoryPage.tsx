import { Navigate, useParams, Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Chip from '@mui/material/Chip'
import Typography from '@mui/material/Typography'
import DocsSidebar from '../components/DocsSidebar'
import {
  docPath,
  docCategoryById,
  isKnownVersion,
  latestVersion,
  resolveVersion,
  type DocCategoryId,
  type DocNavNode,
} from '../data/docs'

/** The table of contents for one manual version — a flat render of its nav tree. */
function Contents({ nodes, basePath, depth = 0 }: { nodes: DocNavNode[]; basePath: string; depth?: number }) {
  return (
    <Box component="ul" sx={{ listStyle: 'none', m: 0, p: 0, pl: depth ? 2 : 0 }}>
      {nodes.map((node) => (
        <Box component="li" key={node.slug ?? node.title} sx={{ mb: 0.5 }}>
          {node.slug ? (
            <Typography
              component={RouterLink}
              to={`${basePath}/${node.slug}`}
              sx={{
                color: 'primary.main',
                textDecoration: 'none',
                fontSize: '0.925rem',
                '&:hover': { textDecoration: 'underline' },
              }}
            >
              {node.title}
            </Typography>
          ) : (
            <Typography sx={{ fontWeight: 600, fontSize: '0.925rem', mt: depth ? 1 : 1.5 }}>
              {node.title}
            </Typography>
          )}
          {node.children && <Contents nodes={node.children} basePath={basePath} depth={depth + 1} />}
        </Box>
      ))}
    </Box>
  )
}

export const DocsCategoryPage = ({ categoryId }: { categoryId: DocCategoryId }) => {
  const { version } = useParams<{ version?: string }>()
  const category = docCategoryById(categoryId)
  if (!category) return <Navigate to="/help" replace />

  // `/{category}` (and any unknown version) resolves to the newest published manual, so the
  // version always appears in the URL directly after the category.
  if (!isKnownVersion(category, version)) {
    return <Navigate to={docPath(category, latestVersion(category).version)} replace />
  }

  const docVersion = resolveVersion(category, version)
  const basePath = docPath(category, docVersion.version)

  return (
    <Box sx={{ display: 'flex', gap: 4, alignItems: 'flex-start', textAlign: 'left' }}>
      <DocsSidebar category={category} docVersion={docVersion} />

      <Box sx={{ flex: 1, minWidth: 0, maxWidth: 760 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1, flexWrap: 'wrap' }}>
          <Typography variant="h4" sx={{ fontWeight: 600 }}>
            {category.title} Documentation
          </Typography>
          <Chip label={`v${docVersion.version}`} size="small" variant="outlined" />
        </Box>

        <Typography color="text.secondary" sx={{ mb: docVersion.released ? 0.5 : 3 }}>
          {category.description}
        </Typography>
        {docVersion.released && (
          <Typography variant="body2" color="text.secondary" sx={{ mb: 3 }}>
            Released {docVersion.released}
          </Typography>
        )}

        <Typography variant="h6" sx={{ fontWeight: 600, mb: 1 }}>
          Table of Contents
        </Typography>
        <Contents nodes={docVersion.nav} basePath={basePath} />
      </Box>
    </Box>
  )
}
