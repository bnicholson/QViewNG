import type { GameTS } from "./GameAPI";

export interface StatsGroupTS {
  sgid: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
  tournament_id: string;
  division_id: string | null;
}

export const StatsGroupAPI = {
  getByTournament: async (tid: string, page: number, size: number): Promise<StatsGroupTS[]> =>
    (await fetch(`/api/tournaments/${tid}/statsgroups?page=${page}&page_size=${size}`)).json(),
  // Games that belong to a stats group (via games_statsgroups).
  getGames: async (sgid: string, page: number, size: number): Promise<GameTS[]> =>
    (await fetch(`/api/statsgroups/${sgid}/games?page=${page}&page_size=${size}`)).json(),
};
