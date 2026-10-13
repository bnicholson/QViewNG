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
  /** Optional roomgroup (e.g. building) this room belongs to. */
  roomgroupid: string | null;
  /** Whether this room may host 3-team games (games with a center team). */
  supports_threeteam_games: boolean;
}

export interface NewRoomPayload {
  tid: string;
  name: string;
  building: string;
  comments: string;
  clientkey: string;
  quizmaster_id?: string | null;
  contentjudge_id?: string | null;
  roomgroupid?: string | null;
  supports_threeteam_games?: boolean;
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
  /** The room's own position, from its pings. */
  question: number | null;
  /**
   * The question the game events the server holds put the room on (the same number Games in
   * Progress shows). Behind `question` while the room's event queue is backed up. Null until the
   * game has events, or when they cannot be scored.
   */
  event_question: number | null;
  host_ip: string | null;
  qm_version: string | null;
  pending: number | null;
  resend: string | null;
  game_id: string | null;
  game_in_progress: boolean;
  data_incomplete: boolean;
  resend_sent: boolean;
  /**
   * Where this game's resend request stands, decided server-side: asked for but not yet picked up
   * by the room, picked up within the last 30 seconds, or picked up longer ago with the data still
   * incomplete. Null when there is no request, or the data is whole again.
   */
  resend_status: 'requested' | 'received' | 'failed' | null;
  /** The room's main row (its current game), which carries the room-level state below. */
  is_room_row: boolean;
  // Room-level queue state (filled on the room's main row only).
  /** Events acknowledged to the client but set aside, not applied. */
  quarantine_count: number;
  /** The client looks stuck: pending above zero and not falling for 90 seconds. Decided server-side. */
  pending_stalled: boolean;
  /** The in-flight or recently finished Clear Pending + Resend, if any. */
  clear_status: ClearStatus | null;
  /** How many events that clear set aside. */
  clear_cleared_count: number;
}

export type ClearStatus = 'clearing' | 'resending' | 'complete' | 'unconfirmed' | 'cancelled' | 'expired';

/** An event the server acknowledged to a room's client but did not apply. */
export interface QuarantinedGameEventTS {
  quarantine_id: number;
  created_at: string;
  /** 'rejected': failed a check it would fail on every retry. 'cleared': set aside while clearing. */
  kind: 'rejected' | 'cleared';
  reason: string;
  roomid: string | null;
  room_clear_id: number | null;
  gid_raw: string;
  gid: string | null;
  tournament: string;
  division: string;
  room: string;
  round: string;
  question: number;
  eventnum: number;
  event: string;
  name: string;
  team: number;
  quizzer: number;
  parm1: string;
  parm2: string;
  clientkey: string;
  query_string: string;
}

/** Enriched room row: the room plus the display name of the user who last modified it. */
export interface RoomRowTS {
  roomid: string;
  name: string;
  building: string;
  comments: string;
  created_at: string;
  updated_at: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedRoomRows {
  count: number;
  items: RoomRowTS[];
}

const bearer = (accessToken?: string): Record<string, string> =>
  accessToken ? { Authorization: `Bearer ${accessToken}` } : {};

/** Reads the server's `{ error }` message when it sent one. */
async function failure(res: Response, fallback: string): Promise<Error> {
  try {
    const body = await res.json();
    if (body && typeof body.error === 'string') return new Error(body.error);
  } catch { /* no JSON body */ }
  return new Error(`${fallback} (${res.status})`);
}

export const RoomAPI = {
  /** The room's quarantined events, newest first. Server Monitor staff only. */
  getQuarantine: async (roomid: string, accessToken?: string): Promise<QuarantinedGameEventTS[]> => {
    const res = await fetch(`/api/rooms/${roomid}/quarantine`, { headers: bearer(accessToken) });
    if (!res.ok) throw await failure(res, 'Failed to load quarantined events');
    return res.json();
  },
  /** Starts a Clear Pending + Resend for the room. */
  startClearPending: async (roomid: string, accessToken?: string): Promise<void> => {
    const res = await fetch(`/api/rooms/${roomid}/clear-pending`, { method: 'POST', headers: bearer(accessToken) });
    if (!res.ok) throw await failure(res, 'Failed to start Clear Pending + Resend');
  },
  /** Stops a clear that is still acknowledging events. */
  cancelClearPending: async (roomid: string, accessToken?: string): Promise<void> => {
    const res = await fetch(`/api/rooms/${roomid}/clear-pending`, { method: 'DELETE', headers: bearer(accessToken) });
    if (!res.ok) throw await failure(res, 'Failed to cancel Clear Pending + Resend');
  },
  /** One page of the tournament's enriched room rows (+ last-modified user name), plus total count. */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedRoomRows> =>
    (await fetch(`/api/tournaments/${tid}/room-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of a roomgroup's (building's) enriched room rows, plus total count. */
  getRowsByRoomGroup: async (roomgroupid: string, page: number, size: number): Promise<PagedRoomRows> =>
    (await fetch(`/api/roomgroups/${roomgroupid}/room-rows?page=${page}&page_size=${size}`)).json(),
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
