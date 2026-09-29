/** Request body for the transactional auto-schedule endpoint (POST /api/schedule). */
export interface CreateSchedulePayload {
  did: string;
  /** One created session per name (the "# of times each team plays" count). */
  session_names: string[];
  /** How many pools to split the division's teams across. */
  num_pools: number;
  /** Minutes between consecutive rounds. */
  round_duration_minutes: number;
  /** ISO 8601 start time of the first round, or null for unscheduled rounds. */
  start_time: string | null;
  /** Ordered room ids to fill each round's concurrent-game slots. */
  room_slots: string[];
  /** When true, re-shuffle teams into new pools for each session's pool group (only affects 2+ sessions). */
  shuffle_teams: boolean;
}

/** Counts of what the endpoint created — everything or nothing (it runs in one transaction). */
export interface CreateScheduleResult {
  sessions_created: number;
  pool_groups_created: number;
  pools_created: number;
  team_placements_created: number;
  rounds_created: number;
  games_created: number;
}

export const ScheduleAPI = {
  /** Create a round-robin schedule (sessions, pool groups, pools, placements, rounds, games) in one
   *  transactional request. On any failure the backend creates nothing. */
  create: async (payload: CreateSchedulePayload, accessToken?: string): Promise<CreateScheduleResult> => {
    const response = await fetch('/api/schedule', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create schedule (${response.status}): ${text}`);
    }
    return response.json();
  },
};
