// The outs-and-fouls half of a Team or Individual stats row's expanded panel: each item's
// breakdown under its heading, then its total.
import Tooltip from "@mui/material/Tooltip";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import type { OutsAndFoulsStatTS } from "../features/StatsGroupAPI";
import { outsGroups, type OutsScope } from "../utils/outsAndFoulsStats";

const NOTES: Record<OutsScope, string> = {
  team:
    "A quiz out with no errors (no erroneous tossups in that game) earns the quiz-out award. Outs and quizzer fouls are summed over the team's quizzers; Fouls also counts team and coach fouls.",
  individual: "A quiz out with no errors (no erroneous tossups in that game) earns the quiz-out award.",
};

/** One row of counts under grouped headers: an item with a breakdown spans its parts and a Total. */
export default function OutsAndFoulsGrid({ stats, scope }: { stats: OutsAndFoulsStatTS; scope: OutsScope }) {
  const groups = outsGroups(scope);
  const cell = { padding: "3px 10px", textAlign: "center" as const, fontVariantNumeric: "tabular-nums" };
  const head = { ...cell, fontSize: 12, fontWeight: 600, color: "#6b7280" };
  const sep = { borderLeft: "1px solid #e5e7eb" };
  const total = { ...cell, ...sep, fontWeight: 600 };

  return (
    <table style={{ borderCollapse: "collapse", fontSize: 13, background: "#fff", border: "1px solid #e5e7eb", borderRadius: 6 }}>
      <thead>
        <tr>
          <th rowSpan={2} style={{ ...head, textAlign: "left", whiteSpace: "nowrap", borderBottom: "1px solid #e5e7eb" }}>
            Outs &amp; Fouls
            <Tooltip title={NOTES[scope]}>
              <InfoOutlinedIcon aria-label={NOTES[scope]} sx={{ fontSize: 15, verticalAlign: "-3px", ml: 0.5, color: "#9ca3af", cursor: "help" }} />
            </Tooltip>
          </th>
          {groups.map((g) => (
            <th
              key={g.header}
              colSpan={g.parts ? g.parts.length + 1 : 1}
              rowSpan={g.parts ? 1 : 2}
              style={{ ...head, ...sep, whiteSpace: g.parts ? "nowrap" : "pre", borderBottom: "1px solid #e5e7eb" }}
            >
              {g.parts ? g.header.replace("\n", " ") : g.header}
            </th>
          ))}
        </tr>
        <tr style={{ borderBottom: "1px solid #e5e7eb" }}>
          {groups.flatMap((g) =>
            g.parts
              ? [
                  ...g.parts.map((p, i) => (
                    <th key={`${g.header}-${p.label}`} style={{ ...head, ...(i === 0 ? sep : {}), whiteSpace: "nowrap" }}>
                      {p.label}
                    </th>
                  )),
                  <th key={`${g.header}-total`} style={{ ...head, ...sep }}>Total</th>,
                ]
              : [],
          )}
        </tr>
      </thead>
      <tbody>
        <tr>
          <td />
          {groups.flatMap((g) => [
            ...(g.parts ?? []).map((p, i) => (
              <td key={`${g.header}-${p.label}`} style={{ ...cell, ...(i === 0 ? sep : {}) }}>
                {p.value(stats)}
              </td>
            )),
            <td key={`${g.header}-total`} style={total}>{g.total(stats)}</td>,
          ])}
        </tr>
      </tbody>
    </table>
  );
}
