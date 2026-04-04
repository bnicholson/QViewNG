export interface RoundTS {
  roundid: string;
  did: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewRoundPayload {
  did: string;
  scheduled_start_time: string; // ISO 8601 datetime string
}

export interface PagedRounds {
  count: number;
  items: RoundTS[];
}

export const RoundAPI = {
  get: async (page: number, size: number): Promise<PagedRounds> =>
    (await fetch(`/api/rounds?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/tournaments/${tid}/rounds?page=${page}&page_size=${size}`)).json(),
  create: async (round: NewRoundPayload): Promise<RoundTS> => {
    const response = await fetch('/api/rounds', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(round),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create round (${response.status}): ${text}`);
    }
    return response.json();
  },
  getById: async (id: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`);
    if (!response.ok) throw new Error(`Round not found (${response.status})`);
    return response.json();
  },
  update: async (id: string, payload: { scheduled_start_time: string | null }): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update round (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/rounds/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete round (${response.status}): ${text}`);
    }
  },
}
