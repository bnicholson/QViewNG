export interface UserTS {
  id: string;
  username: string;
  email: string;
  fname: string;
  mname: string;
  lname: string;
  activated: boolean;
  created_at: string;
  updated_at: string;
}

export interface NewUserPayload {
  fname: string;
  mname: string;
  lname: string;
  username: string;
  email: string;
  hash_password: string;
  activated: boolean;
}

export interface UserChangeset {
  fname?: string;
  mname?: string;
  lname?: string;
  username?: string;
  email?: string;
  activated?: boolean;
}

export interface PagedUsers {
  count: number;
  items: UserTS[];
}

export interface TeamWithTournamentInfoTS {
  teamid: string;
  name: string;
  coachid: string;
  coach_name: string;
  did: string;
  tournament_id: string;
  tournament_name: string;
  tournament_fromdate: string;
  tournament_todate: string;
  created_at: string;
  updated_at: string;
}

export interface GameWithNamesTS {
  gid: string;
  org: string;
  tournamentid: string;
  tournament_name: string;
  tournament_fromdate: string;
  tournament_todate: string;
  divisionid: string;
  roomid: string;
  roundid: string;
  ignore: boolean;
  ruleset: string;
  leftteamid: string;
  left_team_name: string;
  centerteamid: string | null;
  center_team_name: string | null;
  rightteamid: string;
  right_team_name: string;
  quizmasterid: string;
  contentjudgeid: string | null;
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}

export const UserAPI = {
  get: async (page: number, size: number): Promise<PagedUsers> =>
    (await fetch(`/api/users?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string): Promise<PagedUsers> =>
    (await fetch(`/api/tournaments/${tid}/quizzers`)).json(),
  getById: async (id: string): Promise<UserTS> => {
    const response = await fetch(`/api/users/${id}`);
    if (!response.ok) throw new Error(`User not found (${response.status})`);
    return response.json();
  },
  create: async (user: NewUserPayload): Promise<UserTS> => {
    const response = await fetch('/api/users', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(user),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create user (${response.status}): ${text}`);
    }
    return response.json();
  },
  update: async (id: string, changeset: UserChangeset): Promise<UserTS> => {
    const response = await fetch(`/api/users/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update user (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/users/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete user (${response.status}): ${text}`);
    }
  },
  getTeamsAsQuizzer: async (userId: string, page: number, size: number): Promise<TeamWithTournamentInfoTS[]> =>
    (await fetch(`/api/users/${userId}/teams-where-quizzer-enriched?page=${page}&page_size=${size}`)).json(),
  getTeamsAsCoach: async (userId: string, page: number, size: number): Promise<TeamWithTournamentInfoTS[]> =>
    (await fetch(`/api/users/${userId}/teams-where-coach-enriched?page=${page}&page_size=${size}`)).json(),
  getTournamentsAsAdmin: async (userId: string, page: number, size: number): Promise<TournamentForUserTS[]> =>
    (await fetch(`/api/users/${userId}/tournaments-as-admin-or-owner?page=${page}&page_size=${size}`)).json(),
  getGamesAsQuizmaster: async (userId: string, page: number, size: number): Promise<GameWithNamesTS[]> =>
    (await fetch(`/api/users/${userId}/games-where-quizmaster-enriched?page=${page}&page_size=${size}`)).json(),
  getGamesAsContentJudge: async (userId: string, page: number, size: number): Promise<GameWithNamesTS[]> =>
    (await fetch(`/api/users/${userId}/games-where-contentjudge-enriched?page=${page}&page_size=${size}`)).json(),

  // ── Single-call, paginated, audit-enriched rows for the User-profile data tables ──────────────
  getTeamRows: async (userId: string, page: number, size: number): Promise<PagedResponse<UserTeamRowTS>> =>
    (await fetch(`/api/users/${userId}/team-rows?page=${page}&page_size=${size}`)).json(),
  getManagedTournamentRows: async (userId: string, page: number, size: number): Promise<PagedResponse<UserManagedTournamentRowTS>> =>
    (await fetch(`/api/users/${userId}/managed-tournament-rows?page=${page}&page_size=${size}`)).json(),
  getManagedTournamentGroupRows: async (userId: string, page: number, size: number): Promise<PagedResponse<UserManagedTournamentGroupRowTS>> =>
    (await fetch(`/api/users/${userId}/managed-tournamentgroup-rows?page=${page}&page_size=${size}`)).json(),
  getRosterQuizzerRows: async (userId: string, page: number, size: number): Promise<PagedResponse<UserRosterQuizzerRowTS>> =>
    (await fetch(`/api/users/${userId}/roster-quizzer-rows?page=${page}&page_size=${size}`)).json(),
  getGearRows: async (userId: string, page: number, size: number): Promise<PagedResponse<UserGearRowTS>> =>
    (await fetch(`/api/users/${userId}/gear-rows?page=${page}&page_size=${size}`)).json(),
}

export interface PagedResponse<T> { count: number; items: T[]; }

export interface TeamQuizzerRefTS { id: string; name: string; }

/** A team the user participates in (as coach or quizzer), enriched for a single-call table. */
export interface UserTeamRowTS {
  teamid: string;
  name: string;
  did: string;
  division_name: string;
  tournament_id: string;
  tournament_name: string;
  tournament_fromdate: string;
  tournament_todate: string;
  coachid: string;
  coach_name: string;
  role: string;
  quizzers: TeamQuizzerRefTS[];
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}

/** A tournament the user owns, enriched for a single-call table. */
export interface UserManagedTournamentRowTS {
  tid: string;
  tname: string;
  venue: string;
  city: string;
  state: string;
  country: string;
  fromdate: string;
  todate: string;
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}

/** A tournament group the user owns, enriched for a single-call table. */
export interface UserManagedTournamentGroupRowTS {
  tgid: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}

/** A distinct quizzer across the user's rosters (no audit columns — not an audited entity). */
export interface UserRosterQuizzerRowTS {
  quizzer_id: string;
  fname: string;
  mname: string;
  lname: string;
  email: string;
}

/** A gear item across the user's equipment sets, enriched for a single-call table. */
export interface UserGearRowTS {
  id: number;
  gear_type: string;
  equipmentsetid: number;
  set_name: string;
  misc_note: string | null;
  /** Same tagged shape as EquipmentDetail (e.g. `{ Computer: {...} }`); null if the type row is missing. */
  detail: import('./EquipmentSetAPI').EquipmentDetail | null;
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}

export interface TournamentForUserTS {
  tid: string;
  tname: string;
  organization: string;
  fromdate: string;
  todate: string;
  venue: string;
  city: string;
  state: string;
  country: string;
  owner_id: string;
  created_at: string;
  updated_at: string;
  creator_id: string;
  creator_name: string;
  last_modified_user_id: string;
  last_modified_user_name: string;
}
