export interface TournamentGroupTS {
  tgid: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
  creator_id: string;
  owner_id: string;
}

export interface NewTournamentGroupPayload {
  name: string;
  description: string | null;
}

export interface TournamentGroupChangeset {
  name: string;
  description: string | null;
}

export const TournamentGroupAPI = {
  getAll: async (page: number, size: number): Promise<{ count: number; items: TournamentGroupTS[] }> => {
    const res = await fetch(`/api/tournamentgroups?page=${page}&page_size=${size}`);
    if (!res.ok) throw new Error(`Failed to load tournament groups (${res.status})`);
    return res.json();
  },

  getByTournament: async (tid: string, page: number, size: number): Promise<TournamentGroupTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/tournamentgroups?page=${page}&page_size=${size}`);
    if (!res.ok) throw new Error(`Failed to load tournament groups (${res.status})`);
    return res.json();
  },

  getById: async (tgid: string): Promise<TournamentGroupTS> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}`);
    if (!res.ok) throw new Error(`Tournament group not found (${res.status})`);
    return res.json();
  },

  // Creates a new group and optionally associates it with a tournament when tid is provided.
  create: async (tid: string | undefined, payload: NewTournamentGroupPayload, accessToken: string): Promise<TournamentGroupTS> => {
    const authHeaders = { 'Content-Type': 'application/json', 'Authorization': `Bearer ${accessToken}` };

    const createRes = await fetch('/api/tournamentgroups', {
      method: 'POST',
      headers: authHeaders,
      body: JSON.stringify(payload),
    });
    if (!createRes.ok) {
      const text = await createRes.text();
      throw new Error(`Failed to create tournament group (${createRes.status}): ${text}`);
    }
    const envelope = await createRes.json();
    const group: TournamentGroupTS = envelope.data ?? envelope;

    if (tid) {
      const linkRes = await fetch(`/api/tournamentgroups/${group.tgid}/tournaments`, {
        method: 'POST',
        headers: authHeaders,
        body: JSON.stringify({ tournamentgroupid: group.tgid, tournamentid: tid }),
      });
      if (!linkRes.ok) {
        const text = await linkRes.text();
        throw new Error(`Group created but failed to link to tournament (${linkRes.status}): ${text}`);
      }
    }

    return group;
  },

  update: async (tgid: string, changeset: TournamentGroupChangeset): Promise<TournamentGroupTS> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to update tournament group (${res.status}): ${text}`);
    }
    const envelope = await res.json();
    return envelope.data ?? envelope;
  },

  delete: async (tgid: string): Promise<void> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}`, { method: 'DELETE' });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to delete tournament group (${res.status}): ${text}`);
    }
  },

  // Removes this tournament's association with the group (does not delete the group globally).
  removeFromTournament: async (tgid: string, tid: string): Promise<void> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}/tournaments/${tid}`, { method: 'DELETE' });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to remove tournament group association (${res.status}): ${text}`);
    }
  },

  // Returns all tournaments linked to a group.
  getTournaments: async (tgid: string, page: number, size: number): Promise<any[]> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}/tournaments?page=${page}&page_size=${size}`);
    if (!res.ok) throw new Error(`Failed to load tournaments for group (${res.status})`);
    return res.json();
  },

  // Links an existing tournament to this group.
  addTournament: async (tgid: string, tid: string): Promise<void> => {
    const res = await fetch(`/api/tournamentgroups/${tgid}/tournaments`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ tournamentgroupid: tgid, tournamentid: tid }),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to add tournament to group (${res.status}): ${text}`);
    }
  },
};
