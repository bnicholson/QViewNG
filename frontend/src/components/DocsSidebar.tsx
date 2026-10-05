import { useEffect, useState } from 'react'
import { Link as RouterLink, useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Collapse from '@mui/material/Collapse'
import Divider from '@mui/material/Divider'
import FormControl from '@mui/material/FormControl'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select, { type SelectChangeEvent } from '@mui/material/Select'
import Typography from '@mui/material/Typography'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import {
  docCategories,
  docPath,
  latestVersion,
  orderedSlugs,
  resolveVersion,
  type DocCategory,
  type DocNavNode,
  type DocVersion,
} from '../data/docs'

/** Horizontal inset shared by the header, the selectors and every nav row. */
const ITEM_INSET = 1

function containsSlug(node: DocNavNode, slug?: string): boolean {
  if (!slug) return false
  if (node.slug === slug) return true
  return (node.children ?? []).some((c) => containsSlug(c, slug))
}

// Folder-path ids of every folder that is an ancestor of the active article.
function ancestorFolderIds(nodes: DocNavNode[], slug: string | undefined, path: string[] = []): string[] {
  const ids: string[] = []
  for (const n of nodes) {
    if (n.children) {
      const id = [...path, n.title].join('/')
      if (containsSlug(n, slug)) ids.push(id)
      ids.push(...ancestorFolderIds(n.children, slug, [...path, n.title]))
    }
  }
  return ids
}

function NavNodes({
  nodes,
  path,
  basePath,
  activeSlug,
  open,
  toggle,
}: {
  nodes: DocNavNode[]
  path: string[]
  /** `/{category}/{version}` — article links are this plus the slug. */
  basePath: string
  activeSlug?: string
  open: Record<string, boolean>
  toggle: (id: string) => void
}) {
  const depth = path.length

  return (
    <>
      {nodes.map((node) => {
        // Leaf: a link to an article.
        if (node.slug) {
          const active = node.slug === activeSlug
          return (
            <Box
              key={node.slug}
              component={RouterLink}
              to={`${basePath}/${node.slug}`}
              sx={{
                display: 'block',
                textDecoration: 'none',
                px: ITEM_INSET,
                py: 0.625,
                my: '1px',
                borderRadius: 1,
                fontSize: '0.8125rem',
                lineHeight: 1.45,
                fontWeight: active ? 600 : 400,
                color: active ? 'primary.main' : 'text.secondary',
                bgcolor: active ? 'action.selected' : 'transparent',
                '&:hover': {
                  bgcolor: active ? 'action.selected' : 'action.hover',
                  color: active ? 'primary.main' : 'text.primary',
                },
              }}
            >
              {node.title}
            </Box>
          )
        }

        // Folder: a collapsible group.
        const id = [...path, node.title].join('/')
        const isOpen = !!open[id]
        return (
          <Box key={id}>
            <Box
              onClick={() => toggle(id)}
              role="button"
              aria-expanded={isOpen}
              sx={{
                display: 'flex',
                alignItems: 'center',
                gap: 0.5,
                px: ITEM_INSET,
                py: 0.75,
                mt: depth === 0 ? 1.75 : 0.5,
                borderRadius: 1,
                cursor: 'pointer',
                userSelect: 'none',
                fontSize: depth === 0 ? '0.9375rem' : '0.875rem',
                fontWeight: 700,
                lineHeight: 1.35,
                color: 'text.primary',
                '&:hover': { bgcolor: 'action.hover' },
              }}
            >
              <ChevronRightIcon
                sx={{
                  fontSize: 18,
                  flexShrink: 0,
                  color: 'text.disabled',
                  transition: 'transform 150ms ease',
                  transform: isOpen ? 'rotate(90deg)' : 'none',
                }}
              />
              {node.title}
            </Box>
            <Collapse in={isOpen} unmountOnExit>
              <Box sx={{ ml: 1.25, pl: 1, borderLeft: '1px solid', borderColor: 'divider' }}>
                <NavNodes
                  nodes={node.children ?? []}
                  path={[...path, node.title]}
                  basePath={basePath}
                  activeSlug={activeSlug}
                  open={open}
                  toggle={toggle}
                />
              </Box>
            </Collapse>
          </Box>
        )
      })}
    </>
  )
}

interface Props {
  category: DocCategory
  docVersion: DocVersion
  activeSlug?: string
}

export default function DocsSidebar({ category, docVersion, activeSlug }: Props) {
  const navigate = useNavigate()
  const [open, setOpen] = useState<Record<string, boolean>>(() => {
    const init: Record<string, boolean> = {}
    ancestorFolderIds(docVersion.nav, activeSlug).forEach((id) => (init[id] = true))
    return init
  })

  // When the active article (or the version's tree) changes, ensure its ancestor folders are open.
  // User toggles are otherwise preserved.
  useEffect(() => {
    setOpen((prev) => {
      const next = { ...prev }
      ancestorFolderIds(docVersion.nav, activeSlug).forEach((id) => (next[id] = true))
      return next
    })
  }, [activeSlug, docVersion])

  const toggle = (id: string) => setOpen((p) => ({ ...p, [id]: !p[id] }))

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

      <NavNodes
        nodes={docVersion.nav}
        path={[]}
        basePath={basePath}
        activeSlug={activeSlug}
        open={open}
        toggle={toggle}
      />
    </Box>
  )
}
