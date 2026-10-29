// Quiz outs, error outs, error zones, fouls and foul outs on the Team and Individual stats
// tables: a total column per item in the row (the breakdown is components/OutsAndFoulsGrid.tsx),
// and the flat columns of the export.
import type { ColumnDef } from "../components/DataTableTemplate";
import type { OutsAndFoulsStatTS } from "../features/StatsGroupAPI";

/** Team rows add team and coach fouls to the quizzer fouls; quizzer rows have only quizzer fouls. */
export type OutsScope = "team" | "individual";

export type OutsKey = "quizOuts" | "errorOuts" | "errorZoneTeam" | "errorZoneLate" | "fouls" | "foulOuts";

export interface OutsGroup {
  key: OutsKey;
  /** Header label; a "\n" starts a new line. */
  header: string;
  total: (s: OutsAndFoulsStatTS) => number;
  /** Export column name of the total. */
  exportLabel: string;
  /** The counts the total adds up, when it has a breakdown. */
  parts?: { label: string; exportLabel: string; value: (s: OutsAndFoulsStatTS) => number }[];
}

/** The items in display order, each with its breakdown where it has one. */
export function outsGroups(scope: OutsScope): OutsGroup[] {
  return [
    {
      key: "quizOuts",
      header: "Quiz\nOuts",
      total: (s) => s.quiz_outs_without_errors + s.quiz_outs_with_errors,
      exportLabel: "QO Total",
      parts: [
        { label: "No Errors", exportLabel: "QO No Errors", value: (s) => s.quiz_outs_without_errors },
        { label: "With Errors", exportLabel: "QO With Errors", value: (s) => s.quiz_outs_with_errors },
      ],
    },
    { key: "errorOuts", header: "Error\nOuts", total: (s) => s.error_outs, exportLabel: "Error Outs" },
    { key: "errorZoneTeam", header: "Err\nZone\n5-15", total: (s) => s.error_zone_team, exportLabel: "Err Zone 5-15" },
    { key: "errorZoneLate", header: "Err\nZone\n16-20+", total: (s) => s.error_zone_late, exportLabel: "Err Zone 16-20+" },
    scope === "team"
      ? {
          key: "fouls",
          header: "Fouls",
          total: (s) => s.quizzer_fouls + s.team_fouls + s.coach_fouls,
          exportLabel: "Fouls Total",
          parts: [
            { label: "Quizzer", exportLabel: "Quizzer Fouls", value: (s) => s.quizzer_fouls },
            { label: "Team", exportLabel: "Team Fouls", value: (s) => s.team_fouls },
            { label: "Coach", exportLabel: "Coach Fouls", value: (s) => s.coach_fouls },
          ],
        }
      : { key: "fouls", header: "Fouls", total: (s) => s.quizzer_fouls, exportLabel: "Fouls" },
    { key: "foulOuts", header: "Foul\nOuts", total: (s) => s.foul_outs, exportLabel: "Foul Outs" },
  ];
}

/** One total column per item, keyed so each table can place them; an item's breakdown is in the cell's tooltip. */
export function outsAndFoulsColumns<T>(getStats: (row: T) => OutsAndFoulsStatTS, scope: OutsScope): Record<OutsKey, ColumnDef<T>> {
  const columns = {} as Record<OutsKey, ColumnDef<T>>;
  for (const g of outsGroups(scope)) {
    columns[g.key] = {
      header: g.header,
      align: "center",
      render: (row: T) => {
        const s = getStats(row);
        const title = g.parts?.map((p) => `${p.label}: ${p.value(s)}`).join(" · ");
        return <span title={title}>{g.total(s)}</span>;
      },
    };
  }
  return columns;
}

/** An item's total as its table column shows it, for the export's copy of the table: its header on one line, and its value. */
export function outsTotalExport(scope: OutsScope, key: OutsKey): { label: string; value: (s: OutsAndFoulsStatTS) => number } {
  const g = outsGroups(scope).find((g) => g.key === key)!;
  return { label: g.header.replace(/\n/g, " "), value: g.total };
}

/** The flat export columns of the breakdown (each item's parts, then its total) and one row's values for them. */
export function outsAndFoulsExportColumns(scope: OutsScope): string[] {
  return outsGroups(scope).flatMap((g) => [...(g.parts ?? []).map((p) => p.exportLabel), g.exportLabel]);
}

export function outsAndFoulsExportValues(s: OutsAndFoulsStatTS, scope: OutsScope): number[] {
  return outsGroups(scope).flatMap((g) => [...(g.parts ?? []).map((p) => p.value(s)), g.total(s)]);
}
