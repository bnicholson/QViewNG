import { useState } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'

interface Props {
  group: TournamentGroupTS
  canEdit: boolean
  onUpdated: (group: TournamentGroupTS) => void
}

export const TournamentGroupOverviewPage = ({ group, canEdit, onUpdated }: Props) => {
  const [editing, setEditing] = useState(false)
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const openEdit = () => {
    setName(group.name)
    setDescription(group.description ?? '')
    setError(null)
    setEditing(true)
  }

  const handleSave = async () => {
    if (!name.trim()) { setError('Name is required.'); return }
    setSaving(true)
    setError(null)
    try {
      const updated = await TournamentGroupAPI.update(group.tgid, {
        name: name.trim(),
        description: description.trim() || null,
      })
      onUpdated(updated)
      setEditing(false)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2, gap: 2 }}>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 600, flex: 1 }}>
          {group.name}
        </Typography>
        {canEdit && !editing && (
          <Button variant="contained" size="small" onClick={openEdit}>Edit</Button>
        )}
      </Box>

      {editing ? (
        <Box sx={{ maxWidth: 600 }}>
          <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
            Tournament Group: Edit Info
          </Typography>
          <Divider sx={{ mb: 2 }} />

          {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}

          <Grid container spacing={2}>
            <Grid item xs={12}>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 0.5 }}>Name *</Typography>
              <TextField
                fullWidth
                size="small"
                value={name}
                onChange={e => setName(e.target.value)}
              />
            </Grid>
            <Grid item xs={12}>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 0.5 }}>Description</Typography>
              <TextField
                fullWidth
                size="small"
                multiline
                minRows={3}
                value={description}
                onChange={e => setDescription(e.target.value)}
                placeholder="Optional description"
              />
            </Grid>
          </Grid>

          <Box sx={{ display: 'flex', gap: 1, mt: 2 }}>
            <Button
              variant="contained"
              onClick={handleSave}
              disabled={saving}
              startIcon={saving ? <CircularProgress size={14} /> : undefined}
            >
              Save
            </Button>
            <Button variant="outlined" onClick={() => setEditing(false)} disabled={saving}>
              Cancel
            </Button>
          </Box>
        </Box>
      ) : (
        <Box>
          <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
            Tournament Group: General Info
          </Typography>
          <Divider sx={{ mb: 2 }} />

          <Grid container spacing={{ xs: 1, sm: 2 }}>
            <Grid item xs={12} sm={6}>
              <Typography variant="body2" color="text.secondary">Name</Typography>
              <Typography variant="body1">{group.name}</Typography>
            </Grid>
            <Grid item xs={12}>
              <Typography variant="body2" color="text.secondary">Description</Typography>
              <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>
                {group.description || <span style={{ color: '#9ca3af' }}>—</span>}
              </Typography>
            </Grid>
            <Grid item xs={12} sm={6}>
              <Typography variant="body2" color="text.secondary">Created</Typography>
              <Typography variant="body1">
                {new Date(group.created_at).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}
              </Typography>
            </Grid>
            <Grid item xs={12} sm={6}>
              <Typography variant="body2" color="text.secondary">Last Modified</Typography>
              <Typography variant="body1">
                {new Date(group.updated_at).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}
              </Typography>
            </Grid>
          </Grid>
        </Box>
      )}
    </Box>
  )
}
