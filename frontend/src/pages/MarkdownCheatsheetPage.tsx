import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import Divider from '@mui/material/Divider'
import Paper from '@mui/material/Paper'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

/**
 * A dedicated, self-contained Markdown reference. Each example is rendered with the same
 * `react-markdown` + `remark-gfm` pipeline used elsewhere in the app (e.g. the Tournament
 * Overview description), so what users see here matches what they'll get.
 */

interface CheatItem {
  label: string
  /** The raw Markdown source shown in the left column and rendered in the right column. */
  source: string
}

interface CheatSection {
  title: string
  items: CheatItem[]
}

const sections: CheatSection[] = [
  {
    title: 'Headings',
    items: [
      {
        label: 'Headings (levels 1–6)',
        source: '# Heading 1\n## Heading 2\n### Heading 3\n#### Heading 4',
      },
    ],
  },
  {
    title: 'Text formatting',
    items: [
      { label: 'Bold', source: '**bold text**' },
      { label: 'Italic', source: '_italic text_' },
      { label: 'Bold + italic', source: '**_bold and italic_**' },
      { label: 'Strikethrough', source: '~~struck through~~' },
      { label: 'Inline code', source: 'Use the `pairing code` to connect.' },
    ],
  },
  {
    title: 'Links & images',
    items: [
      {
        label: 'Link to an external website',
        source: '[Visit our website](https://example.com)',
      },
      {
        label: 'Bare URL (auto-linked)',
        source: 'https://example.com',
      },
      {
        label: 'Image',
        source: '![Alt text describing the image](https://placehold.co/120x40/png)',
      },
    ],
  },
  {
    title: 'Lists',
    items: [
      {
        label: 'Bulleted list',
        source: '- First item\n- Second item\n  - Nested item\n- Third item',
      },
      {
        label: 'Numbered list',
        source: '1. First step\n2. Second step\n3. Third step',
      },
      {
        label: 'Task list',
        source: '- [x] Registration open\n- [ ] Schedule posted\n- [ ] Results published',
      },
    ],
  },
  {
    title: 'Quotes, code & rules',
    items: [
      {
        label: 'Blockquote',
        source: '> Doors open at 8:00 AM.\n> Please arrive early.',
      },
      {
        label: 'Fenced code block',
        source: '```\nCheck-in: 8:00 AM\nRound 1:  9:00 AM\n```',
      },
      {
        label: 'Horizontal rule',
        source: 'Above the line\n\n---\n\nBelow the line',
      },
    ],
  },
  {
    title: 'Tables',
    items: [
      {
        label: 'Table',
        source:
          '| Round | Start Time |\n| ----- | ---------- |\n| 1     | 9:00 AM    |\n| 2     | 10:30 AM   |',
      },
    ],
  },
  {
    title: 'Line breaks',
    items: [
      {
        label: 'New paragraph (blank line between)',
        source: 'First paragraph.\n\nSecond paragraph.',
      },
      {
        label: 'Line break (two spaces at end of line)',
        source: 'First line.  \nSecond line.',
      },
    ],
  },
]

const CheatRow = ({ item }: { item: CheatItem }) => (
  <Box
    sx={{
      display: 'grid',
      gridTemplateColumns: { xs: '1fr', md: '1fr 1fr' },
      gap: 2,
      alignItems: 'start',
      py: 2,
    }}
  >
    <Box>
      <Typography variant="caption" color="text.secondary" sx={{ display: 'block', mb: 0.5 }}>
        {item.label}
      </Typography>
      <Box
        component="pre"
        sx={{
          m: 0,
          p: 1.5,
          borderRadius: 1,
          bgcolor: (theme) => (theme.palette.mode === 'dark' ? '#1a2027' : '#f5f5f5'),
          fontFamily: 'monospace',
          fontSize: '0.85rem',
          whiteSpace: 'pre-wrap',
          overflowX: 'auto',
        }}
      >
        {item.source}
      </Box>
    </Box>
    <Box>
      <Typography variant="caption" color="text.secondary" sx={{ display: 'block', mb: 0.5 }}>
        Result
      </Typography>
      <Box sx={{ '& > *:first-of-type': { mt: 0 }, '& > *:last-child': { mb: 0 } }}>
        <Markdown remarkPlugins={[remarkGfm]}>{item.source}</Markdown>
      </Box>
    </Box>
  </Box>
)

export const MarkdownCheatsheetPage = () => {
  return (
    <Box sx={{ maxWidth: 900, mx: 'auto', px: { xs: 2, sm: 3 }, py: { xs: 3, sm: 5 }, textAlign: 'left' }}>
      <Typography variant="h4" component="h1" sx={{ fontWeight: 600, mb: 1 }}>
        Markdown Cheatsheet
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ mb: 3 }}>
        Markdown is a simple way to format text. Type the syntax on the left to get the result on the
        right. These examples render exactly the way descriptions do throughout QView.
      </Typography>

      {sections.map((section) => (
        <Paper key={section.title} variant="outlined" sx={{ p: { xs: 2, sm: 3 }, mb: 3 }}>
          <Typography variant="h6" sx={{ fontWeight: 600 }}>
            {section.title}
          </Typography>
          <Divider sx={{ mt: 1 }} />
          {section.items.map((item, i) => (
            <Box key={item.label}>
              {i > 0 && <Divider />}
              <CheatRow item={item} />
            </Box>
          ))}
        </Paper>
      ))}
    </Box>
  )
}
