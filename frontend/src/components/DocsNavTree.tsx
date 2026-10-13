import { useEffect, useState } from 'react'
import { Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Collapse from '@mui/material/Collapse'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import type { DocNavNode } from '../data/docs'

/** Horizontal inset of every nav row; the sidebar's header and selectors use the same. */
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
  touch,
  onSelect,
}: {
  nodes: DocNavNode[]
  path: string[]
  /** `/{category}/{version}` — article links are this plus the slug. */
  basePath: string
  activeSlug?: string
  open: Record<string, boolean>
  toggle: (id: string) => void
  touch: boolean
  onSelect?: () => void
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
              onClick={onSelect}
              aria-current={active ? 'page' : undefined}
              sx={{
                display: 'block',
                textDecoration: 'none',
                px: ITEM_INSET,
                py: touch ? 1.125 : 0.625,
                my: '1px',
                borderRadius: 1,
                fontSize: touch ? '0.9375rem' : '0.8125rem',
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
        // Folders start expanded; only a folder the reader has collapsed is closed.
        const isOpen = open[id] ?? true
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
                py: touch ? 1.125 : 0.75,
                mt: depth === 0 ? (touch ? 1 : 1.75) : 0.5,
                borderRadius: 1,
                cursor: 'pointer',
                userSelect: 'none',
                fontSize: touch ? (depth === 0 ? '1rem' : '0.9375rem') : depth === 0 ? '0.9375rem' : '0.875rem',
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
                  touch={touch}
                  onSelect={onSelect}
                />
              </Box>
            </Collapse>
          </Box>
        )
      })}
    </>
  )
}

interface DocsNavTreeProps {
  nav: DocNavNode[]
  /** `/{category}/{version}` — article links are this plus the slug. */
  basePath: string
  activeSlug?: string
  /** Larger touch targets, for the mobile menu sheet. */
  touch?: boolean
  /** Called when an article link is chosen. */
  onSelect?: () => void
}

/**
 * One manual version's navigation tree: collapsible folders of article links. Every folder starts
 * expanded. Shared by the desktop sidebar and the mobile menu sheet.
 */
export default function DocsNavTree({ nav, basePath, activeSlug, touch = false, onSelect }: DocsNavTreeProps) {
  // Only folders the reader has toggled are recorded; any other folder is open.
  const [open, setOpen] = useState<Record<string, boolean>>({})

  // When the active article (or the version's tree) changes, reopen any of its ancestor folders the
  // reader collapsed. Their other toggles are preserved.
  useEffect(() => {
    setOpen((prev) => {
      const next = { ...prev }
      ancestorFolderIds(nav, activeSlug).forEach((id) => (next[id] = true))
      return next
    })
  }, [activeSlug, nav])

  const toggle = (id: string) => setOpen((p) => ({ ...p, [id]: !(p[id] ?? true) }))

  return (
    <NavNodes
      nodes={nav}
      path={[]}
      basePath={basePath}
      activeSlug={activeSlug}
      open={open}
      toggle={toggle}
      touch={touch}
      onSelect={onSelect}
    />
  )
}
