import { useRef, useState } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button, { type ButtonProps } from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import Divider from '@mui/material/Divider'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import ListAltIcon from '@mui/icons-material/ListAlt'
import UploadFileIcon from '@mui/icons-material/UploadFile'
import { RoundAPI, type ParsedQuestionSetTS, type RoundQuestionTypesTS } from '../features/RoundAPI'
import { useAuth } from '../hooks/useAuth'
import { QuestionTypesEditor, type QuestionTypesValue } from './QuestionTypesEditor'
import { toSlots, toStored, validateSlots } from '../utils/questionTypes'

/** One round in the dialog: where its types came from and what will be saved for it. */
interface RoundReview {
  round: RoundQuestionTypesTS
  /** Index into the imported file's sets, or null when the round isn't using one. */
  setIndex: number | null
  value: QuestionTypesValue
}

/** What the round has saved now. */
const currentValue = (round: RoundQuestionTypesTS): QuestionTypesValue => ({
  enabled: !!round.question_types,
  usingStandard: round.using_standard_question_types,
  slots: toSlots(round.question_types),
})

/** `fallback` with a set from the file applied; a set that couldn't be read changes nothing. */
function applySet(fallback: QuestionTypesValue, set: ParsedQuestionSetTS | undefined): QuestionTypesValue {
  if (!set || set.errors.length > 0 || !set.question_types) return fallback
  return { enabled: true, usingStandard: fallback.usingStandard, slots: toSlots(set.question_types) }
}

/**
 * Sets the question types of every round in a division. The dialog opens on each round's current
 * question types for entering by hand; "Import from file" fills them in from a question set file
 * (.rtf or .csv), matching its sets to the rounds in schedule order (Set #1 → the first round, …).
 * Either way the user reviews them in the same editor the Round dialog uses, then saves every
 * round at once.
 */
