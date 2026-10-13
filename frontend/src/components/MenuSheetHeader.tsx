import Box from '@mui/material/Box'
import IconButton from '@mui/material/IconButton'
import Typography from '@mui/material/Typography'
import CloseIcon from '@mui/icons-material/Close'

interface MenuSheetHeaderProps {
  /** Small label above the title, e.g. "Team Profile:". */
  overline: string
  title?: React.ReactNode
  onClose: () => void
}

/** The top of the mobile Menu sheet: what the menu belongs to, and a close button. */
export default function MenuSheetHeader({ overline, title, onClose }: MenuSheetHeaderProps) {
  return (
    <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1, px: 2, pt: 1, pb: 1.5 }}>
      <Box sx={{ flex: 1, minWidth: 0 }}>
        <Typography variant="overline" sx={{ color: 'text.secondary', lineHeight: 1.6, display: 'block' }}>
          {overline}
        </Typography>
        {title !== undefined && (
          <Typography variant="h6" component="h2" sx={{ fontWeight: 700, lineHeight: 1.3, overflowWrap: 'anywhere' }}>
            {title}
          </Typography>
        )}
      </Box>
      <IconButton aria-label="Close menu" onClick={onClose} edge="end">
        <CloseIcon />
      </IconButton>
    </Box>
  )
}
