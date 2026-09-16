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
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import type { DivisionTS } from '../features/DivisionAPI'
import { useAuth } from '../hooks/useAuth'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

interface Props {
  /** The pool bracket being viewed. */
  bracket: PoolBracketTS
  /** Its parent division. */
  division: DivisionTS
  /** Singular label for this bracket's type ("Pool" or "Bracket"). */
  entityLabel: string
  onUpdated: (bracket: PoolBracketTS) => void
  canEdit?: boolean
}

export const PoolBracketProfileOverviewPage = ({ bracket, division, entityLabel, onUpdated, canEdit = false }: Props) => {
  const { accessToken } = useAuth()
  const [editing, setEditing] = useState(false)
  const [name, setName] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const startEdit = () => {
    setName(bracket.name)
    setError(null)
    setEditing(true)
  }

  const cancelEdit = () => {
    setEditing(false)
    setError(null)
  }

  const handleSave = async () => {
    if (!name.trim()) { setError(`${entityLabel} name is required.`); return }
    setSaving(true)
    setError(null)
    try {
      const updated = await PoolBracketAPI.update(bracket.pool_bracket_id, { name: name.trim() }, accessToken)
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
          {entityLabel}: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
        )}

        <Grid container spacing={{ xs: 1, sm: 2 }}>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">{entityLabel} Name</Typography>
            {editing ? (
              <TextField size="small" value={name} onChange={e => setName(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{bracket.name}</Typography>
            )}
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Type</Typography>
            <Typography variant="body1">{bracket.type}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Division</Typography>
            <Typography variant="body1">
              <Link
                to={`/division/${division.did}/overview`}
                style={{ color: '#2563eb', textDecoration: 'none' }}
                onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
                onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
              >
                {division.dname}
              </Link>
            </Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(bracket.created_date)}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(bracket.last_modified_date)}</Typography>
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
          ) : canEdit && (
            <Button variant="outlined" size="small" onClick={startEdit}>
              Edit
            </Button>
          )}
        </Box>
      </Box>
    </Stack>
  )
}
