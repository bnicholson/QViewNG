import { useEffect, useState } from 'react'
import Alert from '@mui/material/Alert'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import IconButton from '@mui/material/IconButton'
import Paper from '@mui/material/Paper'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import DragIndicatorIcon from '@mui/icons-material/DragIndicator'
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp'
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown'
import {
  StatsGroupAPI,
  TIE_BREAKER_LABELS,
  type TieBreakerSettingTS,
} from '../features/StatsGroupAPI'
import { useAuth } from '../hooks/useAuth'

interface Props {
  statsGroupId: string
  isOpen: boolean
  onCancel: () => void
  /** Called with the saved order so the caller can re-sort the standings. */
  onSaved: (rules: TieBreakerSettingTS[]) => void
}

/**
 * Orders the criteria used to separate teams that are level in the standings.
 *
 * Rows are dragged to reorder and unchecked to leave a criterion out. An unchecked criterion keeps
 * its position, so re-checking it puts it back where it was. The numbers shown are the rule numbers
 * that appear in the "Rank Rules" explanation on the Team Stats table.
 */
export default function TieBreakerRulesDialog({ statsGroupId, isOpen, onCancel, onSaved }: Props) {
  const { accessToken } = useAuth()
  const [rules, setRules] = useState<TieBreakerSettingTS[]>([])
  const [loading, setLoading] = useState(false)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [dragIndex, setDragIndex] = useState<number | null>(null)
  const [overIndex, setOverIndex] = useState<number | null>(null)

  // Load on open so the dialog always reflects what is stored — and so Cancel discards edits
  // simply by not writing them anywhere.
  useEffect(() => {
    if (!isOpen || !statsGroupId) return
    let cancelled = false
    setLoading(true)
    setError(null)
    StatsGroupAPI.getTieBreakerRules(statsGroupId, accessToken)
      .then((r) => { if (!cancelled) setRules(r) })
      .catch((e) => { if (!cancelled) setError(e.message) })
      .finally(() => { if (!cancelled) setLoading(false) })
    return () => { cancelled = true }
  }, [isOpen, statsGroupId, accessToken])

  const move = (from: number, to: number) => {
    if (to < 0 || to >= rules.length || from === to) return
    setRules((prev) => {
      const next = [...prev]
      const [moved] = next.splice(from, 1)
      next.splice(to, 0, moved)
      return next
    })
  }

  const toggle = (index: number) =>
    setRules((prev) => prev.map((r, i) => (i === index ? { ...r, enabled: !r.enabled } : r)))

  const save = async () => {
    setSaving(true)
    setError(null)
    try {
      const saved = await StatsGroupAPI.saveTieBreakerRules(statsGroupId, rules, accessToken)
      onSaved(saved)
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to save tie-breaking rules.')
    } finally {
      setSaving(false)
    }
  }

  const noneEnabled = rules.length > 0 && rules.every((r) => !r.enabled)

  return (
    <Dialog open={isOpen} onClose={onCancel} maxWidth="sm" fullWidth>
      <DialogTitle>Tie-breaking Rules</DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          Teams that are level are separated by these criteria in order, top first. Drag a row to
          move it, or uncheck a criterion to leave it out. The numbers are the rule numbers shown in
          the Team Stats "Rank Rules" explanation.
        </Typography>

        {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
        {noneEnabled && (
          <Alert severity="warning" sx={{ mb: 2 }}>
            With every criterion unchecked, level teams fall back to alphabetical order by name.
          </Alert>
        )}
        {loading && <Typography variant="body2">Loading…</Typography>}

        <Stack spacing={1}>
          {rules.map((setting, index) => (
            <Paper
              key={setting.rule}
              variant="outlined"
              draggable
              onDragStart={() => setDragIndex(index)}
              onDragEnter={() => setOverIndex(index)}
              onDragOver={(e) => e.preventDefault()}
              onDragEnd={() => { setDragIndex(null); setOverIndex(null) }}
              onDrop={(e) => {
                e.preventDefault()
                if (dragIndex !== null) move(dragIndex, index)
                setDragIndex(null)
                setOverIndex(null)
              }}
              sx={{
                display: 'flex',
                alignItems: 'center',
                gap: 1,
                px: 1,
                py: 0.5,
                cursor: 'grab',
                opacity: dragIndex === index ? 0.4 : 1,
                borderColor: overIndex === index && dragIndex !== index ? 'primary.main' : 'divider',
                bgcolor: setting.enabled ? 'background.paper' : 'action.hover',
              }}
            >
              <Checkbox
                checked={setting.enabled}
                onChange={() => toggle(index)}
                inputProps={{ 'aria-label': `Use ${TIE_BREAKER_LABELS[setting.rule]}` }}
              />
              <DragIndicatorIcon fontSize="small" sx={{ color: 'text.disabled' }} />
              <Typography
                sx={{
                  flexGrow: 1,
                  color: setting.enabled ? 'text.primary' : 'text.disabled',
                  textDecoration: setting.enabled ? 'none' : 'line-through',
                }}
              >
                {index + 1}. {TIE_BREAKER_LABELS[setting.rule]}
              </Typography>
              {/* Keyboard/click alternative to dragging. */}
              <IconButton
                size="small"
                aria-label={`Move ${TIE_BREAKER_LABELS[setting.rule]} up`}
                disabled={index === 0}
                onClick={() => move(index, index - 1)}
              >
                <KeyboardArrowUpIcon fontSize="small" />
              </IconButton>
              <IconButton
                size="small"
                aria-label={`Move ${TIE_BREAKER_LABELS[setting.rule]} down`}
                disabled={index === rules.length - 1}
                onClick={() => move(index, index + 1)}
              >
                <KeyboardArrowDownIcon fontSize="small" />
              </IconButton>
            </Paper>
          ))}
        </Stack>
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel} disabled={saving}>Cancel</Button>
        <Button onClick={save} variant="contained" disableElevation disabled={saving || loading}>
          {saving ? 'Saving…' : 'Save'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
