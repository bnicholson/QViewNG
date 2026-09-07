import { useEffect, useState } from 'react'
import { Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Collapse from '@mui/material/Collapse'
import Typography from '@mui/material/Typography'
import ExpandMoreIcon from '@mui/icons-material/ExpandMore'
import ChevronRightIcon from '@mui/icons-material/ChevronRight'
import { kbNav, type KbNavNode } from '../data/knowledgeBase'

function containsSlug(node: KbNavNode, slug?: string): boolean {
  if (!slug) return false
  if (node.slug === slug) return true
  return (node.children ?? []).some((c) => containsSlug(c, slug))
}

// Folder-path ids of every folder that is an ancestor of the active article.
function ancestorFolderIds(nodes: KbNavNode[], slug: string | undefined, path: string[] = []): string[] {
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
  activeSlug,
  open,
  toggle,
}: {
  nodes: KbNavNode[]
  path: string[]
  activeSlug?: string
  open: Record<string, boolean>
  toggle: (id: string) => void
}) {
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
              to={`/help/${node.slug}`}
              sx={{
                display: 'block',
                textDecoration: 'none',
                px: '10px',
                py: '3px',
                borderLeft: '3px solid',
                borderColor: active ? 'primary.main' : 'transparent',
                color: active ? 'primary.main' : 'text.primary',
                fontWeight: active ? 600 : 400,
                fontSize: '0.875rem',
                '&:hover': { bgcolor: 'action.hover' },
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
                gap: 0.25,
                px: '6px',
                py: '3px',
                cursor: 'pointer',
                userSelect: 'none',
                fontSize: '0.875rem',
                fontWeight: 600,
                color: 'text.secondary',
                '&:hover': { bgcolor: 'action.hover' },
              }}
            >
              {isOpen ? <ExpandMoreIcon fontSize="small" /> : <ChevronRightIcon fontSize="small" />}
              {node.title}
            </Box>
            <Collapse in={isOpen} unmountOnExit>
              <Box sx={{ ml: '9px', pl: '6px', borderLeft: '1px solid', borderColor: 'divider' }}>
                <NavNodes nodes={node.children ?? []} path={[...path, node.title]} activeSlug={activeSlug} open={open} toggle={toggle} />
              </Box>
            </Collapse>
          </Box>
        )
      })}
    </>
  )
}

export default function HelpSidebar({ activeSlug }: { activeSlug?: string }) {
  const [open, setOpen] = useState<Record<string, boolean>>(() => {
    const init: Record<string, boolean> = {}
    ancestorFolderIds(kbNav, activeSlug).forEach((id) => (init[id] = true))
    return init
  })

  // When the active article changes, ensure its ancestor folders are open (user toggles preserved otherwise).
  useEffect(() => {
    setOpen((prev) => {
      const next = { ...prev }
      ancestorFolderIds(kbNav, activeSlug).forEach((id) => (next[id] = true))
      return next
    })
  }, [activeSlug])

  const toggle = (id: string) => setOpen((p) => ({ ...p, [id]: !p[id] }))

  return (
    <Box
      component="nav"
      aria-label="Help navigation"
      sx={{
        width: 220,
        flexShrink: 0,
        position: 'sticky',
        top: 'calc(74px + 16px)',
        alignSelf: 'flex-start',
        display: { xs: 'none', md: 'block' },
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
          px: '10px',
          mb: 1,
          '&:hover': { color: 'primary.main' },
        }}
      >
        Help Center
      </Typography>
      <NavNodes nodes={kbNav} path={[]} activeSlug={activeSlug} open={open} toggle={toggle} />
    </Box>
  )
}
