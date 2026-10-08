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
import Checkbox from '@mui/material/Checkbox'
import FormControlLabel from '@mui/material/FormControlLabel'
import Alert from '@mui/material/Alert'
import CircularProgress from '@mui/material/CircularProgress'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import AddIcon from '@mui/icons-material/Add'
import RemoveIcon from '@mui/icons-material/Remove'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import { type Dayjs } from 'dayjs'
import { TournamentAPI } from '../features/TournamentAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { ScheduleAPI } from '../features/ScheduleAPI'
import { useAuth } from '../hooks/useAuth'

interface Props {
  isOpen: boolean
  tid: string
  /** The division being scheduled — its teams are placed into pools and its games are created. */
  did: string
  /** Total number of teams currently in the division being scheduled. */
  totalTeams: number
  /**
   * Advanced mode (default) exposes the pool count, the team-shuffling option and the names of the
   * sessions to be created. In Normal mode those are hidden and fixed at one pool / one pool group /
   * one session per repeat — still created in the background, just never shown or named here.
   */
  advanced?: boolean
  onCancel: VoidFunction
  /** Called after a schedule is successfully created so the parent can reload its data. */
  onCreated?: () => void
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
  const { isOpen, tid, did, totalTeams, advanced = true, onCancel, onCreated } = props
  const { accessToken } = useAuth()
  const [scheduleType, setScheduleType] = useState<'round_robin' | 'brackets'>('round_robin')
  const [teamsPerGame, setTeamsPerGame] = useState<2 | 3 | 'mixed'>(2)
  const [timesEachPlays, setTimesEachPlays] = useState(1)
  // Whether to re-shuffle teams into new pools for each session's pool group (only shown/used with 2+ plays).
  const [shuffleTeams, setShuffleTeams] = useState(true)
  const [numPools, setNumPools] = useState(1)
  const [startTime, setStartTime] = useState<Dayjs | null>(null)
  const [roundDurationMin, setRoundDurationMin] = useState(30)
  // Rooms available to the schedule, and the ordered room ids the user has assigned to the needed slots.
  const [rooms, setRooms] = useState<RoomTS[]>([])
  const [roomSlots, setRoomSlots] = useState<string[]>([])
  // One editable name per session to be created (count = timesEachPlays).
  const [sessionNames, setSessionNames] = useState<string[]>(['Session 1'])
  const [creating, setCreating] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Default the start time to 8am on the tournament's start date, and pull the tournament's default
  // round duration, whenever the dialog opens.
  useEffect(() => {
    if (!isOpen) return
    setTimesEachPlays(1)
    setShuffleTeams(true)
    setNumPools(1)
    setTeamsPerGame(2)
    setStartTime(null)
    setRoundDurationMin(30)
    setRoomSlots([])
    setSessionNames(['Session 1'])
    setCreating(false)
    setError(null)
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

  // One session is created for each time every team plays, so keep sessionNames sized to timesEachPlays.
  useEffect(() => {
    setSessionNames(prev => {
      if (prev.length === timesEachPlays) return prev
      const next = prev.slice(0, timesEachPlays)
      while (next.length < timesEachPlays) next.push(`Session ${next.length + 1}`)
      return next
    })
  }, [timesEachPlays])

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
  // Total games across the whole schedule: each pool plays one single round-robin per session, so a
  // pool of `size` teams contributes size*(size-1)/2 games; summed over pools × the session count.
  const totalGames = useMemo(() => {
    if (numPools < 1 || totalTeams < 2) return 0
    const divisor = teamsPerGame === 'mixed' ? 2 : teamsPerGame
    const base = Math.floor(totalTeams / numPools)
    const remainder = totalTeams % numPools
    let perSession = 0
    for (let p = 0; p < numPools; p++) {
      const size = base + (p < remainder ? 1 : 0)
      // Games per round in this pool × the pool's round count. For 2-team this equals size*(size-1)/2.
      perSession += Math.floor(size / divisor) * singleRoundRobinRounds(size)
    }
    return perSession * timesEachPlays
  }, [totalTeams, numPools, timesEachPlays, teamsPerGame])
  // Games (and therefore rooms) in a full round = concurrent games across all pools. Each pool of
  // `size` teams runs floor(size / teamsPerGame) games at once; the 2-/3-team toggle sets how many
  // teams share a game/room, and "Mixed" uses 2 (the smaller size) — the worst case that needs the
  // most rooms. Summed over the actual (possibly uneven) pool sizes.
  const gamesPerRound = useMemo(() => {
    if (numPools < 1 || totalTeams < 2) return 0
    const divisor = teamsPerGame === 'mixed' ? 2 : teamsPerGame
    const base = Math.floor(totalTeams / numPools)
    const remainder = totalTeams % numPools
    let games = 0
    for (let p = 0; p < numPools; p++) {
      const size = base + (p < remainder ? 1 : 0)
      games += Math.floor(size / divisor)
    }
    return games
  }, [totalTeams, numPools, teamsPerGame])

  // Rooms needed per round equals the concurrent games per round.
  const roomsNeeded = gamesPerRound
  // Rooms still selectable (not already assigned to a slot), name-sorted numerically so "Room 10"
  // follows "Room 9" rather than "Room 1".
  const availableRooms = useMemo(
    () => rooms
      .filter(r => !roomSlots.includes(r.roomid))
      .sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true })),
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