export default function SetQuestionTypesButton({ did, onSaved, buttonProps }: {
  did: string
  onSaved?: () => void
  /** Styling for the button, to match the buttons beside it. */
  buttonProps?: ButtonProps
}) {
  const { accessToken } = useAuth()
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [open, setOpen] = useState(false)
  const [fileName, setFileName] = useState('')
  const [sets, setSets] = useState<ParsedQuestionSetTS[]>([])
  const [reviews, setReviews] = useState<RoundReview[]>([])
  const [loading, setLoading] = useState(false)
  const [importing, setImporting] = useState(false)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [roundErrors, setRoundErrors] = useState<Map<string, string[]>>(new Map())

  const openDialog = async () => {
    setOpen(true)
    setFileName('')
    setSets([])
    setReviews([])
    setError(null)
    setRoundErrors(new Map())
    setLoading(true)
    try {
      const division = await RoundAPI.getQuestionTypesByDivision(did)
      setReviews(division.rounds.map(round => ({ round, setIndex: null, value: currentValue(round) })))
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "Failed to load the division's rounds.")
    } finally {
      setLoading(false)
    }
  }

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    // Reset the input so selecting the same file again re-triggers change.
    e.target.value = ''
    if (!file) return

    setError(null)
    setRoundErrors(new Map())

    const lower = file.name.toLowerCase()
    if (!lower.endsWith('.rtf') && !lower.endsWith('.csv')) {
      setError('Only .rtf and .csv question set files can be imported.')
      return
    }

    setImporting(true)
    try {
      const parsed = await RoundAPI.parseQuestionSetFile(did, file.name, await file.text(), accessToken)
      if (parsed.errors.length > 0) {
        setError(parsed.errors.join(' '))
        return
      }
      setFileName(file.name)
      setSets(parsed.sets)
      // Rounds past the last set keep whatever they have, including any edits made by hand.
      setReviews(rs => rs.map((r, i) => (i < parsed.sets.length
        ? { ...r, setIndex: i, value: applySet(r.value, parsed.sets[i]) }
        : { ...r, setIndex: null })))
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Failed to read the file.')
    } finally {
      setImporting(false)
    }
  }

  const updateReview = (index: number, change: Partial<RoundReview>) =>
    setReviews(rs => rs.map((r, i) => (i === index ? { ...r, ...change } : r)))

  // "No set" puts the round back to what it has saved.
  const chooseSet = (index: number, setIndex: number | null) => {
    const review = reviews[index]
    const saved = currentValue(review.round)
    updateReview(index, { setIndex, value: setIndex === null ? saved : applySet(saved, sets[setIndex]) })
  }

  const handleSave = async () => {
    // All-or-nothing: nothing is saved while any round's question types are incomplete or off-count.
    const errors = new Map<string, string[]>()
    for (const r of reviews) {
      if (!r.value.enabled) continue
      const messages = validateSlots(r.value.slots, r.value.usingStandard)
      if (messages.length > 0) errors.set(r.round.roundid, messages)
    }
    setRoundErrors(errors)
    if (errors.size > 0) {
      setError('Some rounds have question type problems (shown below). Nothing was saved.')
      return
    }

    setSaving(true)
    setError(null)
    try {
      await RoundAPI.updateQuestionTypesOfDivision(did, reviews.map(r => ({
        roundid: r.round.roundid,
        question_types: toStored(r.value.enabled, r.value.slots),
        using_standard_question_types: r.value.usingStandard,
      })), accessToken)
      setOpen(false)
      onSaved?.()
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Failed to save question types.')
    } finally {
      setSaving(false)
    }
  }

  const handleClose = () => { if (!saving && !importing) setOpen(false) }

  const unusedSets = sets.length > reviews.length ? sets.length - reviews.length : 0

  return (
    <>
      <Button size="small" startIcon={<ListAltIcon />} {...buttonProps} onClick={openDialog}>
        Set Question Types
      </Button>

      <Dialog open={open} onClose={handleClose} maxWidth="lg" fullWidth scroll="paper">
        <DialogTitle>Set Question Types</DialogTitle>
        <DialogContent dividers>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexWrap: 'wrap', mb: 2 }}>
            <Typography variant="body2" color="text.secondary" sx={{ flex: 1, minWidth: 260 }}>
              Pick each question's type below, or import a question set file (.rtf or .csv) to fill
              them in for you. Its sets are matched to the rounds in schedule order, and you can still
              change any of them before saving. Every round is saved together, so if any round has a
              problem nothing is saved.
            </Typography>
            <Button
              variant="outlined"
              startIcon={importing ? <CircularProgress size={16} /> : <UploadFileIcon />}
              disabled={loading || importing || saving || reviews.length === 0}
              onClick={() => fileInputRef.current?.click()}
            >
              Import from file
            </Button>
            <input ref={fileInputRef} type="file" accept=".rtf,.csv" hidden onChange={handleFileChange} />
          </Box>

          {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
          {fileName && <Alert severity="info" sx={{ mb: 2 }}>Filled in from {fileName}. Review the rounds, then save.</Alert>}

          {loading && (
            <Stack direction="row" spacing={1.5} alignItems="center" sx={{ py: 2 }}>
              <CircularProgress size={20} />
              <Typography variant="body2" color="text.secondary">Loading rounds…</Typography>
            </Stack>
          )}

          {!loading && reviews.length === 0 && !error && (
            <Typography variant="body2" color="text.secondary">This division has no rounds yet. Add rounds first.</Typography>
          )}

          {unusedSets > 0 && (
            <Alert severity="warning" sx={{ mb: 1 }}>
              The file has {sets.length} question sets but the division has {reviews.length} rounds; the
              last {unusedSets} set{unusedSets === 1 ? '' : 's'} won't be used unless you pick {unusedSets === 1 ? 'it' : 'them'} for a round.
            </Alert>
          )}

          {!loading && reviews.map((r, i) => {
            const set = r.setIndex === null ? undefined : sets[r.setIndex]
            const problems = roundErrors.get(r.round.roundid) ?? []
            return (
              <Box key={r.round.roundid} sx={{ py: 2 }}>
                {i > 0 && <Divider sx={{ mb: 2 }} />}
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, flexWrap: 'wrap', mb: 1 }}>
                  <Typography sx={{ fontWeight: 700 }}>Round {r.round.name}</Typography>
                  {/* Choosing which of the file's sets a round uses only makes sense once one is imported. */}
                  {sets.length > 0 && (
                    <Select
                      size="small"
                      value={r.setIndex === null ? '' : String(r.setIndex)}
                      displayEmpty
                      onChange={(e) => chooseSet(i, e.target.value === '' ? null : Number(e.target.value))}
                      sx={{ minWidth: 260 }}
                    >
                      <MenuItem value=""><em>No set — keep current question types</em></MenuItem>
                      {sets.map((s, si) => (
                        <MenuItem key={si} value={String(si)}>{s.name}{s.errors.length > 0 ? ' (unreadable)' : ''}</MenuItem>
                      ))}
                    </Select>
                  )}
                </Box>
                {set && set.errors.length > 0 && (
                  <Alert severity="warning" sx={{ mb: 1 }}>
                    {set.name} couldn't be read, so this round keeps its question types: {set.errors.join(' ')}
                  </Alert>
                )}
                {problems.length > 0 && <Alert severity="error" sx={{ mb: 1 }}>{problems.join(' ')}</Alert>}
                <QuestionTypesEditor value={r.value} onChange={(value) => updateReview(i, { value })} />
              </Box>
            )
          })}
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={saving || importing}>Cancel</Button>
          <Button variant="contained" onClick={handleSave} disabled={loading || importing || saving || reviews.length === 0}>
            {saving ? <CircularProgress size={20} color="inherit" /> : 'Save Question Types'}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}
