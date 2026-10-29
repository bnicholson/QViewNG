// The expanded panel of a Team or Individual stats row: every count for every question type.
import Tooltip from "@mui/material/Tooltip";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import type { QuestionTypeStatTS } from "../features/StatsGroupAPI";
import { COUNTS, STATS_TYPE_CODES, TYPE_NAMES, questionTypeTotal, statFor } from "../utils/questionTypeStats";

const NOTE =
  "M (Memory Verse) re-counts the R, Q and V questions, so Total leaves it out. Questions without a question type (overtime, or rounds with none set) aren't counted.";

/** The expanded panel: counts down the side, types across the top, plus a Total column. */
export default function QuestionTypeGrid({ stats }: { stats: QuestionTypeStatTS[] }) {
  const cell = { padding: "3px 10px", textAlign: "center" as const, fontVariantNumeric: "tabular-nums" };
  const head = { ...cell, fontSize: 12, fontWeight: 600, color: "#6b7280" };
  const sep = { borderLeft: "1px solid #e5e7eb" };

  return (
    <table style={{ borderCollapse: "collapse", fontSize: 13, background: "#fff", border: "1px solid #e5e7eb", borderRadius: 6 }}>
      <thead>
        <tr style={{ borderBottom: "1px solid #e5e7eb" }}>
          <th style={{ ...head, textAlign: "left", whiteSpace: "nowrap" }}>
            Question Types
            <Tooltip title={NOTE}>
              <InfoOutlinedIcon aria-label={NOTE} sx={{ fontSize: 15, verticalAlign: "-3px", ml: 0.5, color: "#9ca3af", cursor: "help" }} />
            </Tooltip>
          </th>
          {STATS_TYPE_CODES.map((code) => (
            <th key={code} title={TYPE_NAMES[code]} style={{ ...head, ...(code === "M" ? { fontStyle: "italic" } : {}) }}>
              {code}
            </th>
          ))}
          <th style={{ ...head, ...sep }}>Total</th>
        </tr>
      </thead>
      <tbody>
        {COUNTS.map((c) => (
          <tr key={c.key}>
            <th style={{ ...head, textAlign: "left", whiteSpace: "nowrap" }}>{c.label}</th>
            {STATS_TYPE_CODES.map((code) => (
              <td key={code} style={{ ...cell, ...(code === "M" ? { color: "#6b7280", fontStyle: "italic" } : {}) }}>
                {statFor(stats, code)[c.key]}
              </td>
            ))}
            <td style={{ ...cell, ...sep, fontWeight: 600 }}>{questionTypeTotal(stats, c.key)}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
