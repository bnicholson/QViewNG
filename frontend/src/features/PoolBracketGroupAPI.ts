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

export const PoolBracketGroupAPI = {
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
