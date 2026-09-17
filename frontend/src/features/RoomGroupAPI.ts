export interface RoomGroupTS {
  roomgroupid: string;
  tournamentid: string;
  type: string;
  name: string;
  notes: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
}

export interface NewRoomGroupPayload {
  tournamentid: string;
  type?: string;
  name: string;
  notes?: string;
}

export interface RoomGroupChangeset {
  type?: string;
  name?: string;
  notes?: string;
}

export const RoomGroupAPI = {
  /** The roomgroups (e.g. buildings) a tournament uses — every roomgroup referenced by its rooms. */
  getByTournament: async (tid: string): Promise<RoomGroupTS[]> =>
    (await fetch(`/api/tournaments/${tid}/roomgroups`)).json(),
  getById: async (id: string): Promise<RoomGroupTS> => {
    const response = await fetch(`/api/roomgroups/${id}`);
    if (!response.ok) throw new Error(`Roomgroup not found (${response.status})`);
    return response.json();
  },
  create: async (roomgroup: NewRoomGroupPayload, accessToken?: string): Promise<RoomGroupTS> => {
    const response = await fetch('/api/roomgroups', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(roomgroup),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create roomgroup (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  update: async (id: string, changeset: RoomGroupChangeset, accessToken?: string): Promise<RoomGroupTS> => {
    const response = await fetch(`/api/roomgroups/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update roomgroup (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const response = await fetch(`/api/roomgroups/${id}`, {
      method: 'DELETE',
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete roomgroup (${response.status}): ${text}`);
    }
  },
}
