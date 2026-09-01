import Button from '@mui/material/Button'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faFloppyDisk } from '@fortawesome/free-solid-svg-icons'

interface SaveButtonProps {
  onClick: () => void
  label?: string
  savingLabel?: string
  saving?: boolean
  disabled?: boolean
  autoFocus?: boolean
}

export function SaveButton({
  onClick,
  label = 'Save',
  savingLabel = 'Saving...',
  saving = false,
  disabled = false,
  autoFocus = true,
}: SaveButtonProps) {
  return (
    <Button
      autoFocus={autoFocus}
      color="inherit"
      startIcon={<FontAwesomeIcon icon={faFloppyDisk} style={{ fontSize: '0.9em' }} />}
      onClick={onClick}
      disabled={disabled || saving}
    >
      {saving ? savingLabel : label}
    </Button>
  )
}
