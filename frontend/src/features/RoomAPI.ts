export interface RoomTS {
  roomid: string;
  tid: string;
  name: string;
  building: string;
  comments: string;
  clientkey: string;
  created_at: string;
  updated_at: string;
  quizmaster_id: string | null;
  contentjudge_id: string | null;
}

export interface NewRoomPayload {
  tid: string;
  name: string;
  building: string;
  comments: string;
  clientkey: string;
  quizmaster_id?: string | null;
  contentjudge_id?: string | null;
}

export interface PagedRooms {
  count: number;
  items: RoomTS[];
}

export interface RoomMonitorRowTS {
  roomid: string;
  check_in: string | null;
  client_ts: string | null;
  room: string | null;
  round: string | null;
  question: number | null;
  host_ip: string | null;
  qm_version: string | null;
  pending: number | null;
  resend: string | null;
  game_id: string | null;
  game_in_progress: boolean;
  data_incomplete: boolean;
}

export const RoomAPI = {
  // Live Room Monitor rows (latest ping + referenced game's resend status) for a tournament.
  // Access is restricted server-side to super users, the tournament owner, and admins.
  getMonitorByTournament: async (tid: string, accessToken?: string): Promise<RoomMonitorRowTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/roommonitor`, {
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!res.ok) throw new Error(`Failed to load room monitor (${res.status})`);
    return res.json();
  },
  get: async (page: number, size: number): Promise<PagedRooms> =>
    (await fetch(`/api/rooms?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<RoomTS[]> =>
    (await fetch(`/api/tournaments/${tid}/rooms?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<RoomTS> => {
    const response = await fetch(`/api/rooms/${id}`);
    if (!response.ok) throw new Error(`Room not found (${response.status})`);
    return response.json();
  },
  create: async (room: NewRoomPayload, accessToken?: string): Promise<RoomTS> => {
    const response = await fetch('/api/rooms', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(room),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create room (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const response = await fetch(`/api/rooms/${id}`, {
      method: 'DELETE',
      headers: {
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete room (${response.status}): ${text}`);
    }
  },
  update: async (id: string, room: Partial<NewRoomPayload>, accessToken?: string): Promise<RoomTS> => {
    const response = await fetch(`/api/rooms/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(room),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update room (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
}
