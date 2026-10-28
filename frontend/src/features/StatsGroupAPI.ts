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

/** The criteria a stats group can apply when two teams are otherwise level. */
export type TieBreakerRule =
  | "team_wl"
  | "memory_verse"
  | "head_to_head"
  | "total_points"
  | "avg_points"
  | "olympic"
  | "mod_olympic";

/** One row of a stats group's tie-breaker configuration, in application order. */
export interface TieBreakerSettingTS {
  rule: TieBreakerRule;
  enabled: boolean;
  /**
   * False when the stats group lacks the data the criterion needs (Memory Verse: question types for
   * every round). It then comes back unchecked and can't be checked. Set by the server on read.
   */
  available?: boolean;
}

/** How the criteria read in the UI. Mirrors `TieBreakerRule::label` on the backend. */
export const TIE_BREAKER_LABELS: Record<TieBreakerRule, string> = {
  team_wl: "Team W-L Record",
  memory_verse: "Memory Verse (M) Correct",
  head_to_head: "Head-to-Head W-L",
  total_points: "Total Points",
  avg_points: "Total Avg Pts",
  olympic: "Olympic Pts",
  mod_olympic: "Mod Olympic Pts",
};

/** Why one team sits above another, for the "Rank Rules" column. */
export interface RankRuleExplanationTS {
  number: number;        // 1-based position of the criterion in the configured sequence
  rule: TieBreakerRule;
  label: string;
  other_team: string;
  detail: string;        // the values that decided it, e.g. "6-2 vs 4-4"
}

/** One question type's counts for a team or quizzer. Mirrors `QuestionTypeStat` on the backend. */
export interface QuestionTypeStatTS {
  code: string;            // G, A, V, Q, R, M, I, S or X; M (Memory Verse) re-counts R, Q and V
  attempted: number;       // tossups answered (correct + erroneous)
  correct: number;
  bonus_attempted: number; // bonuses answered (correct + erroneous)
  bonus_correct: number;
}

/** A team's or quizzer's outs and fouls. Mirrors `OutsAndFoulsStat` on the backend. */
export interface OutsAndFoulsStatTS {
  quiz_outs_without_errors: number; // these earn the quiz-out award
  quiz_outs_with_errors: number;
  error_outs: number;
  foul_outs: number;
  quizzer_fouls: number;
  team_fouls: number;               // always 0 for a quizzer
  coach_fouls: number;              // always 0 for a quizzer
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
  avg_points: number;
  /** Correct tossups (bonuses excluded) on M (Memory Verse: R, Q and V) questions; 0 when that criterion is unavailable. */
  memory_verse_correct: number;
  /** Counts per question type, summed over the team's quizzers. */
  question_types: QuestionTypeStatTS[];
  /** Outs and quizzer fouls summed over the team's quizzers, plus the team's own team and coach fouls. */
  outs_and_fouls: OutsAndFoulsStatTS;
  rank_rule_above: RankRuleExplanationTS | null;
  rank_rule_below: RankRuleExplanationTS | null;
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
  question_types: QuestionTypeStatTS[];
  outs_and_fouls: OutsAndFoulsStatTS;
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
  // The stats group's tie-breaking criteria, in the order they are applied.
  getTieBreakerRules: async (sgid: string, accessToken?: string): Promise<TieBreakerSettingTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/tiebreaker-rules`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load tie-breaking rules (${res.status})`);
    return res.json();
  },
  // Replaces them; the next team-stats read is sorted by the new order.
  saveTieBreakerRules: async (sgid: string, rules: TieBreakerSettingTS[], accessToken?: string): Promise<TieBreakerSettingTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/tiebreaker-rules`, {
      method: "PUT",
      headers: { ...authHeaders(accessToken), "Content-Type": "application/json" },
      body: JSON.stringify({ rules }),
    });
    if (!res.ok) throw new Error(`Failed to save tie-breaking rules (${res.status})`);
    return res.json();
  },
  // Computed individual (per-quizzer) stats for a stats group.
  getIndividualStats: async (sgid: string, accessToken?: string): Promise<IndividualStatTS[]> => {
    const res = await fetch(`/api/statsgroups/${sgid}/individualstats`, { headers: authHeaders(accessToken) });
    if (!res.ok) throw new Error(`Failed to load individual stats (${res.status})`);
    return res.json();
  },
};
