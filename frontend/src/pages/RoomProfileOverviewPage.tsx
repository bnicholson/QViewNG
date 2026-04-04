import { useState } from 'react'
import { Link } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import type { TournamentTS } from '../features/TournamentAPI'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  room: RoomTS
  tournament: TournamentTS
  onUpdated: (room: RoomTS) => void
}

export const RoomProfileOverviewPage = ({ room, tournament, onUpdated }: Props) => {
  const [editing, setEditing] = useState(false)
  const [name, setName] = useState('')
  const [building, setBuilding] = useState('')
  const [comments, setComments] = useState('')
  const [clientkey, setClientkey] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const startEdit = () => {
    setName(room.name)
    setBuilding(room.building)
    setComments(room.comments)
    setClientkey(room.clientkey)
    setError(null)
    setEditing(true)
  }

  const cancelEdit = () => {
    setEditing(false)
    setError(null)
  }

  const handleSave = async () => {
    if (!name.trim()) { setError('Name is required.'); return }
    setSaving(true)
    setError(null)
    try {
      const updated = await RoomAPI.update(room.roomid, { name, building, comments, clientkey })
      onUpdated(updated)
      setEditing(false)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  return (
    <Stack spacing={3}>
      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Room: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
        )}

        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Tournament</Typography>
            <Typography variant="body1">
              <Link
                to={`/tournament/${tournament.tid}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {tournament.tname}
              </Link>
            </Typography>
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Name</Typography>
            {editing ? (
              <TextField size="small" value={name} onChange={e => setName(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{room.name}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Building</Typography>
            {editing ? (
              <TextField size="small" value={building} onChange={e => setBuilding(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{room.building || '—'}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Client Key</Typography>
            {editing ? (
              <TextField size="small" value={clientkey} onChange={e => setClientkey(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{room.clientkey || '—'}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(room.created_at)}</Typography>
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(room.updated_at)}</Typography>
          </Grid>

          <Grid item xs={12}>
            <Typography variant="body2" color="text.secondary">Comments</Typography>
            {editing ? (
              <TextField size="small" value={comments} onChange={e => setComments(e.target.value)} fullWidth multiline minRows={2} sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{room.comments || '—'}</Typography>
            )}
          </Grid>
        </Grid>

        <Box sx={{ mt: 2, display: 'flex', gap: 1 }}>
          {editing ? (
            <>
              <Button variant="contained" size="small" onClick={handleSave} disabled={saving}>
                {saving ? 'Saving…' : 'Save'}
              </Button>
              <Button variant="outlined" size="small" onClick={cancelEdit} disabled={saving}>
                Cancel
              </Button>
            </>
          ) : (
            <Button variant="outlined" size="small" onClick={startEdit}>
              Edit
            </Button>
          )}
        </Box>
      </Box>
    </Stack>
  )
}
