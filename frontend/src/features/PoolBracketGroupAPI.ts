import { type PoolBracketTS } from './PoolBracketAPI';

export interface PoolBracketGroupTS {
  poolbracketgroupid: string;
  divisionid: string;
  name: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
}

export interface NewPoolBracketGroupPayload {
  divisionid: string;
  name: string;
}

export interface PoolBracketGroupChangeset {
  divisionid?: string;
  name?: string;
}

/**
 * One fully-formed row of the pool/bracket-groups data table: the group plus its parent division
 * name and the display name of the user who last modified it, so the whole table is populated per page.
 */
export interface PoolBracketGroupRowTS {
  poolbracketgroupid: string;
  did: string;
  division_name: string;
  name: string;
  created_date: string;
  last_modified_date: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedPoolBracketGroupRows {
  count: number;
  items: PoolBracketGroupRowTS[];
}

export const PoolBracketGroupAPI = {
  /** One page of the division's enriched pool/bracket-group rows, plus total count. */
  getRowsByDivision: async (did: string, page: number, size: number): Promise<PagedPoolBracketGroupRows> =>
    (await fetch(`/api/divisions/${did}/poolbracketgroup-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched pool/bracket-group rows (across all divisions). */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedPoolBracketGroupRows> =>
    (await fetch(`/api/tournaments/${tid}/poolbracketgroup-rows?page=${page}&page_size=${size}`)).json(),
  /** Every pool and bracket belonging to one group (both types, unpaged). */
  getPoolBrackets: async (id: string): Promise<PoolBracketTS[]> =>
    (await fetch(`/api/poolbracketgroups/${id}/poolbrackets`)).json(),
  /** All poolbracketgroups in a division. */
  getByDivision: async (did: string): Promise<PoolBracketGroupTS[]> =>
    (await fetch(`/api/divisions/${did}/poolbracketgroups`)).json(),
  getById: async (id: string): Promise<PoolBracketGroupTS> => {
    const response = await fetch(`/api/poolbracketgroups/${id}`);
    if (!response.ok) throw new Error(`Pool group not found (${response.status})`);
    return response.json();
  },
  create: async (group: NewPoolBracketGroupPayload, accessToken?: string): Promise<PoolBracketGroupTS> => {
    const response = await fetch('/api/poolbracketgroups', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(group),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create pool group (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  update: async (id: string, changeset: PoolBracketGroupChangeset, accessToken?: string): Promise<PoolBracketGroupTS> => {
    const response = await fetch(`/api/poolbracketgroups/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update pool group (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const response = await fetch(`/api/poolbracketgroups/${id}`, {
      method: 'DELETE',
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!response.ok) {
      // Surface the server's message (e.g. the "still has pools" guard) when present.
      let message = `Failed to delete pool group (${response.status})`;
      try {
        const body = await response.json();
        if (body?.error) message = body.error;
      } catch { /* non-JSON body */ }
      throw new Error(message);
    }
  },
}
