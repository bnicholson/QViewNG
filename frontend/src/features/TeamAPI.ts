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

export const TeamAPI = {
  get: async (page: number, size: number): Promise<TeamTS[]> =>
    (await fetch(`/api/teams?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<TeamTS> => {
    const response = await fetch(`/api/teams/${id}`);
    if (!response.ok) throw new Error(`Team not found (${response.status})`);
    return response.json();
  },
  create: async (team: NewTeamPayload): Promise<TeamTS> => {
    const response = await fetch('/api/teams', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(team),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create team (${response.status}): ${text}`);
    }
    return response.json();
  },
  update: async (id: string, changeset: TeamChangeset): Promise<TeamTS> => {
    const response = await fetch(`/api/teams/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update team (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/teams/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete team (${response.status}): ${text}`);
    }
  },
};
