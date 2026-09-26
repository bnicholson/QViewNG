import { useEffect, useMemo, useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import FormControl from '@mui/material/FormControl'
import Select, { type SelectChangeEvent } from '@mui/material/Select'
import MenuItem from '@mui/material/MenuItem'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import Chip from '@mui/material/Chip'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import AddIcon from '@mui/icons-material/Add'
import RemoveIcon from '@mui/icons-material/Remove'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import { type Dayjs } from 'dayjs'
import { TournamentAPI } from '../features/TournamentAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'

interface Props {
  isOpen: boolean
  tid: string
  /** Total number of teams currently in the division being scheduled. */
  totalTeams: number
  onCancel: VoidFunction
}

/** Small +/- stepper for a positive integer value. */
function IncrementControl({ label, value, onChange, min = 1 }: { label: string; value: number; onChange: (v: number) => void; min?: number }) {
  return (
    <Box>
      <InputLabel sx={{ whiteSpace: 'normal' }}>{label}</InputLabel>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 0.5 }}>
        <IconButton size="small" onClick={() => onChange(Math.max(min, value - 1))} disabled={value <= min} aria-label={`Decrease ${label}`}>
          <RemoveIcon fontSize="small" />
        </IconButton>
        <TextField
          value={value}
          onChange={(e) => {
            const n = parseInt(e.target.value, 10)
            onChange(Number.isNaN(n) ? min : Math.max(min, n))
          }}
          size="small"
          type="number"
          slotProps={{ htmlInput: { min, style: { textAlign: 'center', width: 56 } } }}
        />
        <IconButton size="small" onClick={() => onChange(value + 1)} aria-label={`Increase ${label}`}>
          <AddIcon fontSize="small" />
        </IconButton>
      </Box>
    </Box>
  )
}

/**
 * Number of rounds a single round-robin needs for `n` teams: n-1 when n is even, n when odd (odd
 * needs a bye each round). Zero for fewer than two teams.
 */
function singleRoundRobinRounds(n: number): number {
  if (n < 2) return 0
  return n % 2 === 0 ? n - 1 : n
}

