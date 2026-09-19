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
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import AddIcon from '@mui/icons-material/Add'
import RemoveIcon from '@mui/icons-material/Remove'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs, { type Dayjs } from 'dayjs'
import { TournamentAPI } from '../features/TournamentAPI'

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
  const [timesEachPlays, setTimesEachPlays] = useState(1)
  const [numPools, setNumPools] = useState(1)
  const [startTime, setStartTime] = useState<Dayjs | null>(null)

  // Default the start time to 8am on the tournament's start date whenever the dialog opens.
  useEffect(() => {
    if (!isOpen) return
    setTimesEachPlays(1)
    setNumPools(1)
    setStartTime(null)
    TournamentAPI.getById(tid)
      .then(t => { if (t.fromdate) setStartTime(t.fromdate.hour(8).minute(0).second(0).millisecond(0)) })
      .catch(() => { /* leave start time blank if the tournament can't be loaded */ })
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

  // Total time duration and end time depend on a per-round duration that isn't wired up yet — left blank for now.
  const totalDuration = ''
  const endTime: Dayjs | null = null

  return (
    <Dialog open={isOpen} onClose={onCancel} fullWidth maxWidth="sm">
      <DialogTitle>Auto-Schedule Pools</DialogTitle>
      <DialogContent dividers>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <IncrementControl
            label="# of times each Team plays every other Team"
            value={timesEachPlays}
            onChange={setTimesEachPlays}
          />
          <IncrementControl
            label="# of Pools"
            value={numPools}
            onChange={setNumPools}
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
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel}>Cancel</Button>
        <Button variant="contained" onClick={onCancel}>Auto-Schedule</Button>
      </DialogActions>
    </Dialog>
  )
}
