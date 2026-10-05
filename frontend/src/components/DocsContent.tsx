import { useMemo, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Dialog from '@mui/material/Dialog'
import IconButton from '@mui/material/IconButton'
import Typography from '@mui/material/Typography'
import CloseIcon from '@mui/icons-material/Close'
import { resolveDocHref, type DocCategory } from '../data/docs'

interface Props {
  html: string
  category: DocCategory
  version: string
}

/**
 * Renders authored documentation HTML.
 *
 * Internal cross-references are authored with the `doc:` scheme (see `resolveDocHref`), which is
 * how the source manuals' bookmark links are preserved. They're rewritten to real app paths here
 * and clicks are intercepted so they navigate within the SPA instead of reloading the page.
 *
 * Screenshots from the source manuals are authored as `<figure><img><figcaption>`; clicking one
 * opens it full size in a lightbox, since many are dense dialog captures.
 */
export default function DocsContent({ html, category, version }: Props) {
  const navigate = useNavigate()
  const [lightbox, setLightbox] = useState<{ src: string; alt: string; caption: string } | null>(null)

  const resolved = useMemo(
    () =>
      html.replace(/href="doc:([^"]*)"/g, (match, body: string) => {
        const path = resolveDocHref(`doc:${body}`, category, version)
        return path ? `href="${path}"` : match
      }),
    [html, category, version],
  )

  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.defaultPrevented || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey || e.button !== 0) return
    const target = e.target as HTMLElement

    // Clicking a screenshot opens it at full size.
    const img = target.closest('figure img') as HTMLImageElement | null
    if (img) {
      e.preventDefault()
      const caption = img.closest('figure')?.querySelector('figcaption')?.textContent ?? ''
      setLightbox({ src: img.getAttribute('src') ?? '', alt: img.getAttribute('alt') ?? '', caption })
      return
    }

    const anchor = target.closest('a')
    if (!anchor) return
    const href = anchor.getAttribute('href')
    // Only intercept same-app paths; external links and new-tab modifiers behave normally.
    if (!href || !href.startsWith('/')) return
    e.preventDefault()
    navigate(href)
  }

  return (
    <>
      <Box
        onClick={handleClick}
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
          '& table': { borderCollapse: 'collapse', mb: 2, fontSize: '0.9rem' },
          '& th, & td': { border: '1px solid', borderColor: 'divider', px: 1, py: 0.5, textAlign: 'left' },
          '& th': { bgcolor: 'action.hover', fontWeight: 600 },
          // Screenshots carried over from the source manuals.
          '& figure': { m: 0, mb: 2.5, mt: 1.5 },
          '& figure img': {
            display: 'block',
            maxWidth: '100%',
            height: 'auto',
            border: '1px solid',
            borderColor: 'divider',
            borderRadius: 1,
            cursor: 'zoom-in',
            bgcolor: 'background.paper',
          },
          '& figcaption': {
            fontSize: '0.8125rem',
            color: 'text.secondary',
            mt: 0.75,
            fontStyle: 'italic',
          },
          // A neutral aside from the manual ("Note: …").
          '& .doc-note': {
            bgcolor: 'action.hover',
            borderLeft: '3px solid',
            borderColor: 'info.main',
            px: 1.5,
            py: 1,
            borderRadius: 1,
            mb: 1.5,
          },
          // Something the manual warns against, or that risks losing data.
          '& .doc-warning': {
            bgcolor: 'action.hover',
            borderLeft: '3px solid',
            borderColor: 'error.main',
            px: 1.5,
            py: 1,
            borderRadius: 1,
            mb: 1.5,
          },
          // Marks a section whose prose hasn't been migrated from the source manual yet.
          '& .doc-draft-note': {
            bgcolor: 'action.hover',
            borderLeft: '3px solid',
            borderColor: 'warning.main',
            px: 1.5,
            py: 1,
            borderRadius: 1,
            fontSize: '0.875rem',
            color: 'text.secondary',
          },
        }}
        dangerouslySetInnerHTML={{ __html: resolved }}
      />

      <Dialog open={!!lightbox} onClose={() => setLightbox(null)} maxWidth="xl" fullWidth>
        <Box sx={{ position: 'relative', bgcolor: 'background.paper' }}>
          <IconButton
            aria-label="Close image"
            onClick={() => setLightbox(null)}
            sx={{ position: 'absolute', top: 8, right: 8, bgcolor: 'background.paper', boxShadow: 1 }}
          >
            <CloseIcon />
          </IconButton>
          {lightbox && (
            <Box sx={{ p: 2 }}>
              <Box
                component="img"
                src={lightbox.src}
                alt={lightbox.alt}
                sx={{ display: 'block', maxWidth: '100%', maxHeight: '80vh', mx: 'auto' }}
              />
              {lightbox.caption && (
                <Typography variant="body2" color="text.secondary" sx={{ mt: 1.5, textAlign: 'center' }}>
                  {lightbox.caption}
                </Typography>
              )}
            </Box>
          )}
        </Box>
      </Dialog>
    </>
  )
}
