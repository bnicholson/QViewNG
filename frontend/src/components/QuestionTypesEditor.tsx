import Box from '@mui/material/Box'
import Checkbox from '@mui/material/Checkbox'
import Chip from '@mui/material/Chip'
import FormControlLabel from '@mui/material/FormControlLabel'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Tooltip from '@mui/material/Tooltip'
import Typography from '@mui/material/Typography'
import {
  MEMORY_VERSE_STANDARD_COUNT, QUESTION_TYPES, countTypes,
  type QuestionTypeCode, type QuestionTypeSlots,
} from '../utils/questionTypes'

export interface QuestionTypesValue {
  /**
   * Whether the round has question types. Starts from whether any are stored; saving while off
   * removes them.
   */
  enabled: boolean
  usingStandard: boolean
  slots: QuestionTypeSlots
}

/** green: the standard count, orange: fewer, red: more. */
function countColor(found: number, expected: number): 'success' | 'warning' | 'error' {
  if (found === expected) return 'success'
  return found < expected ? 'warning' : 'error'
}

/**
 * A round's question types: an On/Off switch for the section, the standard-counts switch, a live
 * count of each type, and one dropdown per question. Used by the Round editor and the import review.
 */
export function QuestionTypesEditor({ value, onChange, label = 'Question Types' }: {
  value: QuestionTypesValue
  onChange: (value: QuestionTypesValue) => void
  label?: string
}) {
  const { enabled, usingStandard, slots } = value
  const counts = countTypes(slots)

  const setSlot = (index: number, code: QuestionTypeCode | '') =>
    onChange({ ...value, slots: slots.map((s, i) => (i === index ? code : s)) })

  // S and I share the standard set's one slot, so they're counted together.
  const counters: { key: string; label: string; title: string; found: number; expected: number }[] = [
    ...QUESTION_TYPES.filter(t => t.code !== 'S' && t.code !== 'I').map(t => ({
      key: t.code, label: t.code, title: t.name, found: counts[t.code], expected: t.standardCount,
    })),
    { key: 'SI', label: 'S/I', title: 'Situation or In what Book and Chapter (one or the other)', found: counts.S + counts.I, expected: 1 },
    { key: 'M', label: 'M', title: 'Memory Verse: inferred from one each of R, V and Q', found: counts.M, expected: MEMORY_VERSE_STANDARD_COUNT },
  ]

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexWrap: 'wrap' }}>
        <FormControlLabel
          control={<Checkbox checked={enabled} onChange={(e) => onChange({ ...value, enabled: e.target.checked })} />}
          label={<Typography sx={{ fontWeight: 600 }}>{label}</Typography>}
        />
        <Tooltip title="When on, saving checks that the round has the standard count of each question type. Turn it off to save any mix, e.g. only G questions.">
          <FormControlLabel
            disabled={!enabled}
            control={<Checkbox size="small" checked={usingStandard} onChange={(e) => onChange({ ...value, usingStandard: e.target.checked })} />}
            label={<Typography variant="body2">Use standard question type counts</Typography>}
          />
        </Tooltip>
      </Box>

      <Box sx={{ opacity: enabled ? 1 : 0.45, pointerEvents: enabled ? 'auto' : 'none' }}>
        <Box sx={{ display: 'flex', gap: 0.75, flexWrap: 'wrap', my: 1 }}>
          {counters.map(c => (
            <Tooltip key={c.key} title={c.title}>
              <Chip
                size="small"
                variant="outlined"
                color={usingStandard ? countColor(c.found, c.expected) : 'default'}
                sx={c.key === 'M' ? { fontStyle: 'italic' } : undefined}
                label={usingStandard ? `${c.label}: ${c.found}/${c.expected}` : `${c.label}: ${c.found}`}
              />
            </Tooltip>
          ))}
        </Box>

        <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(120px, 1fr))', gap: 1 }}>
          {slots.map((slot, i) => (
            <Box key={i} sx={{ display: 'flex', alignItems: 'center', gap: 0.75 }}>
              <Typography variant="body2" sx={{ width: 26, textAlign: 'right', color: 'text.secondary' }}>{i + 1}.</Typography>
              <Select
                size="small"
                fullWidth
                displayEmpty
                disabled={!enabled}
                value={slot}
                error={enabled && !slot}
                onChange={(e) => setSlot(i, e.target.value as QuestionTypeCode | '')}
                renderValue={(v) => v || <em style={{ color: '#9ca3af' }}>—</em>}
                inputProps={{ 'aria-label': `Question ${i + 1} type` }}
              >
                <MenuItem value=""><em>None</em></MenuItem>
                {QUESTION_TYPES.map(t => (
                  <MenuItem key={t.code} value={t.code}>{t.code} — {t.name}</MenuItem>
                ))}
              </Select>
            </Box>
          ))}
        </Box>
      </Box>
    </Box>
  )
}
