import { useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import { type RoomGroupTS } from '../features/RoomGroupAPI'
import { BuildingEditorDialog } from '../components/BuildingEditorDialog'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  /** The building being viewed. */
  building: RoomGroupTS
  /** The parent tournament's id — needed by the editor dialog. */
  tid: string
  onUpdated: (building: RoomGroupTS) => void
  /** Whether audit fields (created/last-modified) are shown. */
  showAuditColumns?: boolean
  canEdit?: boolean
}

export const BuildingProfileOverviewPage = ({ building, tid, onUpdated, showAuditColumns = false, canEdit = false }: Props) => {
  const [editorOpen, setEditorOpen] = useState(false)

  return (
    <Stack spacing={3}>
      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Building: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Building Name</Typography>
            <Typography variant="body1">{building.name}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Notes</Typography>
            <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>{building.notes || '—'}</Typography>
          </Grid>

          {showAuditColumns && (
            <>
              <Grid size={{ xs: 12, sm: 6, md: 4 }}>
                <Typography variant="body2" color="text.secondary">Created</Typography>
                <Typography variant="body1" color="text.secondary">{formatDate(building.created_date)}</Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6, md: 4 }}>
                <Typography variant="body2" color="text.secondary">Last Modified</Typography>
                <Typography variant="body1" color="text.secondary">{formatDate(building.last_modified_date)}</Typography>
              </Grid>
            </>
          )}
        </Grid>

        {canEdit && (
          <Box sx={{ mt: 2 }}>
            <Button variant="outlined" size="small" onClick={() => setEditorOpen(true)}>Edit</Button>
          </Box>
        )}
      </Box>

      <BuildingEditorDialog
        tid={tid}
        building={building}
        isOpen={editorOpen}
        onCancel={() => setEditorOpen(false)}
        onSave={(updated) => { setEditorOpen(false); onUpdated(updated) }}
      />
    </Stack>
  )
}