  // Create-only and transactional: the backend builds the sessions, pool groups, pools, team
  // placements, rounds and games in one transaction — on any failure nothing is created. Each
  // session is one full single round-robin per pool (so each pair meets `timesEachPlays` times
  // overall). The Teams-per-game toggle only sizes the room slots; matchups are 2-team games.
  const createSchedule = async () => {
    setCreating(true)
    setError(null)
    try {
      await ScheduleAPI.create({
        did,
        // Normal mode: one pool holding every team, so there is no pool division to shuffle. The
        // session names are generated, not entered — the user never sees them.
        session_names: sessionNames,
        num_pools: advanced ? numPools : 1,
        round_duration_minutes: roundDurationMin,
        start_time: startTime ? startTime.toISOString() : null,
        room_slots: roomSlots,
        shuffle_teams: advanced ? shuffleTeams : false,
      }, accessToken)
      onCreated?.()
      onCancel()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to create the schedule.')
    } finally {
      setCreating(false)
    }
  }

  return (
    <Dialog open={isOpen} onClose={creating ? undefined : onCancel} fullWidth maxWidth="sm">
      <DialogTitle>Create Schedule</DialogTitle>
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
              <ToggleButton value={2}>2-Team Only</ToggleButton>
              <ToggleButton value={3}>3-Team Only</ToggleButton>
              <ToggleButton value="mixed">Mixed</ToggleButton>
            </ToggleButtonGroup>
          </Box>
          {advanced && (
            <IncrementControl
              label="# of Pools"
              value={numPools}
              onChange={setNumPools}
            />
          )}
          <IncrementControl
            label={advanced
              ? '# of Sessions / # of times each Team plays every other Team within their Pool'
              : '# of times each Team plays every other Team'}
            value={timesEachPlays}
            onChange={setTimesEachPlays}
          />
          {/* Derived from the settings above. */}
          <Box sx={{ display: 'flex', gap: 4 }}>
            <Box>
              <Typography variant="caption" color="text.secondary">Total Teams in the Division</Typography>
              <Typography variant="body1">{totalTeams}</Typography>
            </Box>
            <Box>
              <Typography variant="caption" color="text.secondary">Games/Rooms per Round</Typography>
              <Typography variant="body1">{gamesPerRound}</Typography>
            </Box>
          </Box>
          <Box sx={{ display: 'flex', gap: 4 }}>
            <Box>
              <Typography variant="caption" color="text.secondary"># of Rounds</Typography>
              <Typography variant="body1">{totalRounds}</Typography>
            </Box>
            <Box>
              <Typography variant="caption" color="text.secondary"># of Games</Typography>
              <Typography variant="body1">{totalGames}</Typography>
            </Box>
          </Box>
          {advanced && timesEachPlays > 1 && (
            <FormControlLabel
              control={<Checkbox size="small" checked={shuffleTeams} onChange={(e) => setShuffleTeams(e.target.checked)} />}
              label={<Typography variant="body2">Shuffle Teams into New Pools for each Pool Group</Typography>}
            />
          )}

          <Divider />

          {/* Assign a room to each concurrent-game slot needed per round. All slots must be filled. */}
          <Box>
            <InputLabel>Rooms</InputLabel>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 0.5, flexWrap: 'wrap' }}>
              <FormControl size="small" sx={{ minWidth: 180 }} disabled={roomSlots.length >= roomsNeeded}>
                <Select
                  // Selecting a room adds it to the slots immediately, then resets back to "Select a room".
                  value=""
                  onChange={(e: SelectChangeEvent) => { const rid = e.target.value; if (rid && roomSlots.length < roomsNeeded) setRoomSlots(s => [...s, rid]) }}
                  displayEmpty
                  renderValue={() => <em>Select a room</em>}
                >
                  {availableRooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
                </Select>
              </FormControl>
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

          {advanced && <Divider />}

          {/* One session is created per "times each team plays"; each can be renamed here. */}
          {advanced && (
          <Box>
            <InputLabel sx={{ whiteSpace: 'normal' }}>Sessions to create</InputLabel>
            <Typography variant="caption" color="text.secondary">
              One session is created for each time every team plays. Rename any of them below.
            </Typography>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1.5, mt: 1 }}>
              {sessionNames.map((name, i) => (
                <Box key={i} sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>{i + 1}:</Typography>
                  <TextField
                    value={name}
                    onChange={(e) => setSessionNames(prev => prev.map((v, idx) => (idx === i ? e.target.value : v)))}
                    size="small"
                    fullWidth
                  />
                </Box>
              ))}
            </Box>
          </Box>
          )}
          </>
          )}

          {error && <Alert severity="error">{error}</Alert>}
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel} disabled={creating}>Cancel</Button>
        <Button
          variant="contained"
          onClick={createSchedule}
          disabled={creating || scheduleType !== 'round_robin' || !slotsFilled}
          startIcon={creating ? <CircularProgress size={16} color="inherit" /> : undefined}
        >
          {creating ? 'Creating…' : 'Create Schedule'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
