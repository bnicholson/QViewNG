import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Stack from '@mui/material/Stack'
import Switch from '@mui/material/Switch'
import FormControlLabel from '@mui/material/FormControlLabel'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import { GameAPI, type GameTS, type GameChangeset } from '../features/GameAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import type { TournamentTS } from '../features/TournamentAPI'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit',
  })
}

function userLabel(u: UserTS): string {
  return [u.fname, u.mname, u.lname].filter(Boolean).join(' ') + ` (@${u.username})`
}

interface Lookups {
  divisions: DivisionTS[]
  rooms: RoomTS[]
  rounds: RoundTS[]
  teams: TeamTS[]
  users: UserTS[]
}

interface FormState {
  org: string
  divisionid: string
  roomid: string
  roundid: string
  ruleset: string
  ignore: boolean
  leftteamid: string
  centerteamid: string
  rightteamid: string
  quizmasterid: string
  contentjudgeid: string
}

interface Props {
  game: GameTS
  tournament: TournamentTS
  onUpdated: (game: GameTS) => void
}

export const GameProfileOverviewPage = ({ game, tournament, onUpdated }: Props) => {
  const [editing, setEditing] = useState(false)
  const [form, setForm] = useState<FormState | null>(null)
  const [lookups, setLookups] = useState<Lookups | null>(null)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    Promise.all([
      DivisionAPI.getByTournament(tournament.tid, 0, 200),
      RoomAPI.getByTournament(tournament.tid, 0, 200),
      RoundAPI.getByTournament(tournament.tid, 0, 500),
      TeamAPI.getByTournament(tournament.tid, 0, 500),
      UserAPI.get(0, 500),
    ]).then(([divisions, rooms, rounds, teamResult, userResult]) => {
      setLookups({ divisions, rooms, rounds, teams: teamResult.items, users: userResult.items })
    }).catch(() => setError('Failed to load lookup data.'))
  }, [tournament.tid])

  const startEdit = () => {
    setForm({
      org: game.org,
      divisionid: game.divisionid,
      roomid: game.roomid,
      roundid: game.roundid,
      ruleset: game.ruleset,
      ignore: game.ignore,
      leftteamid: game.leftteamid,
      centerteamid: game.centerteamid ?? '',
      rightteamid: game.rightteamid,
      quizmasterid: game.quizmasterid,
      contentjudgeid: game.contentjudgeid ?? '',
    })
    setError(null)
    setEditing(true)
  }

  const cancelEdit = () => {
    setEditing(false)
    setError(null)
    setForm(null)
  }

  const set = (patch: Partial<FormState>) => setForm(f => f ? { ...f, ...patch } : f)

  const handleSave = async () => {
    if (!form) return
    if (!form.org.trim()) { setError('Org is required.'); return }
    if (!form.divisionid) { setError('Division is required.'); return }
    if (!form.roomid) { setError('Room is required.'); return }
    if (!form.roundid) { setError('Round is required.'); return }
    if (!form.ruleset.trim()) { setError('Ruleset is required.'); return }
    if (!form.leftteamid) { setError('Left team is required.'); return }
    if (!form.rightteamid) { setError('Right team is required.'); return }
    if (!form.quizmasterid) { setError('Quizmaster is required.'); return }

    setSaving(true)
    setError(null)
    try {
      const changeset: GameChangeset = {
        org: form.org,
        divisionid: form.divisionid,
        roomid: form.roomid,
        roundid: form.roundid,
        ruleset: form.ruleset,
        ignore: form.ignore,
        leftteamid: form.leftteamid,
        centerteamid: form.centerteamid || null,
        rightteamid: form.rightteamid,
        quizmasterid: form.quizmasterid,
        contentjudgeid: form.contentjudgeid || null,
      }
      const updated = await GameAPI.update(game.gid, changeset)
      onUpdated(updated)
      setEditing(false)
      setForm(null)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSaving(false)
    }
  }

  // Lookup helpers for display
  const divMap = new Map(lookups?.divisions.map(d => [d.did, d.dname]) ?? [])
  const roomMap = new Map(lookups?.rooms.map(r => [r.roomid, r.name]) ?? [])
  const roundMap = new Map(lookups?.rounds.map(r => [r.roundid, r.scheduled_start_time]) ?? [])
  const teamMap = new Map(lookups?.teams.map(t => [t.teamid, t.name]) ?? [])
  const userMap = new Map(lookups?.users.map(u => [u.id, userLabel(u)]) ?? [])

  const teamLabel = (id: string | null) => id ? (teamMap.get(id) ?? id) : '—'
  const userDisplay = (id: string | null) => id ? (userMap.get(id) ?? id) : '—'

  return (
    <Stack spacing={3}>
      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Game: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>
        )}

        <Grid container spacing={{ xs: 1, sm: 2 }}>

          {/* Tournament */}
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

          {/* Org */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Org</Typography>
            {editing && form ? (
              <TextField size="small" value={form.org} onChange={e => set({ org: e.target.value })} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{game.org}</Typography>
            )}
          </Grid>

          {/* Ruleset */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Ruleset</Typography>
            {editing && form ? (
              <TextField size="small" value={form.ruleset} onChange={e => set({ ruleset: e.target.value })} fullWidth sx={{ mt: 0.5 }} />
            ) : (
              <Typography variant="body1">{game.ruleset}</Typography>
            )}
          </Grid>

          {/* Division */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Division</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.divisionid} onChange={e => set({ divisionid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{divMap.get(game.divisionid) ?? game.divisionid}</Typography>
            )}
          </Grid>

          {/* Room */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Room</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.roomid} onChange={e => set({ roomid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.rooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{roomMap.get(game.roomid) ?? game.roomid}</Typography>
            )}
          </Grid>

          {/* Round */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Round</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.roundid} onChange={e => set({ roundid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.rounds.map(r => <MenuItem key={r.roundid} value={r.roundid}>{formatDateTime(r.scheduled_start_time)}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{formatDateTime(roundMap.get(game.roundid))}</Typography>
            )}
          </Grid>

          {/* Left Team */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Left Team</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.leftteamid} onChange={e => set({ leftteamid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{teamLabel(game.leftteamid)}</Typography>
            )}
          </Grid>

          {/* Center Team */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Center Team</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.centerteamid} onChange={e => set({ centerteamid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                <MenuItem value=""><em>None</em></MenuItem>
                {lookups?.teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{teamLabel(game.centerteamid ?? null)}</Typography>
            )}
          </Grid>

          {/* Right Team */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Right Team</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.rightteamid} onChange={e => set({ rightteamid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{teamLabel(game.rightteamid)}</Typography>
            )}
          </Grid>

          {/* Quizmaster */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Quizmaster</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.quizmasterid} onChange={e => set({ quizmasterid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                {lookups?.users.map(u => <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{userDisplay(game.quizmasterid)}</Typography>
            )}
          </Grid>

          {/* Content Judge */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Content Judge</Typography>
            {editing && form ? (
              <Select size="small" fullWidth value={form.contentjudgeid} onChange={e => set({ contentjudgeid: e.target.value })} displayEmpty sx={{ mt: 0.5 }}>
                <MenuItem value=""><em>None</em></MenuItem>
                {lookups?.users.map(u => <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>)}
              </Select>
            ) : (
              <Typography variant="body1">{userDisplay(game.contentjudgeid ?? null)}</Typography>
            )}
          </Grid>

          {/* Ignore */}
          <Grid item xs={12} sm={6} md={4} sx={{ display: 'flex', alignItems: 'center' }}>
            {editing && form ? (
              <FormControlLabel
                control={<Switch checked={form.ignore} onChange={e => set({ ignore: e.target.checked })} />}
                label="Ignore"
              />
            ) : (
              <Box>
                <Typography variant="body2" color="text.secondary">Ignore</Typography>
                <Chip label={game.ignore ? 'Yes' : 'No'} size="small" color={game.ignore ? 'warning' : 'default'} sx={{ mt: 0.5 }} />
              </Box>
            )}
          </Grid>

          {/* Timestamps */}
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(game.created_at)}</Typography>
          </Grid>
          <Grid item xs={12} sm={6} md={4}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(game.updated_at)}</Typography>
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
