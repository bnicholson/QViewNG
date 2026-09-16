export interface RoundGroupTS {
  roundgroup_id: string;
  did: string;
  name: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
}

export interface NewRoundGroupPayload {
  did: string;
  name: string;
}

export interface RoundGroupChangeset {
  did?: string;
  name?: string;
}

/**
 * One fully-formed row of the roundgroups data table: the roundgroup plus its division name and the
 * display name of the user who last modified it, so the whole table is populated per page.
 */
export interface RoundGroupRowTS {
  roundgroup_id: string;
  did: string;
  division_name: string;
  name: string;
  created_date: string;
  last_modified_date: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedRoundGroupRows {
  count: number;
  items: RoundGroupRowTS[];
}

export const RoundGroupAPI = {
  /** Plain list of a division's roundgroups. */
  getByDivision: async (did: string): Promise<RoundGroupTS[]> =>
    (await fetch(`/api/divisions/${did}/roundgroups`)).json(),
  /** One page of the division's enriched roundgroup rows, plus total count. */
  getRowsByDivision: async (did: string, page: number, size: number): Promise<PagedRoundGroupRows> =>
    (await fetch(`/api/divisions/${did}/roundgroup-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched roundgroup rows (across all divisions), plus total count. */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedRoundGroupRows> =>
    (await fetch(`/api/tournaments/${tid}/roundgroup-rows?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<RoundGroupTS> => {
    const response = await fetch(`/api/roundgroups/${id}`);
    if (!response.ok) throw new Error(`Session not found (${response.status})`);
    return response.json();
  },
  create: async (roundgroup: NewRoundGroupPayload, accessToken?: string): Promise<RoundGroupTS> => {
    const response = await fetch('/api/roundgroups', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(roundgroup),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create session (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  update: async (id: string, changeset: RoundGroupChangeset, accessToken?: string): Promise<RoundGroupTS> => {
    const response = await fetch(`/api/roundgroups/${id}`, {
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
    const response = await fetch(`/api/roundgroups/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete session (${response.status}): ${text}`);
    }
  },
}
