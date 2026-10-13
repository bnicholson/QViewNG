import { Link as RouterLink, useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Divider from '@mui/material/Divider'
import FormControl from '@mui/material/FormControl'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select, { type SelectChangeEvent } from '@mui/material/Select'
import Typography from '@mui/material/Typography'
import {
  docCategories,
  docPath,
  latestVersion,
  orderedSlugs,
  resolveVersion,
  type DocCategory,
  type DocVersion,
} from '../data/docs'
import { usePublishMobileMenu } from '../contexts/MobileMenuContext'
import DocsNavTree from './DocsNavTree'

/** Horizontal inset of the header and selectors; matches DocsNavTree's rows. */
const ITEM_INSET = 1

interface Props {
  category: DocCategory
  docVersion: DocVersion
  activeSlug?: string
}

/**
 * The desktop Help Center sidebar. Below `md` it is hidden, and it offers the same navigation
 * through the mobile bottom bar's Menu sheet instead.
 */
export default function DocsSidebar({ category, docVersion, activeSlug }: Props) {
  const navigate = useNavigate()
  usePublishMobileMenu({ kind: 'docs', categoryId: category.id, version: docVersion.version, activeSlug })

  // Switching category lands on that category's newest version, at the same article when it has
  // one — so readers can move between the three manuals without losing their place entirely.
  const onCategoryChange = (e: SelectChangeEvent) => {
    const next = docCategories.find((c) => c.id === e.target.value)
    if (!next) return
    const nextVersion = latestVersion(next)
    const keepSlug = activeSlug && orderedSlugs(nextVersion).includes(activeSlug) ? activeSlug : undefined
    navigate(docPath(next, nextVersion.version, keepSlug))
  }

  // Switching version keeps the current article when that version publishes it.
  const onVersionChange = (e: SelectChangeEvent) => {
    const next = resolveVersion(category, e.target.value)
    const keepSlug = activeSlug && orderedSlugs(next).includes(activeSlug) ? activeSlug : undefined
    navigate(docPath(category, next.version, keepSlug))
  }

  const basePath = docPath(category, docVersion.version)

  return (
    <Box
      component="nav"
      aria-label="Documentation navigation"
      sx={{
        width: 240,
        flexShrink: 0,
        position: 'sticky',
        top: 'calc(74px + 16px)',
        alignSelf: 'flex-start',
        display: { xs: 'none', md: 'block' },
        maxHeight: 'calc(100vh - 74px - 32px)',
        overflowY: 'auto',
        pr: 0.5,
        // Trailing room so the last items can be scrolled up near the top of the column rather
        // than being stuck at its bottom edge.
        pb: '50vh',
        '&::-webkit-scrollbar': { width: 6 },
        '&::-webkit-scrollbar-thumb': { bgcolor: 'divider', borderRadius: 3 },
        '&::-webkit-scrollbar-track': { bgcolor: 'transparent' },
      }}
    >
      <Typography
        component={RouterLink}
        to="/help"
        variant="subtitle2"
        sx={{
          display: 'block',
          textDecoration: 'none',
          color: 'text.secondary',
          fontWeight: 700,
          textTransform: 'uppercase',
          fontSize: '0.68rem',
          letterSpacing: '0.08em',
          px: ITEM_INSET,
          mb: 1.5,
          '&:hover': { color: 'primary.main' },
        }}
      >
        Help Center
      </Typography>

      {/* Category + version selectors drive the navigation below them. */}
      <Box sx={{ px: ITEM_INSET, display: 'flex', flexDirection: 'column', gap: 1.25 }}>
        <FormControl size="small" fullWidth>
          <InputLabel id="docs-category-label">Documentation</InputLabel>
          <Select
            labelId="docs-category-label"
            label="Documentation"
            value={category.id}
            onChange={onCategoryChange}
          >
            {docCategories.map((c) => (
              <MenuItem key={c.id} value={c.id}>
                {c.title}
              </MenuItem>
            ))}
          </Select>
        </FormControl>

        <FormControl size="small" fullWidth>
          <InputLabel id="docs-version-label">Version</InputLabel>
          <Select
            labelId="docs-version-label"
            label="Version"
            value={docVersion.version}
            onChange={onVersionChange}
          >
            {category.versions.map((v) => (
              <MenuItem key={v.version} value={v.version}>
                {v.version}
                {v.released ? ` · ${v.released}` : ''}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
      </Box>

      <Divider sx={{ mt: 2 }} />

      <DocsNavTree nav={docVersion.nav} basePath={basePath} activeSlug={activeSlug} />
    </Box>
  )
}
