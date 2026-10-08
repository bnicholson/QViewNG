import { useEffect, useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import InputLabel from '@mui/material/InputLabel'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import Alert from '@mui/material/Alert'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs, { type Dayjs } from 'dayjs'
import { TournamentAPI } from '../features/TournamentAPI'

interface Props {
  isOpen: boolean
  tid: string
  /** The session being scheduled — shown in the title. */
  sessionName: string
  /** The session's current first-round start (ISO), used to prefill the picker; null if unscheduled. */
  defaultStart: string | null
  /** How many rounds will be re-timed (shown so the user knows the span this affects). */
  roundCount: number
  /**
   * Normal scheduling mode: word everything in terms of the rounds being re-timed rather than the
   * session they belong to (sessions aren't a concept the user manages there). `sessionName` is
   * then unused.
   */
  hideSessionWording?: boolean
  onCancel: VoidFunction
  /** Apply: the chosen start time (ISO) and the per-round duration (minutes) to space the rounds by. */
  onApply: (startTimeIso: string, durationMinutes: number) => void
}

/**
 * Sets a session's start time: the first (chronological) round starts at the chosen time, and each
 * later round starts one "round duration" after the one before it. The duration is prefilled from the
 * tournament's default round duration and can be adjusted before applying.
 */
export const SessionStartTimeDialog = (props: Props) => {
  const { isOpen, tid, sessionName, defaultStart, roundCount, hideSessionWording = false, onCancel, onApply } = props
  const [startTime, setStartTime] = useState<Dayjs | null>(null)
  const [durationMinutes, setDurationMinutes] = useState(30)
  const [tournamentDefault, setTournamentDefault] = useState<number | null>(null)
  const [error, setError] = useState('')

  // On open: reset the start time to the session's current start and pull the tournament's default
  // round duration (which the user can then override below).
  useEffect(() => {
    if (!isOpen) return
    setStartTime(defaultStart ? dayjs(defaultStart) : null)
    setError('')
    TournamentAPI.getById(tid)
      .then(t => {
        const d = t.default_round_duration ?? 30
        setTournamentDefault(d)
        setDurationMinutes(d)
      })
      .catch(() => { setTournamentDefault(null); setDurationMinutes(30) })
  }, [isOpen, tid, defaultStart])

  const handleApply = () => {
    if (!startTime || !startTime.isValid()) { setError('A valid start date and time is required.'); return }
    if (!Number.isFinite(durationMinutes) || durationMinutes < 1) { setError('Round duration must be at least 1 minute.'); return }
    onApply(startTime.toISOString(), durationMinutes)
  }

  return (
    <Dialog
      open={isOpen}
      onClose={onCancel}
      fullWidth
      maxWidth="sm"
      // Anchor near the top so the date/time picker popup has room to open downward (and its OK
      // button stays on-screen) instead of hanging off the bottom of a vertically-centered dialog.
      sx={{ '& .MuiDialog-container': { alignItems: 'flex-start' } }}
      slotProps={{ paper: { sx: { mt: 4 } } }}
    >
      <DialogTitle>{hideSessionWording ? 'Edit Start Times' : `Edit Start Time — ${sessionName}`}</DialogTitle>
      <DialogContent dividers>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Alert severity="info">
            Applying this updates the start time of{' '}
            <strong>{hideSessionWording ? 'every round in this schedule' : 'every round in this session'}</strong>: the first round
            starts at the time you choose, and each following round is spaced one round-duration after the previous
            one — keeping {hideSessionWording ? 'the schedule' : 'the session'} on a consistent, evenly-spaced schedule.
            This re-times {roundCount} {roundCount === 1 ? 'round' : 'rounds'}.
          </Alert>

          {error && <Alert severity="error">{error}</Alert>}

          <LocalizationProvider dateAdapter={AdapterDayjs}>
            <Box>
              <InputLabel>{hideSessionWording ? 'Start (first round)' : 'Session start (first round)'}</InputLabel>
              <DateTimePicker
                enableAccessibleFieldDOMStructure={false}
                value={startTime}
                onChange={(val) => setStartTime(val)}
                slots={{ textField: TextField }}
                slotProps={{ textField: { size: 'small', fullWidth: true } }}
              />
            </Box>
          </LocalizationProvider>

          <Box>
            <InputLabel>Round duration (minutes)</InputLabel>
            <TextField
              type="number"
              size="small"
              fullWidth
              value={durationMinutes}
              slotProps={{ htmlInput: { min: 1 } }}
              onChange={(e) => {
                const n = parseInt(e.target.value, 10)
                setDurationMinutes(Number.isNaN(n) ? 1 : Math.max(1, n))
              }}
            />
            <Typography variant="caption" color="text.secondary">
              {tournamentDefault != null
                ? `Tournament default: ${tournamentDefault} min. Adjusting here only affects ${hideSessionWording ? 'these rounds' : 'this session'}.`
                : 'Spacing between consecutive rounds.'}
            </Typography>
          </Box>
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel}>Cancel</Button>
        <Button variant="contained" onClick={handleApply}>Apply</Button>
      </DialogActions>
    </Dialog>
  )
}
