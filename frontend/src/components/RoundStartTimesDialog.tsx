import { useEffect, useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import InputLabel from '@mui/material/InputLabel'
import List from '@mui/material/List'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemIcon from '@mui/material/ListItemIcon'
import ListItemText from '@mui/material/ListItemText'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import Alert from '@mui/material/Alert'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs, { type Dayjs } from 'dayjs'
import { TournamentAPI } from '../features/TournamentAPI'
import type { RoundTS } from '../features/RoundAPI'

interface Props {
  isOpen: boolean
  tid: string
  /** The division's rounds in chronological order; the user picks a consecutive run of them. */
  rounds: RoundTS[]
  onCancel: VoidFunction
  /** Apply: the chosen consecutive rounds (in order), the start time (ISO) of the first of them, and
   *  the per-round duration (minutes) to space them by. */
  onApply: (rounds: RoundTS[], startTimeIso: string, durationMinutes: number) => void
}

/** The selected rounds, as an inclusive [first, last] index range into `rounds`; null if none. */
type Range = { first: number, last: number } | null

/**
 * Re-times a consecutive run of rounds: the first selected round starts at the chosen time, and each
 * later selected round starts one "round duration" after the one before it. Rounds outside the run
 * are left alone. The duration is prefilled from the tournament's default round duration and can be
 * adjusted before applying.
 */
export const RoundStartTimesDialog = (props: Props) => {
  const { isOpen, tid, rounds, onCancel, onApply } = props
  const allRounds = (): Range => rounds.length > 0 ? { first: 0, last: rounds.length - 1 } : null
  const [range, setRange] = useState<Range>(allRounds)
  // The start time the user picked; undefined until they pick one, so it follows the selection.
  const [pickedStart, setPickedStart] = useState<Dayjs | null | undefined>(undefined)
  const [durationMinutes, setDurationMinutes] = useState(30)
  const [tournamentDefault, setTournamentDefault] = useState<number | null>(null)
  const [error, setError] = useState('')

  // On open: select every round and clear the picked start time. Done while rendering (not in an
  // effect) so the dialog never shows the previous selection for a frame.
  const [wasOpen, setWasOpen] = useState(isOpen)
  if (isOpen !== wasOpen) {
    setWasOpen(isOpen)
    if (isOpen) {
      setRange(allRounds())
      setPickedStart(undefined)
      setError('')
    }
  }

  // On open: pull the tournament's default round duration (which the user can then override below).
  useEffect(() => {
    if (!isOpen) return
    TournamentAPI.getById(tid)
      .then(t => {
        const d = t.default_round_duration ?? 30
        setTournamentDefault(d)
        setDurationMinutes(d)
      })
      .catch(() => { setTournamentDefault(null); setDurationMinutes(30) })
  }, [isOpen, tid])

  // Until the user picks a start time, it's the first selected round's current start.
  const firstStart = range ? rounds[range.first]?.scheduled_start_time ?? null : null
  const startTime = pickedStart !== undefined ? pickedStart : (firstStart ? dayjs(firstStart) : null)

  // Keeps the selection one consecutive run: checking a round extends the run to reach it, and only
  // the run's first or last round can be unchecked (the ones between them are locked).
  const toggleRound = (i: number) => {
    setError('')
    setRange(r => {
      if (!r) return { first: i, last: i }
      if (i < r.first) return { first: i, last: r.last }
      if (i > r.last) return { first: r.first, last: i }
      if (r.first === r.last) return null
      if (i === r.first) return { first: i + 1, last: r.last }
      if (i === r.last) return { first: r.first, last: i - 1 }
      return r
    })
  }

  const isSelected = (i: number) => !!range && i >= range.first && i <= range.last
  const isLocked = (i: number) => !!range && i > range.first && i < range.last
  const newStartOf = (i: number) =>
    range && startTime?.isValid() ? startTime.add((i - range.first) * durationMinutes, 'minute') : null
  const fmt = (d: Dayjs) => d.format('ddd MMM D · h:mm A')

  const handleApply = () => {
    if (!range) { setError('Select at least one round.'); return }
    if (!startTime || !startTime.isValid()) { setError('A valid start date and time is required.'); return }
    if (!Number.isFinite(durationMinutes) || durationMinutes < 1) { setError('Round duration must be at least 1 minute.'); return }
    onApply(rounds.slice(range.first, range.last + 1), startTime.toISOString(), durationMinutes)
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
      <DialogTitle>Edit Start Times</DialogTitle>
      <DialogContent dividers>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Box>
            <InputLabel>Rounds to re-time</InputLabel>
            <Typography variant="caption" color="text.secondary">
              Pick one run of consecutive rounds. Rounds between the first and last checked are always included.
            </Typography>
            {rounds.length === 0 ? (
              <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>This Division has no rounds.</Typography>
            ) : (
              <List dense disablePadding sx={{ mt: 0.5, maxHeight: 280, overflowY: 'auto', border: 1, borderColor: 'divider', borderRadius: 1 }}>
                {rounds.map((r, i) => {
                  const selected = isSelected(i)
                  const next = selected ? newStartOf(i) : null
                  const current = r.scheduled_start_time ? fmt(dayjs(r.scheduled_start_time)) : 'Unscheduled'
                  return (
                    <ListItemButton key={r.roundid} onClick={() => toggleRound(i)} disabled={isLocked(i)} sx={{ py: 0 }}>
                      <ListItemIcon sx={{ minWidth: 36 }}>
                        <Checkbox edge="start" size="small" checked={selected} tabIndex={-1} disableRipple />
                      </ListItemIcon>
                      <ListItemText
                        primary={r.name}
                        secondary={next ? `${current} → ${fmt(next)}` : current}
                      />
                    </ListItemButton>
                  )
                })}
              </List>
            )}
          </Box>

          {error && <Alert severity="error">{error}</Alert>}

          <LocalizationProvider dateAdapter={AdapterDayjs}>
            <Box>
              <InputLabel>Start (first selected round)</InputLabel>
              <DateTimePicker
                enableAccessibleFieldDOMStructure={false}
                value={startTime}
                onChange={(val) => setPickedStart(val)}
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
                ? `Tournament default: ${tournamentDefault} min. Adjusting here only affects these rounds.`
                : 'Spacing between consecutive rounds.'}
            </Typography>
          </Box>
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel}>Cancel</Button>
        <Button variant="contained" onClick={handleApply} disabled={!range}>Apply</Button>
      </DialogActions>
    </Dialog>
  )
}
