export interface PoolBracketTS {
  pool_bracket_id: string;
  divisionid: string;
  type: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
  name: string;
  /** The poolbracketgroup this pool belongs to (teams are placed one-per-pool within a group). */
  poolbracketgroupid: string | null;
}

export interface NewPoolBracketPayload {
  divisionid: string;
  name: string;
  type: string;
  poolbracketgroupid?: string | null;
}

export interface PoolBracketChangeset {
  divisionid?: string;
  name?: string;
  type?: string;
  poolbracketgroupid?: string | null;
}

/**
 * One fully-formed row of the pool-brackets data table: the bracket plus its parent division name
 * and the display name of the user who last modified it.
 */
export interface PoolBracketRowTS {
  pool_bracket_id: string;
  did: string;
  division_name: string;
  name: string;
  type: string;
  created_date: string;
  last_modified_date: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedPoolBracketRows {
  count: number;
  items: PoolBracketRowTS[];
}

export const PoolBracketAPI = {
  /** All pool brackets in a division. */
  getByDivision: async (did: string): Promise<PoolBracketTS[]> =>
    (await fetch(`/api/divisions/${did}/pool-brackets`)).json(),
  /** One page of the division's enriched pool-bracket rows for a given `type`, plus total count. */
  getRowsByDivision: async (did: string, type: string, page: number, size: number): Promise<PagedPoolBracketRows> =>
    (await fetch(`/api/divisions/${did}/pool-bracket-rows?type=${encodeURIComponent(type)}&page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched pool-bracket rows for a given `type` (across all divisions). */
  getRowsByTournament: async (tid: string, type: string, page: number, size: number): Promise<PagedPoolBracketRows> =>
    (await fetch(`/api/tournaments/${tid}/pool-bracket-rows?type=${encodeURIComponent(type)}&page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<PoolBracketTS> => {
    const response = await fetch(`/api/poolbrackets/${id}`);
    if (!response.ok) throw new Error(`Pool bracket not found (${response.status})`);
    return response.json();
  },
  create: async (bracket: NewPoolBracketPayload, accessToken?: string): Promise<PoolBracketTS> => {
    const response = await fetch('/api/poolbrackets', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(bracket),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  update: async (id: string, changeset: PoolBracketChangeset, accessToken?: string): Promise<PoolBracketTS> => {
    const response = await fetch(`/api/poolbrackets/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/poolbrackets/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete (${response.status}): ${text}`);
    }
  },
}
