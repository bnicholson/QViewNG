import { useRef, useState } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
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
import UploadFileIcon from '@mui/icons-material/UploadFile'
import { RoundAPI, type ParsedQuestionSetTS, type RoundQuestionTypesTS } from '../features/RoundAPI'
import { useAuth } from '../hooks/useAuth'
import { QuestionTypesEditor, type QuestionTypesValue } from './QuestionTypesEditor'
import { toSlots, toStored, validateSlots } from '../utils/questionTypes'

/** One round in the review: where its types came from and what will be saved for it. */
interface RoundReview {
  round: RoundQuestionTypesTS
  /** Index into the file's sets, or null to keep the round's current question types. */
  setIndex: number | null
  value: QuestionTypesValue
}

const currentValue = (round: RoundQuestionTypesTS): QuestionTypesValue => ({
  enabled: !!round.question_types,
  usingStandard: round.using_standard_question_types,
  slots: toSlots(round.question_types),
})

/** The round's state with a set from the file applied; a set that couldn't be read changes nothing. */
function applySet(round: RoundQuestionTypesTS, set: ParsedQuestionSetTS | undefined): QuestionTypesValue {
  if (!set || set.errors.length > 0 || !set.question_types) return currentValue(round)
  return { enabled: true, usingStandard: round.using_standard_question_types, slots: toSlots(set.question_types) }
}

/**
 * Imports a division's round question types from a question set file (.rtf or .csv). The file's
 * sets go to the division's rounds in schedule order (Set #1 → the first round, …); the user reviews
 * and adjusts them in the same editor the Round dialog uses, then saves every round at once.
 */
export default function ImportQuestionTypesButton({ did, onImported }: { did: string; onImported?: () => void }) {
  const { accessToken } = useAuth()
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [open, setOpen] = useState(false)
  const [fileName, setFileName] = useState('')
  const [sets, setSets] = useState<ParsedQuestionSetTS[]>([])
  const [reviews, setReviews] = useState<RoundReview[]>([])
  const [loading, setLoading] = useState(false)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [roundErrors, setRoundErrors] = useState<Map<string, string[]>>(new Map())

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    // Reset the input so selecting the same file again re-triggers change.
    e.target.value = ''
    if (!file) return

    setOpen(true)
    setFileName(file.name)
    setSets([])
    setReviews([])
    setError(null)
    setRoundErrors(new Map())

    const lower = file.name.toLowerCase()
    if (!lower.endsWith('.rtf') && !lower.endsWith('.csv')) {
      setError('Only .rtf and .csv question set files can be imported.')
      return
    }

    setLoading(true)
    try {
      const [content, division] = await Promise.all([file.text(), RoundAPI.getQuestionTypesByDivision(did)])
      const parsed = await RoundAPI.parseQuestionSetFile(did, file.name, content, accessToken)
      if (parsed.errors.length > 0) setError(parsed.errors.join(' '))
      setSets(parsed.sets)
      setReviews(division.rounds.map((round, i) => {
        const setIndex = i < parsed.sets.length ? i : null
        return { round, setIndex, value: applySet(round, setIndex === null ? undefined : parsed.sets[setIndex]) }
      }))
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Failed to read the file.')
    } finally {
      setLoading(false)
    }
  }

  const updateReview = (index: number, change: Partial<RoundReview>) =>
    setReviews(rs => rs.map((r, i) => (i === index ? { ...r, ...change } : r)))

  const chooseSet = (index: number, setIndex: number | null) => {
    const review = reviews[index]
    updateReview(index, { setIndex, value: applySet(review.round, setIndex === null ? undefined : sets[setIndex]) })
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
      onImported?.()
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Failed to save question types.')
    } finally {
      setSaving(false)
    }
  }

  const handleClose = () => { if (!saving) setOpen(false) }

  const unusedSets = sets.length > reviews.length ? sets.length - reviews.length : 0

  return (
    <>
      <Button size="small" startIcon={<UploadFileIcon />} onClick={() => fileInputRef.current?.click()}>
        Import Question Types
      </Button>
      <input ref={fileInputRef} type="file" accept=".rtf,.csv" hidden onChange={handleFileChange} />

      <Dialog open={open} onClose={handleClose} maxWidth="lg" fullWidth scroll="paper">
        <DialogTitle>Import Question Types{fileName ? ` — ${fileName}` : ''}</DialogTitle>
        <DialogContent dividers>
          {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}

          {loading && (
            <Stack direction="row" spacing={1.5} alignItems="center" sx={{ py: 2 }}>
              <CircularProgress size={20} />
              <Typography variant="body2" color="text.secondary">Reading file…</Typography>
            </Stack>
          )}

          {!loading && reviews.length === 0 && sets.length > 0 && (
            <Typography variant="body2" color="text.secondary">This division has no rounds yet. Add rounds first, then import.</Typography>
          )}

          {!loading && reviews.length > 0 && (
            <>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
                The file's question sets are matched to the rounds in schedule order. Check each round, change
                which set it uses or adjust individual questions, then save. Every round is saved together, so
                if any round has a problem nothing is saved.
              </Typography>
              {unusedSets > 0 && (
                <Alert severity="warning" sx={{ mb: 1 }}>
                  The file has {sets.length} question sets but the division has {reviews.length} rounds; the
                  last {unusedSets} set{unusedSets === 1 ? '' : 's'} won't be used unless you pick {unusedSets === 1 ? 'it' : 'them'} for a round.
                </Alert>
              )}

              {reviews.map((r, i) => {
                const set = r.setIndex === null ? undefined : sets[r.setIndex]
                const problems = roundErrors.get(r.round.roundid) ?? []
                return (
                  <Box key={r.round.roundid} sx={{ py: 2 }}>
                    {i > 0 && <Divider sx={{ mb: 2 }} />}
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, flexWrap: 'wrap', mb: 1 }}>
                      <Typography sx={{ fontWeight: 700 }}>Round {r.round.name}</Typography>
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
                    </Box>
                    {set && set.errors.length > 0 && (
                      <Alert severity="warning" sx={{ mb: 1 }}>
                        {set.name} couldn't be read, so this round keeps its current question types: {set.errors.join(' ')}
                      </Alert>
                    )}
                    {problems.length > 0 && <Alert severity="error" sx={{ mb: 1 }}>{problems.join(' ')}</Alert>}
                    <QuestionTypesEditor value={r.value} onChange={(value) => updateReview(i, { value })} />
                  </Box>
                )
              })}
            </>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={saving}>Cancel</Button>
          <Button variant="contained" onClick={handleSave} disabled={loading || saving || reviews.length === 0}>
            {saving ? <CircularProgress size={20} color="inherit" /> : 'Save Question Types'}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}
