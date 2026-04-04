export interface TeamTS {
  teamid: string;
  did: string;
  coachid: string;
  name: string;
  created_at: string;
  updated_at: string;
  quizzer_one_id: string | null;
  quizzer_two_id: string | null;
  quizzer_three_id: string | null;
  quizzer_four_id: string | null;
  quizzer_five_id: string | null;
  quizzer_six_id: string | null;
}

export interface NewTeamPayload {
  did: string;
  coachid: string;
  name: string;
  quizzer_one_id?: string | null;
  quizzer_two_id?: string | null;
  quizzer_three_id?: string | null;
  quizzer_four_id?: string | null;
  quizzer_five_id?: string | null;
  quizzer_six_id?: string | null;
}

export interface TeamChangeset {
  name?: string;
  did?: string;
  coachid?: string;
  quizzer_one_id?: string | null;
  quizzer_two_id?: string | null;
  quizzer_three_id?: string | null;
  quizzer_four_id?: string | null;
  quizzer_five_id?: string | null;
  quizzer_six_id?: string | null;
}

export interface TeamWithCoachTS extends TeamTS {
  coach_name: string;
}

export interface PagedTeams {
  count: number;
  items: TeamTS[];
}

export interface PagedTeamsWithCoach {
  count: number;
  items: TeamWithCoachTS[];
}

async function throwApiError(verb: string, status: number, body: string): Promise<never> {
  let message: string
  try {
    const json = JSON.parse(body)
    if (typeof json?.error === 'string') {
      message = `Failed to ${verb} team. Error: ${json.error} (${status})`
    } else {
      message = `Failed to ${verb} team (${status}): ${body}`
    }
  } catch {
    message = `Failed to ${verb} team (${status}): ${body}`
  }
  throw new Error(message)
}

export const TeamAPI = {
  get: async (page: number, size: number): Promise<PagedTeams> =>
    (await fetch(`/api/teams?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<PagedTeamsWithCoach> =>
    (await fetch(`/api/tournaments/${tid}/teams?page=${page}&page_size=${size}`)).json(),
  getByDivision: async (did: string, page: number, size: number): Promise<TeamTS[]> =>
    (await fetch(`/api/divisions/${did}/teams?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<TeamTS> => {
    const response = await fetch(`/api/teams/${id}`);
    if (!response.ok) throw new Error(`Team not found (${response.status})`);
    return response.json();
  },
  create: async (team: NewTeamPayload, accessToken?: string): Promise<TeamTS> => {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (accessToken) headers['Authorization'] = `Bearer ${accessToken}`;
    const response = await fetch('/api/teams', {
      method: 'POST',
      headers,
      body: JSON.stringify(team),
    });
    if (!response.ok) await throwApiError('create', response.status, await response.text());
    return (await response.json()).data;
  },
  update: async (id: string, changeset: TeamChangeset, accessToken?: string): Promise<TeamTS> => {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (accessToken) headers['Authorization'] = `Bearer ${accessToken}`;
    const response = await fetch(`/api/teams/${id}`, {
      method: 'PUT',
      headers,
      body: JSON.stringify(changeset),
    });
    if (!response.ok) await throwApiError('update', response.status, await response.text());
    return (await response.json()).data;
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const headers: Record<string, string> = {};
    if (accessToken) headers['Authorization'] = `Bearer ${accessToken}`;
    const response = await fetch(`/api/teams/${id}`, { method: 'DELETE', headers });
    if (!response.ok) await throwApiError('delete', response.status, await response.text());
  },
};
