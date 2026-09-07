import { Link as RouterLink } from 'react-router-dom'
import Box from '@mui/material/Box'
import Paper from '@mui/material/Paper'
import Typography from '@mui/material/Typography'
import { kbNav, type KbNavNode } from '../data/knowledgeBase'

// Flatten a category subtree into its article leaves (for the home-page card lists).
function articlesUnder(node: KbNavNode): { slug: string; title: string }[] {
  if (node.slug) return [{ slug: node.slug, title: node.title }]
  return (node.children ?? []).flatMap(articlesUnder)
}

export const HelpPage = () => {
  return (
    <Box sx={{ maxWidth: 960, mx: 'auto' }}>
      {/* Hero */}
      <Box sx={{ textAlign: 'center', py: { xs: 4, sm: 6 } }}>
        <Typography variant="h3" sx={{ fontWeight: 600, mb: 1 }}>
          QView Help Center
        </Typography>
        <Typography variant="body1" color="text.secondary" sx={{ maxWidth: 620, mx: 'auto' }}>
          Guides and answers for running tournaments, managing teams and rosters, and setting up
          your equipment. Browse a category below to get started.
        </Typography>
      </Box>

      {/* Category cards */}
      <Box
        sx={{
          display: 'grid',
          gridTemplateColumns: { xs: '1fr', sm: '1fr 1fr' },
          gap: 2,
          pb: 6,
        }}
      >
        {kbNav.map((category) => {
          const articles = articlesUnder(category)
          return (
            <Paper key={category.title} variant="outlined" sx={{ p: 2.5, borderRadius: 2 }}>
              <Typography variant="h6" sx={{ fontWeight: 600, mb: 1 }}>
                {category.title}
              </Typography>
              <Box component="ul" sx={{ listStyle: 'none', m: 0, p: 0 }}>
                {articles.map((a) => (
                  <Box component="li" key={a.slug} sx={{ mb: 0.5 }}>
                    <Typography
                      component={RouterLink}
                      to={`/help/${a.slug}`}
                      sx={{
                        color: 'primary.main',
                        textDecoration: 'none',
                        fontSize: '0.925rem',
                        '&:hover': { textDecoration: 'underline' },
                      }}
                    >
                      {a.title}
                    </Typography>
                  </Box>
                ))}
              </Box>
            </Paper>
          )
        })}
      </Box>
    </Box>
  )
}
