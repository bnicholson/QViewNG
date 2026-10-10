import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Divider from '@mui/material/Divider'
import Grid from '@mui/material/Grid'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import { type RoomTS } from '../features/RoomAPI'
import { RoomGroupAPI, type RoomGroupTS } from '../features/RoomGroupAPI'
import { UserAPI } from '../features/UserAPI'
import type { TournamentTS } from '../features/TournamentAPI'
import { RoomEditorDialog } from '../components/RoomEditorDialog'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

function ProfileLink({ to, name }: { to: string; name: string }) {
  return (
    <Link
      to={to}
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
  /** Owner / system admin: may edit the room (via the editor dialog) and see its Client Key. */
  showSensitiveColumns?: boolean
}

export const RoomProfileOverviewPage = ({ room, tournament, onUpdated, showSensitiveColumns = false }: Props) => {
  const [editorOpen, setEditorOpen] = useState(false)

  // Resolved display names for the building and the current QM/CJ.
  // The loaded building; shown only while it is still the room's building.
  const [building, setBuilding] = useState<RoomGroupTS | null>(null)
  const buildingName = building && building.roomgroupid === room.roomgroupid ? building.name : null
  const [qmName, setQmName] = useState<string | null>(null)
  const [cjName, setCjName] = useState<string | null>(null)

  useEffect(() => {
    if (!room.roomgroupid) return
    RoomGroupAPI.getById(room.roomgroupid)
      .then(setBuilding)
      .catch(() => setBuilding(null))
  }, [room.roomgroupid])

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

  const official = (id: string | null, name: string | null, label: string) => (
    <Box>
      <Typography variant="body2" color="text.secondary">{label}</Typography>
      <Typography variant="body1">
        {id && name ? <ProfileLink to={`/user/${id}/overview`} name={name} /> : '—'}
      </Typography>
    </Box>
  )

  return (
    <Stack spacing={3}>
      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Room: General Info
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Grid container spacing={{ xs: 1, sm: 2 }}>
          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Tournament</Typography>
            <Typography variant="body1">
              <ProfileLink to={`/tournament/${tournament.tid}/overview`} name={tournament.tname} />
            </Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Name</Typography>
            <Typography variant="body1">{room.name}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Building</Typography>
            <Typography variant="body1">
              {room.roomgroupid && buildingName
                ? <ProfileLink to={`/building/${room.roomgroupid}/overview`} name={buildingName} />
                : '—'}
            </Typography>
          </Grid>

          {showSensitiveColumns && (
            <Grid size={{ xs: 12, sm: 6, md: 4 }}>
              <Typography variant="body2" color="text.secondary">Client Key</Typography>
              <Typography variant="body1">{room.clientkey || '—'}</Typography>
            </Grid>
          )}

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Supports 3-team games</Typography>
            <Typography variant="body1">{room.supports_threeteam_games ? 'Yes' : 'No'}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Created</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(room.created_at)}</Typography>
          </Grid>

          <Grid size={{ xs: 12, sm: 6, md: 4 }}>
            <Typography variant="body2" color="text.secondary">Last Modified</Typography>
            <Typography variant="body1" color="text.secondary">{formatDate(room.updated_at)}</Typography>
          </Grid>

          <Grid size={{ xs: 12 }}>
            <Typography variant="body2" color="text.secondary">Comments</Typography>
            <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>{room.comments || '—'}</Typography>
          </Grid>
        </Grid>
      </Box>

      <Box>
        <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
          Room: Officials
        </Typography>
        <Divider sx={{ mb: 2 }} />

        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
          {official(room.quizmaster_id, qmName, 'Quizmaster')}
          {official(room.contentjudge_id, cjName, 'Content Judge')}
        </Box>
      </Box>

      {showSensitiveColumns && (
        <Box>
          <Button variant="outlined" size="small" onClick={() => setEditorOpen(true)}>Edit</Button>
        </Box>
      )}

      <RoomEditorDialog
        tid={tournament.tid.toString()}
        room={room}
        showSensitiveFields={showSensitiveColumns}
        isOpen={editorOpen}
        onCancel={() => setEditorOpen(false)}
        onSave={(updated) => { setEditorOpen(false); onUpdated(updated) }}
      />
    </Stack>
  )
}
