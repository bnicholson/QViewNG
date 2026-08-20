import type { GameTS } from '../features/GameAPI';

// Compares two scheduled-start-time ISO strings, sorting nulls/blanks last.
export function compareStartTime(a: string | null | undefined, b: string | null | undefined): number {
  if (!a && !b) return 0;
  if (!a) return 1;
  if (!b) return -1;
  return new Date(a).getTime() - new Date(b).getTime();
}

// Assigns each game a 1-based "round" ordinal: the Nth game its room plays,
// ordered by the game's scheduled start time (from its round). Ties fall back
// to gid for a stable order. Returns a map of game id -> ordinal.
export function computeRoomRoundSequence(
  games: GameTS[],
  roundStartById: Map<string, string | null>,
): Map<string, number> {
  const startOf = (g: GameTS): string | null => roundStartById.get(g.roundid) ?? null;

  const byRoom = new Map<string, GameTS[]>();
  for (const g of games) {
    const arr = byRoom.get(g.roomid);
    if (arr) arr.push(g);
    else byRoom.set(g.roomid, [g]);
  }

  const sequence = new Map<string, number>();
  for (const roomGames of byRoom.values()) {
    roomGames.sort((a, b) => compareStartTime(startOf(a), startOf(b)) || a.gid.localeCompare(b.gid));
    roomGames.forEach((g, i) => sequence.set(g.gid, i + 1));
  }
  return sequence;
}
