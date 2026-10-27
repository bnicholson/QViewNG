export interface RoundTS {
  roundid: string;
  /** The division the round belongs to. */
  did: string;
  name: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewRoundPayload {
  did: string;
  name: string;
  scheduled_start_time: string | null; // ISO 8601 datetime string; null when not scheduled
}

export interface PagedRounds {
  count: number;
  items: RoundTS[];
}

/**
 * One fully-formed row of the rounds data table: the round plus its division id/name, so the whole
 * table is populated from a single request per page.
 */
export interface RoundRowTS {
  roundid: string;
  did: string;
  division_name: string;
  name: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedRoundRows {
  count: number;
  items: RoundRowTS[];
}

export const RoundAPI = {
  get: async (page: number, size: number): Promise<PagedRounds> =>
    (await fetch(`/api/rounds?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/tournaments/${tid}/rounds?page=${page}&page_size=${size}`)).json(),
  getByDivision: async (did: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/divisions/${did}/rounds?page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched round rows (division name), plus total count. */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedRoundRows> =>
    (await fetch(`/api/tournaments/${tid}/round-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of the division's enriched round rows (division name), plus total count. */
  getRowsByDivision: async (did: string, page: number, size: number): Promise<PagedRoundRows> =>
    (await fetch(`/api/divisions/${did}/round-rows?page=${page}&page_size=${size}`)).json(),
  create: async (round: NewRoundPayload, accessToken?: string): Promise<RoundTS> => {
    const response = await fetch('/api/rounds', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(round),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create round (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  getById: async (id: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`);
    if (!response.ok) throw new Error(`Round not found (${response.status})`);
    return response.json();
  },
  /** Update a round's name and/or start time. */
  update: async (id: string, payload: { name?: string; scheduled_start_time?: string | null }, accessToken?: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update round (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const response = await fetch(`/api/rounds/${id}`, {
      method: 'DELETE',
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!response.ok) {
      // Surface the server's message (e.g. the "a game is associated" guard) when present.
      let message = `Failed to delete round (${response.status})`;
      try {
        const body = await response.json();
        if (body?.error) message = body.error;
      } catch { /* non-JSON body */ }
      throw new Error(message);
    }
  },
}