export const AutoSchedulePoolsDialog = (props: Props) => {
  const { isOpen, tid, totalTeams, onCancel } = props
  const [scheduleType, setScheduleType] = useState<'round_robin' | 'brackets'>('round_robin')
  const [teamsPerGame, setTeamsPerGame] = useState<2 | 3>(2)
  const [timesEachPlays, setTimesEachPlays] = useState(1)
  const [numPools, setNumPools] = useState(1)
  const [startTime, setStartTime] = useState<Dayjs | null>(null)
  const [roundDurationMin, setRoundDurationMin] = useState(30)
  // Rooms available to the schedule, and the ordered room ids the user has assigned to the needed slots.
  const [rooms, setRooms] = useState<RoomTS[]>([])
  const [roomOptionId, setRoomOptionId] = useState('')
  const [roomSlots, setRoomSlots] = useState<string[]>([])

  // Default the start time to 8am on the tournament's start date, and pull the tournament's default
  // round duration, whenever the dialog opens.
  useEffect(() => {
    if (!isOpen) return
    setTimesEachPlays(1)
    setNumPools(1)
    setTeamsPerGame(2)
    setStartTime(null)
    setRoundDurationMin(30)
    setRoomSlots([])
    setRoomOptionId('')
    TournamentAPI.getById(tid)
      .then(t => {
        if (t.fromdate) setStartTime(t.fromdate.hour(8).minute(0).second(0).millisecond(0))
        if (t.default_round_duration) setRoundDurationMin(t.default_round_duration)
      })
      .catch(() => { /* leave defaults if the tournament can't be loaded */ })
    RoomAPI.getByTournament(tid, 0, 500)
      .then(setRooms)
      .catch(() => { /* leave rooms empty if they can't be loaded */ })
  }, [isOpen, tid])

  // Rounds needed = single round-robin rounds for the largest pool, times how many times each pairing plays.
  // Pools play in parallel, so the pool with the most teams sets the round count.
  const teamsPerLargestPool = useMemo(
    () => (numPools > 0 ? Math.ceil(totalTeams / numPools) : totalTeams),
    [totalTeams, numPools],
  )
  const totalRounds = useMemo(
    () => singleRoundRobinRounds(teamsPerLargestPool) * timesEachPlays,
    [teamsPerLargestPool, timesEachPlays],
  )

  // Rooms needed per round = concurrent games in a round: each pool plays floor(poolSize / teamsPerGame)
  // games at once, across all pools. The 2-team/3-team toggle sets how many teams share a game/room.
  const roomsNeeded = useMemo(
    () => numPools * Math.floor(teamsPerLargestPool / teamsPerGame),
    [numPools, teamsPerLargestPool, teamsPerGame],
  )
  // Rooms still selectable (not already assigned to a slot).
  const availableRooms = useMemo(
    () => rooms.filter(r => !roomSlots.includes(r.roomid)),
    [rooms, roomSlots],
  )
  const roomName = (roomid: string) => rooms.find(r => r.roomid === roomid)?.name ?? roomid
  const slotsFilled = roomSlots.length === roomsNeeded

  // Total time duration = rounds × the tournament's default round duration; end time = start + duration.
  const durationMinutes = totalRounds * roundDurationMin
  const totalDuration = useMemo(() => {
    if (durationMinutes <= 0) return ''
    const h = Math.floor(durationMinutes / 60)
    const m = durationMinutes % 60
    return h > 0 ? `${h}h ${m}m (${durationMinutes} min)` : `${m}m`
  }, [durationMinutes])
  const endTime: Dayjs | null = startTime && durationMinutes > 0 ? startTime.add(durationMinutes, 'minute') : null

  return (
    <Dialog open={isOpen} onClose={onCancel} fullWidth maxWidth="sm">
      <DialogTitle>Auto-Schedule</DialogTitle>
      <DialogContent dividers>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Box>
            <InputLabel>Schedule type</InputLabel>
            <Select
              value={scheduleType}
              onChange={(e: SelectChangeEvent) => setScheduleType(e.target.value as 'round_robin' | 'brackets')}
              size="small"
              fullWidth
            >
              <MenuItem value="round_robin">Round-Robin</MenuItem>
              <MenuItem value="brackets">Tournament Bracket(s)</MenuItem>
            </Select>
          </Box>

          {scheduleType === 'brackets' ? (
            <Typography variant="body2" color="text.secondary">
              Tournament Bracket(s) scheduling isn't available yet.
            </Typography>
          ) : (
          <>
          <Box>
            <InputLabel>Teams per game</InputLabel>
            <ToggleButtonGroup
              exclusive
              size="small"
              value={teamsPerGame}
              onChange={(_e, v) => { if (v) setTeamsPerGame(v) }}
              sx={{ mt: 0.5 }}
            >
              <ToggleButton value={2}>2-Team</ToggleButton>
              <ToggleButton value={3}>3-Team</ToggleButton>
            </ToggleButtonGroup>
          </Box>
          <IncrementControl
            label="# of Pools"
            value={numPools}
            onChange={setNumPools}
          />
          <IncrementControl
            label="# of times each Team plays every other Team within their Pool"
            value={timesEachPlays}
            onChange={setTimesEachPlays}
          />

          <Divider />

          <Box>
            <Typography variant="caption" color="text.secondary">Total Teams in the Division</Typography>
            <Typography variant="body1">{totalTeams}</Typography>
          </Box>

          <Box>
            <Typography variant="caption" color="text.secondary">Rounds required</Typography>
            <Typography variant="body1">{totalRounds}</Typography>
          </Box>

          <Divider />

          {/* Assign a room to each concurrent-game slot needed per round. All slots must be filled. */}
          <Box>
            <InputLabel>Rooms</InputLabel>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 0.5, flexWrap: 'wrap' }}>
              <FormControl size="small" sx={{ minWidth: 180 }}>
                <Select
                  value={availableRooms.some(r => r.roomid === roomOptionId) ? roomOptionId : ''}
                  onChange={(e: SelectChangeEvent) => setRoomOptionId(e.target.value)}
                  displayEmpty
                  renderValue={(v) => v ? roomName(v) : <em>Select a room</em>}
                >
                  {availableRooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
                </Select>
              </FormControl>
              <Button
                variant="outlined"
                disabled={!roomOptionId || roomSlots.length >= roomsNeeded}
                onClick={() => { if (roomOptionId) { setRoomSlots(s => [...s, roomOptionId]); setRoomOptionId('') } }}
              >
                Use
              </Button>
              <Typography variant="caption" color={slotsFilled ? 'text.secondary' : 'error'}>
                {roomSlots.length} / {roomsNeeded} room slots filled
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, mt: 1 }}>
              {roomSlots.map((rid, i) => (
                <Chip
                  key={`${rid}-${i}`}
                  label={roomName(rid)}
                  onClick={() => setRoomSlots(s => s.filter((_, idx) => idx !== i))}
                  onDelete={() => setRoomSlots(s => s.filter((_, idx) => idx !== i))}
                />
              ))}
            </Box>
          </Box>

          <Divider />

          <IncrementControl
            label="Duration per Round (minutes)"
            value={roundDurationMin}
            onChange={setRoundDurationMin}
          />

          <TextField
            label="Total time duration"
            value={totalDuration}
            slotProps={{ inputLabel: { shrink: true }, htmlInput: { readOnly: true } }}
            placeholder="—"
            size="small"
            fullWidth
          />

          <LocalizationProvider dateAdapter={AdapterDayjs}>
            <Box>
              <InputLabel>Start time</InputLabel>
              <DateTimePicker
                enableAccessibleFieldDOMStructure={false}
                value={startTime}
                onChange={(val) => setStartTime(val)}
                slots={{ textField: TextField }}
                slotProps={{ textField: { size: 'small', fullWidth: true } }}
              />
            </Box>
            <Box>
              <InputLabel>End time</InputLabel>
              <DateTimePicker
                enableAccessibleFieldDOMStructure={false}
                value={endTime}
                onChange={() => { /* derived from start time + duration once available */ }}
                readOnly
                slots={{ textField: TextField }}
                slotProps={{ textField: { size: 'small', fullWidth: true } }}
              />
            </Box>
          </LocalizationProvider>
          </>
          )}
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel}>Cancel</Button>
        <Button variant="contained" onClick={onCancel} disabled={scheduleType === 'round_robin' && !slotsFilled}>Auto-Schedule</Button>
      </DialogActions>
    </Dialog>
  )
}
