import { useState, useEffect, useCallback } from 'react'
import { Link } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import InputAdornment from '@mui/material/InputAdornment'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import ListItemText from '@mui/material/ListItemText'
import Paper from '@mui/material/Paper'
import Stack from '@mui/material/Stack'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import SearchIcon from '@mui/icons-material/Search'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import type { TournamentTS } from '../features/TournamentAPI'
import { useAuth } from '../hooks/useAuth'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

function UserProfileLink({ id, name }: { id: string; name: string }) {
  return (
    <Link
      to={`/user/${id}/overview`}
      style={{ color: '#2563eb', textDecoration: 'none' }}
      onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
      onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
    >
      {name}
    </Link>
  )
}

interface Props {
  room: RoomTS
  tournament: TournamentTS
  onUpdated: (room: RoomTS) => void
  showSensitiveColumns?: boolean
}

type Role = 'quizmaster' | 'contentjudge'

interface AssignmentSectionProps {
  label: string
  currentId: string | null
  currentName: string | null
  occupiedIds: Set<string>
  onAssign: (user: UserTS) => Promise<void>
  onRemove: () => Promise<void>
  saving: boolean
  canEdit: boolean
}

const AssignmentSection = ({
  label, currentId, currentName, occupiedIds, onAssign, onRemove, saving, canEdit,
}: AssignmentSectionProps) => {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<UserTS[]>([])
  const [searching, setSearching] = useState(false)
  const [assigningId, setAssigningId] = useState<string | null>(null)
  const [removing, setRemoving] = useState(false)

  const search = useCallback(async (q: string) => {
    if (!q.trim()) { setResults([]); return }
    setSearching(true)
    try {
      const paged = await UserAPI.get(0, 100)
      const lower = q.toLowerCase()
      setResults(
        paged.items.filter(u =>
          !occupiedIds.has(u.id) &&
          (`${u.fname} ${u.lname}`.toLowerCase().includes(lower) ||
            u.username.toLowerCase().includes(lower))
        )
      )
    } finally {
      setSearching(false)
    }
  }, [occupiedIds])

  useEffect(() => {
    const t = setTimeout(() => search(query), 300)
    return () => clearTimeout(t)
  }, [query, search])

  const handleAssign = async (user: UserTS) => {
    setAssigningId(user.id)
    try {
      await onAssign(user)
      setQuery('')
      setResults([])
    } finally {
      setAssigningId(null)
    }
  }

  const handleRemove = async () => {
    setRemoving(true)
    try { await onRemove() } finally { setRemoving(false) }
  }

  return (
    <Box>
      <Typography variant="subtitle2" sx={{ fontWeight: 600, mb: 1, textAlign: "left" }}>{label}</Typography>

      {currentId && currentName ? (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
          <UserProfileLink id={currentId} name={currentName} />
          {canEdit && (
            <Button
              size="small"
              variant="outlined"
              color="error"
              disabled={removing || saving}
              onClick={handleRemove}
              sx={{ ml: 1 }}
            >
              {removing ? 'Removing…' : 'Remove'}
            </Button>
          )}
        </Box>
      ) : (
        <Typography variant="body2" color="text.secondary" sx={{ mb: 1, textAlign: "left" }}>
          No {label.toLowerCase()} assigned.
        </Typography>
      )}

      {canEdit && (
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start' }}>
          <TextField
            size="small"
            placeholder={`Search users to assign as ${label.toLowerCase()}…`}
            value={query}
            onChange={e => setQuery(e.target.value)}
            sx={{ maxWidth: 400 }}
            InputProps={{
              endAdornment: (
                <InputAdornment position="end">
                  {searching ? <CircularProgress size={14} /> : <SearchIcon fontSize="small" />}
                </InputAdornment>
              ),
            }}
          />

          {results.length > 0 && (
            <Paper variant="outlined" sx={{ width: '100%', maxWidth: 440, mt: 0.5 }}>
              <List dense disablePadding>
                {results.map((u, i) => (
                  <ListItem
                    key={u.id}
                    divider={i < results.length - 1}
                    sx={{ display: 'flex', justifyContent: 'space-between', gap: 1 }}
                  >
                    <ListItemText
                      primary={`${u.fname} ${u.lname}`}
                      secondary={u.username}
                      sx={{ flex: 1, minWidth: 0 }}
                    />
                    <Button
                      size="small"
                      variant="contained"
                      disabled={!!assigningId || saving}
                      onClick={() => handleAssign(u)}
                      startIcon={assigningId === u.id ? <CircularProgress size={12} /> : undefined}
                      sx={{ flexShrink: 0 }}
                    >
                      Assign
                    </Button>
                  </ListItem>
                ))}
              </List>
            </Paper>
          )}

          {!searching && query.trim() && results.length === 0 && (
            <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
              No available users match "{query}".
            </Typography>
          )}
        </Box>
      )}
    </Box>
  )
}

