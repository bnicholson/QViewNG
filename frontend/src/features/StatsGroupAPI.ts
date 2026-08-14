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

export interface TeamStatTS {
  place: number;
  name: string;
  games: number;
  wins: number;
  losses: number;
  olympic_points: number;
  mod_olympic_points: number;
  total_points: number;
  tie_breaker: string;
}

export const StatsGroupAPI = {
  getByTournament: async (tid: string, page: number, size: number): Promise<StatsGroupTS[]> =>
    (await fetch(`/api/tournaments/${tid}/statsgroups?page=${page}&page_size=${size}`)).json(),
  // Games that belong to a stats group (via games_statsgroups).
  getGames: async (sgid: string, page: number, size: number): Promise<GameTS[]> =>
    (await fetch(`/api/statsgroups/${sgid}/games?page=${page}&page_size=${size}`)).json(),
  // Computed team standings for a stats group.
  getTeamStats: async (sgid: string): Promise<TeamStatTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/teamstats`);
    if (!res.ok) throw new Error(`Failed to load team stats (${res.status})`);
    return res.json();
  },
};
