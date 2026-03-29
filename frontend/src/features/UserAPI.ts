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
}

export interface TournamentForUserTS {
  tid: string;
  tname: string;
  organization: string;
  fromdate: string;
  todate: string;
  venue: string;
  city: string;
  region: string;
  country: string;
  owner_id: string;
  created_at: string;
  updated_at: string;
}