export const RoomProfileOverviewPage = ({ room, tournament, onUpdated, showSensitiveColumns = false }: Props) => {
  const { accessToken } = useAuth()
  const [editing, setEditing] = useState(false)
  const [name, setName] = useState('')
  const [building, setBuilding] = useState('')
  const [comments, setComments] = useState('')
  const [clientkey, setClientkey] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Resolved display names for current QM/CJ
  const [qmName, setQmName] = useState<string | null>(null)
  const [cjName, setCjName] = useState<string | null>(null)

  // IDs currently occupied across all rooms in this tournament (QM + CJ), excluding this room
  const [occupiedIds, setOccupiedIds] = useState<Set<string>>(new Set())

  useEffect(() => {
    if (room.quizmaster_id) {
      UserAPI.getById(room.quizmaster_id)
        .then(u => setQmName(`${u.fname} ${u.lname}`))
        .catch(() => setQmName(null))
    } else {
      setQmName(null)
    }
    if (room.contentjudge_id) {
      UserAPI.getById(room.contentjudge_id)
        .then(u => setCjName(`${u.fname} ${u.lname}`))
        .catch(() => setCjName(null))
    } else {
      setCjName(null)
    }
  }, [room.quizmaster_id, room.contentjudge_id])

  useEffect(() => {
    RoomAPI.getByTournament(tournament.tid.toString(), 0, 500).then(rooms => {
      const ids = new Set<string>()
      for (const r of rooms) {
        if (r.roomid === room.roomid) continue
        if (r.quizmaster_id) ids.add(r.quizmaster_id)
        if (r.contentjudge_id) ids.add(r.contentjudge_id)
      }
      setOccupiedIds(ids)
    }).catch(() => {})
  }, [tournament.tid, room.roomid])

  // IDs occupied by *this* room's current assignments
  const thisRoomIds = new Set<string>([
    ...(room.quizmaster_id ? [room.quizmaster_id] : []),
    ...(room.contentjudge_id ? [room.contentjudge_id] : []),
  ])
  const allOccupiedForQm = new Set([...occupiedIds, ...(room.contentjudge_id ? [room.contentjudge_id] : [])])
  const allOccupiedForCj = new Set([...occupiedIds, ...(room.quizmaster_id ? [room.quizmaster_id] : [])])

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
      const updated = await RoomAPI.update(room.roomid, { name, building, comments, clientkey }, accessToken ?? undefined)
      onUpdated(updated)
      setEditing(false)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  const handleAssign = async (role: Role, user: UserTS) => {
    const patch = role === 'quizmaster'
      ? { quizmaster_id: user.id }
      : { contentjudge_id: user.id }
    const updated = await RoomAPI.update(room.roomid, patch, accessToken ?? undefined)
    onUpdated(updated)
    if (role === 'quizmaster') setQmName(`${user.fname} ${user.lname}`)
    else setCjName(`${user.fname} ${user.lname}`)
  }

  const handleRemove = async (role: Role) => {
    const patch = role === 'quizmaster'
      ? { quizmaster_id: null }
      : { contentjudge_id: null }
    const updated = await RoomAPI.update(room.roomid, patch, accessToken ?? undefined)
    onUpdated(updated)
    if (role === 'quizmaster') setQmName(null)
    else setCjName(null)
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

          {showSensitiveColumns && (
            <Grid item xs={12} sm={6} md={4}>
              <Typography variant="body2" color="text.secondary">Client Key</Typography>
              {editing ? (
                <TextField size="small" value={clientkey} onChange={e => setClientkey(e.target.value)} fullWidth sx={{ mt: 0.5 }} />
              ) : (
                <Typography variant="body1">{room.clientkey || '—'}</Typography>
              )}
            </Grid>
          )}

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
          ) : showSensitiveColumns && (
            <Button variant="outlined" size="small" onClick={startEdit}>
              Edit
            </Button>
          )}
        </Box>
      </Box>

      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Room: Officials
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
          <AssignmentSection
            label="Quizmaster"
            currentId={room.quizmaster_id ?? null}
            currentName={qmName}
            occupiedIds={allOccupiedForQm}
            onAssign={user => handleAssign('quizmaster', user)}
            onRemove={() => handleRemove('quizmaster')}
            saving={saving}
            canEdit={showSensitiveColumns}
          />
          <AssignmentSection
            label="Content Judge"
            currentId={room.contentjudge_id ?? null}
            currentName={cjName}
            occupiedIds={allOccupiedForCj}
            onAssign={user => handleAssign('contentjudge', user)}
            onRemove={() => handleRemove('contentjudge')}
            saving={saving}
            canEdit={showSensitiveColumns}
          />
        </Box>
      </Box>
    </Stack>
  )
}
