import { useState } from 'react'
import { Link } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Stack from '@mui/material/Stack'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import type { TournamentTS } from '../features/TournamentAPI'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  division: DivisionTS
  tournament: TournamentTS
  onUpdated: (division: DivisionTS) => void
}

export const DivisionProfileOverviewPage = ({ division, tournament, onUpdated }: Props) => {
  const [editing, setEditing] = useState(false)
  const [dname, setDname] = useState('')
  const [breadcrumb, setBreadcrumb] = useState('')
  const [isPublic, setIsPublic] = useState(true)
  const [shortinfo, setShortinfo] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const startEdit = () => {
    setDname(division.dname)
    setBreadcrumb(division.breadcrumb)
    setIsPublic(division.is_public)
    setShortinfo(division.shortinfo)
    setError(null)
    setEditing(true)
  }

  const cancelEdit = () => {
    setEditing(false)
    setError(null)
  }

  const handleSave = async () => {
    if (!dname.trim()) { setError('Division name is required.'); return }
    if (!breadcrumb.trim()) { setError('Breadcrumb is required.'); return }
    if (!shortinfo.trim()) { setError('Short info is required.'); return }
    setSaving(true)
    setError(null)
    try {
      const updated = await DivisionAPI.update(division.did, { dname, breadcrumb, is_public: isPublic, shortinfo })
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
          Division: General Info
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
            <Typography variant="body2" color="text.secondary">Division Name</Typography>
            {editing ? (
              <TextField size="small" value={dname} onChange={e => setDname(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{division.dname}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Breadcrumb</Typography>
            {editing ? (
              <TextField size="small" value={breadcrumb} onChange={e => setBreadcrumb(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{division.breadcrumb}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Visibility</Typography>
            {editing ? (
              <Select size="small" value={isPublic ? 'true' : 'false'} onChange={e => setIsPublic(e.target.value === 'true')} sx={{ mt: 0.5 }}>
                <MenuItem value="true">Public</MenuItem>
                <MenuItem value="false">Private</MenuItem>
              </Select>
            ) : (
              <Typography variant="body1">{division.is_public ? 'Public' : 'Private'}</Typography>
            )}
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(division.created_at)}</Typography>
          </Grid>

          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(division.updated_at)}</Typography>
          </Grid>

          <Grid item xs={12}>
            <Typography variant="body2" color="text.secondary">Short Info</Typography>
            {editing ? (
              <TextField size="small" value={shortinfo} onChange={e => setShortinfo(e.target.value)} fullWidth multiline minRows={2} sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{division.shortinfo}</Typography>
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
