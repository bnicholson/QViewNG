// Per-question-type counts on the Team and Individual stats tables: a compact "correct/attempted"
// column per type in the row (the full grid is components/QuestionTypeGrid.tsx), and the flat
// columns of the export.
import type { ColumnDef } from "../components/DataTableTemplate";
import type { QuestionTypeStatTS } from "../features/StatsGroupAPI";
import { QUESTION_TYPES } from "./questionTypes";

/** Display order of the types. Mirrors `STATS_QUESTION_TYPES` on the backend. */
export const STATS_TYPE_CODES = ["G", "A", "M", "R", "Q", "V", "X", "I", "S"] as const;

export const TYPE_NAMES: Record<string, string> = {
  ...Object.fromEntries(QUESTION_TYPES.map((t) => [t.code, t.name])),
  M: "Memory Verse (R, Q and V)",
};

export const COUNTS: { key: keyof Omit<QuestionTypeStatTS, "code">; label: string }[] = [
  { key: "attempted", label: "Attempted" },
  { key: "correct", label: "Correct" },
  { key: "bonus_attempted", label: "Bonuses Attempted" },
  { key: "bonus_correct", label: "Bonuses Correct" },
];

export const MUTED = "#9ca3af";

export function statFor(stats: QuestionTypeStatTS[], code: string): QuestionTypeStatTS {
  return stats.find((s) => s.code === code) ?? { code, attempted: 0, correct: 0, bonus_attempted: 0, bonus_correct: 0 };
}

/** One narrow "correct/attempted" column per type; the bonus counts are in the cell's tooltip. */
export function questionTypeColumns<T>(getStats: (row: T) => QuestionTypeStatTS[]): ColumnDef<T>[] {
  return STATS_TYPE_CODES.map((code) => ({
    header: code,
    headerTitle: TYPE_NAMES[code],
    align: "center",
    render: (row: T) => {
      const s = statFor(getStats(row), code);
      if (s.attempted === 0 && s.bonus_attempted === 0) return <span style={{ color: MUTED }}>–</span>;
      return (
        <span
          title={`${TYPE_NAMES[code]}: ${s.correct} of ${s.attempted} correct · Bonuses: ${s.bonus_correct} of ${s.bonus_attempted} correct`}
          style={{ whiteSpace: "nowrap" }}
        >
          {s.correct}/{s.attempted}
        </span>
      );
    },
  }));
}

/** The flat export columns ("G Attempted", "G Correct", …) and one row's values for them. */
export const questionTypeExportColumns: string[] = STATS_TYPE_CODES.flatMap((code) =>
  COUNTS.map((c) => `${code} ${c.label}`),
);

export function questionTypeExportValues(stats: QuestionTypeStatTS[]): number[] {
  return STATS_TYPE_CODES.flatMap((code) => {
    const s = statFor(stats, code);
    return COUNTS.map((c) => s[c.key]);
  });
}
