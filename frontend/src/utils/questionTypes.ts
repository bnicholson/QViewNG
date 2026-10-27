// Question types of a round's 20-question question set. Mirrors backend/src/models/question_type.rs,
// which re-checks everything on save.

export const QUESTIONS_PER_ROUND = 20

/** A storable question type code. M (Memory Verse) is inferred from R, V and Q, so it isn't one. */
export type QuestionTypeCode = 'G' | 'A' | 'Q' | 'R' | 'V' | 'X' | 'S' | 'I'

/** Every storable type with the number of its questions in a standard set. S and I share one slot. */
export const QUESTION_TYPES: { code: QuestionTypeCode; name: string; standardCount: number }[] = [
  { code: 'G', name: 'General', standardCount: 11 },
  { code: 'A', name: 'According-to', standardCount: 4 },
  { code: 'Q', name: 'Quote', standardCount: 1 },
  { code: 'R', name: 'Reference', standardCount: 1 },
  { code: 'V', name: 'Verse', standardCount: 1 },
  { code: 'X', name: 'Context', standardCount: 1 },
  { code: 'S', name: 'Situation', standardCount: 1 },
  { code: 'I', name: 'In what Book and Chapter', standardCount: 1 },
]

/** M (Memory Verse) is one each of R, V and Q, so a standard set has 3. */
export const MEMORY_VERSE_STANDARD_COUNT = 3

/** One entry per question; '' means the question has no type picked yet. */
export type QuestionTypeSlots = (QuestionTypeCode | '')[]

export const emptySlots = (): QuestionTypeSlots => Array(QUESTIONS_PER_ROUND).fill('')

/** A round's stored types as one slot per question (all empty when none are stored). */
export function toSlots(stored: string | null | undefined): QuestionTypeSlots {
  const slots = emptySlots()
  if (!stored) return slots
  stored.split('').slice(0, QUESTIONS_PER_ROUND).forEach((c, i) => {
    if (QUESTION_TYPES.some(t => t.code === c)) slots[i] = c as QuestionTypeCode
  })
  return slots
}

export function countTypes(slots: QuestionTypeSlots): Record<QuestionTypeCode | 'M', number> {
  const counts = { G: 0, A: 0, Q: 0, R: 0, V: 0, X: 0, S: 0, I: 0, M: 0 }
  for (const s of slots) if (s) counts[s] += 1
  counts.M = counts.R + counts.V + counts.Q
  return counts
}

/**
 * Why the slots can't be saved, or [] when they can. Storage is all-or-nothing, so every question
 * needs a type. The standard counts, including not having both S and I questions, are checked only
 * when `usingStandard` is on.
 */
export function validateSlots(slots: QuestionTypeSlots, usingStandard: boolean): string[] {
  const errors: string[] = []
  const missing = slots.map((s, i) => (s ? null : i + 1)).filter((n): n is number => n !== null)
  if (missing.length > 0) {
    errors.push(`Every question needs a question type (missing: ${missing.join(', ')}).`)
    return errors
  }
  const counts = countTypes(slots)
  if (usingStandard) {
    if (counts.S > 0 && counts.I > 0) {
      errors.push("A standard question set can't contain both S (Situation) and I (In what Book and Chapter) questions.")
    }
    for (const t of QUESTION_TYPES.filter(t => t.code !== 'S' && t.code !== 'I')) {
      if (counts[t.code] !== t.standardCount) {
        errors.push(`Expected ${t.standardCount} ${t.code} (${t.name}) question(s), found ${counts[t.code]}.`)
      }
    }
    if (counts.S + counts.I !== 1) {
      errors.push(`Expected 1 S (Situation) or I (In what Book and Chapter) question, found ${counts.S + counts.I}.`)
    }
  }
  return errors
}

/** What gets stored: the 20 codes, or '' when the section is off. */
export const toStored = (enabled: boolean, slots: QuestionTypeSlots): string =>
  enabled ? slots.join('') : ''
