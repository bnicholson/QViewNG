// Quiz outs, error outs, foul outs and fouls on the Team and Individual stats tables: a total
// column per item in the row (the breakdown is components/OutsAndFoulsGrid.tsx), and the flat
// columns of the export.
import type { ColumnDef } from "../components/DataTableTemplate";
import type { OutsAndFoulsStatTS } from "../features/StatsGroupAPI";

/** Team rows add team and coach fouls to the quizzer fouls; quizzer rows have only quizzer fouls. */
export type OutsScope = "team" | "individual";

export interface OutsGroup {
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
      header: "Quiz\nOuts",
      total: (s) => s.quiz_outs_without_errors + s.quiz_outs_with_errors,
      exportLabel: "QO Total",
      parts: [
        { label: "No Errors", exportLabel: "QO No Errors", value: (s) => s.quiz_outs_without_errors },
        { label: "With Errors", exportLabel: "QO With Errors", value: (s) => s.quiz_outs_with_errors },
      ],
    },
    { header: "Error\nOuts", total: (s) => s.error_outs, exportLabel: "Error Outs" },
    { header: "Foul\nOuts", total: (s) => s.foul_outs, exportLabel: "Foul Outs" },
    scope === "team"
      ? {
          header: "Fouls",
          total: (s) => s.quizzer_fouls + s.team_fouls + s.coach_fouls,
          exportLabel: "Fouls Total",
          parts: [
            { label: "Quizzer", exportLabel: "Quizzer Fouls", value: (s) => s.quizzer_fouls },
            { label: "Team", exportLabel: "Team Fouls", value: (s) => s.team_fouls },
            { label: "Coach", exportLabel: "Coach Fouls", value: (s) => s.coach_fouls },
          ],
        }
      : { header: "Fouls", total: (s) => s.quizzer_fouls, exportLabel: "Fouls" },
  ];
}

/** One total column per item; an item's breakdown is in the cell's tooltip. */
export function outsAndFoulsColumns<T>(getStats: (row: T) => OutsAndFoulsStatTS, scope: OutsScope): ColumnDef<T>[] {
  return outsGroups(scope).map((g) => ({
    header: g.header,
    align: "center",
    render: (row: T) => {
      const s = getStats(row);
      const title = g.parts?.map((p) => `${p.label}: ${p.value(s)}`).join(" · ");
      return <span title={title}>{g.total(s)}</span>;
    },
  }));
}

/** The flat export columns (each breakdown, then its total) and one row's values for them. */
export function outsAndFoulsExportColumns(scope: OutsScope): string[] {
  return outsGroups(scope).flatMap((g) => [...(g.parts ?? []).map((p) => p.exportLabel), g.exportLabel]);
}

export function outsAndFoulsExportValues(s: OutsAndFoulsStatTS, scope: OutsScope): number[] {
  return outsGroups(scope).flatMap((g) => [...(g.parts ?? []).map((p) => p.value(s)), g.total(s)]);
}
