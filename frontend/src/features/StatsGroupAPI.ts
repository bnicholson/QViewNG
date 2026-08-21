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

export interface IndividualStatTS {
  place: number;
  individual: string;
  team_name: string;
  games: number;
  score: number;
  avg: number;
  correct: number;
  errors: number;
  bonus_pts: number;
  bonus_attempts: number;
}

// These endpoints are restricted server-side to super users, the tournament owner, and its
// admins, so each call forwards the access token as a Bearer Authorization header.
const authHeaders = (accessToken?: string): HeadersInit =>
  accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {};

export const StatsGroupAPI = {
  getByTournament: async (tid: string, page: number, size: number, accessToken?: string): Promise<StatsGroupTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/statsgroups?page=${page}&page_size=${size}`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load stats groups (${res.status})`);
    return res.json();
  },
  // Games that belong to a stats group (via games_statsgroups).
  getGames: async (sgid: string, page: number, size: number, accessToken?: string): Promise<GameTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/games?page=${page}&page_size=${size}`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load stats group games (${res.status})`);
    return res.json();
  },
  // Computed team standings for a stats group.
  getTeamStats: async (sgid: string, accessToken?: string): Promise<TeamStatTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/teamstats`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load team stats (${res.status})`);
    return res.json();
  },
  // Computed individual (per-quizzer) stats for a stats group.
  getIndividualStats: async (sgid: string, accessToken?: string): Promise<IndividualStatTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/individualstats`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load individual stats (${res.status})`);
    return res.json();
  },
};
