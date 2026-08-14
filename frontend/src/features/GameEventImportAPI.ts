// CSV import of GameEvents for an existing tournament's games.

export interface ImportableGame {
  gid: string;
  division: string;
  room: string;
  round: string;
  event_count: number;
}

export interface GameImportError {
  division: string;
  room: string;
  round: string;
  message: string;
}

export interface GameRef {
  division: string;
  room: string;
  round: string;
}

export interface ImportPreview {
  importable: ImportableGame[];
  errors: GameImportError[];
  games_not_found: GameRef[];
  missing_games: GameRef[];
}

async function post(url: string, csv: string): Promise<ImportPreview> {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ csv }),
  });
  if (!res.ok) throw new Error(`Request failed (${res.status})`);
  return res.json();
}

export const GameEventImportAPI = {
  // Dry run — returns the plan and per-game errors; writes nothing.
  preview: (tid: string, csv: string): Promise<ImportPreview> =>
    post(`/api/tournaments/${tid}/gameevents/import/preview`, csv),
  // Commit — inserts events for importable games in one transaction.
  commit: (tid: string, csv: string): Promise<ImportPreview> =>
    post(`/api/tournaments/${tid}/gameevents/import/commit`, csv),
};
