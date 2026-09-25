import Dialog from '@mui/material/Dialog'
import DialogTitle from '@mui/material/DialogTitle'
import DialogContent from '@mui/material/DialogContent'
import DialogActions from '@mui/material/DialogActions'
import Button from '@mui/material/Button'
import { PoolsManager } from './PoolsManager'

interface Props {
  isOpen: boolean
  tid: string
  did: string
  /** Shown in the title so the user knows which division's pool groups they're editing. */
  divisionName?: string
  canEdit: boolean
  onClose: VoidFunction
  /** Called after any change inside, so the host can refresh its cached pools/groups/games. */
  onChanged?: () => void
}

/** Edit a division's pool groups — hosts the full Pools workflow (the same one as Edit V1's Pools tab). */
export const PoolGroupsEditorDialog = ({ isOpen, tid, did, divisionName, canEdit, onClose, onChanged }: Props) => (
  <Dialog open={isOpen} onClose={onClose} fullWidth maxWidth="lg" scroll="paper">
    <DialogTitle>Edit Pool Groups{divisionName ? ` — ${divisionName}` : ''}</DialogTitle>
    <DialogContent dividers>
      {isOpen && <PoolsManager tid={tid} did={did} canEdit={canEdit} onChanged={onChanged} />}
    </DialogContent>
    <DialogActions>
      <Button onClick={onClose}>Done</Button>
    </DialogActions>
  </Dialog>
)
