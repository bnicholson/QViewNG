export interface DivisionSessionTS {
  division_session_id: string;
  did: string;
  name: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
}

export interface NewDivisionSessionPayload {
  did: string;
  name: string;
}

export interface DivisionSessionChangeset {
  did?: string;
  name?: string;
}

/**
 * One fully-formed row of the sessions data table: the session plus its division name and the
 * display name of the user who last modified it, so the whole table is populated per page.
 */
export interface DivisionSessionRowTS {
  division_session_id: string;
  did: string;
  division_name: string;
  name: string;
  created_date: string;
  last_modified_date: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedDivisionSessionRows {
  count: number;
  items: DivisionSessionRowTS[];
}

export const DivisionSessionAPI = {
  /** Plain list of a division's sessions. */
  getByDivision: async (did: string): Promise<DivisionSessionTS[]> =>
    (await fetch(`/api/divisions/${did}/sessions`)).json(),
  /** One page of the division's enriched session rows, plus total count. */
  getRowsByDivision: async (did: string, page: number, size: number): Promise<PagedDivisionSessionRows> =>
    (await fetch(`/api/divisions/${did}/session-rows?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<DivisionSessionTS> => {
    const response = await fetch(`/api/divisionsessions/${id}`);
    if (!response.ok) throw new Error(`Session not found (${response.status})`);
    return response.json();
  },
  create: async (session: NewDivisionSessionPayload, accessToken?: string): Promise<DivisionSessionTS> => {
    const response = await fetch('/api/divisionsessions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(session),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create session (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  update: async (id: string, changeset: DivisionSessionChangeset, accessToken?: string): Promise<DivisionSessionTS> => {
    const response = await fetch(`/api/divisionsessions/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update session (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/divisionsessions/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete session (${response.status}): ${text}`);
    }
  },
}
