export interface RoundTS {
  roundid: string;
  /** A round belongs to a (time-bound) division session. */
  division_session_id: string;
  name: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewRoundPayload {
  division_session_id: string;
  name: string;
  scheduled_start_time: string | null; // ISO 8601 datetime string; null when not scheduled
}

export interface PagedRounds {
  count: number;
  items: RoundTS[];
}

/**
 * One fully-formed row of the rounds data table: the round plus its session name and its (derived)
 * division id/name, so the whole table is populated from a single request per page.
 */
export interface RoundRowTS {
  roundid: string;
  division_session_id: string;
  session_name: string;
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
  /** A division session's rounds (the new, time-bound relationship). */
  getByDivisionSession: async (sessionId: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/divisionsessions/${sessionId}/rounds?page=${page}&page_size=${size}`)).json(),
  /** One page of a division session's enriched round rows, plus total count. */
  getRowsByDivisionSession: async (sessionId: string, page: number, size: number): Promise<PagedRoundRows> =>
    (await fetch(`/api/divisionsessions/${sessionId}/round-rows?page=${page}&page_size=${size}`)).json(),
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
    return response.json();
  },
  getById: async (id: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`);
    if (!response.ok) throw new Error(`Round not found (${response.status})`);
    return response.json();
  },
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
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/rounds/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete round (${response.status}): ${text}`);
    }
  },
}
