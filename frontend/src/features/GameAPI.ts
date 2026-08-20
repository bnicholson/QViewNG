export interface GameTS {
  gid: string;
  org: string;
  tournamentid: string;
  divisionid: string;
  roomid: string;
  roundid: string;
  ignore: boolean;
  ruleset: string;
  leftteamid: string;
  centerteamid: string | null;
  rightteamid: string;
  quizmasterid: string;
  contentjudgeid: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewGamePayload {
  org: string;
  tournamentid: string;
  divisionid: string;
  roomid: string;
  roundid: string;
  ignore: boolean;
  ruleset: string;
  leftteamid: string;
  centerteamid?: string | null;
  rightteamid: string;
  quizmasterid: string;
  contentjudgeid?: string | null;
}

export interface PagedGames {
  count: number;
  items: GameTS[];
}

export interface GameStatusTS {
  gid: string;
  done: boolean;
  data_ok: boolean;
  next_question: number | null;
}

export interface GameEventTS {
  gid: string;
  question: number;
  eventnum: number;
  name: string;
  team: number;
  quizzer: number;
  event: string;
  parm1: string;
  parm2: string;
  clientts: string;
  serverts: string;
  md5digest: string;
  qm_registration_key: string | null;
}

export interface GameChangeset {
  org?: string;
  divisionid?: string;
  roomid?: string;
  roundid?: string;
  ignore?: boolean;
  ruleset?: string;
  leftteamid?: string;
  centerteamid?: string | null;
  rightteamid?: string;
  quizmasterid?: string;
  contentjudgeid?: string | null;
}

export const GameAPI = {
  get: async (page: number, size: number): Promise<PagedGames> =>
    (await fetch(`/api/games?page=${page}&page_size=${size}`)).json(),
  // Flag a game so the next ping from its room returns a "resend all events" command.
  requestResend: async (id: string): Promise<void> => {
    const res = await fetch(`/api/games/${id}/request-resend`, { method: 'POST' });
    if (!res.ok) throw new Error(`Failed to request resend (${res.status})`);
  },
  getById: async (id: string): Promise<GameTS> => {
    const response = await fetch(`/api/games/${id}`);
    if (!response.ok) throw new Error(`Game not found (${response.status})`);
    return response.json();
  },
  update: async (id: string, changeset: GameChangeset): Promise<GameTS> => {
    const response = await fetch(`/api/games/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update game (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  getByTournament: async (tid: string, page: number, size: number): Promise<PagedGames> =>
    (await fetch(`/api/tournaments/${tid}/games?page=${page}&page_size=${size}`)).json(),
  // Per-game readiness (done / data_ok) for every game in a tournament.
  getStatuses: async (tid: string): Promise<GameStatusTS[]> =>
    (await fetch(`/api/tournaments/${tid}/gamestatuses`)).json(),
  create: async (game: NewGamePayload): Promise<GameTS> => {
    const response = await fetch('/api/games', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(game),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create game (${response.status}): ${text}`);
    }
    return response.json();
  },
  getGameevents: async (gid: string, page: number, size: number): Promise<GameEventTS[]> => {
    const response = await fetch(`/api/games/${gid}/gameevents?page=${page}&page_size=${size}`);
    if (!response.ok) throw new Error(`Failed to load game events (${response.status})`);
    return response.json();
  },
  getByRound: async (roundid: string, page: number, size: number): Promise<GameTS[]> =>
    (await fetch(`/api/rounds/${roundid}/games?page=${page}&page_size=${size}`)).json(),
  getByRoom: async (roomid: string, page: number, size: number): Promise<GameTS[]> =>
    (await fetch(`/api/rooms/${roomid}/games?page=${page}&page_size=${size}`)).json(),
  getByDivision: async (did: string, page: number, size: number): Promise<GameTS[]> =>
    (await fetch(`/api/divisions/${did}/games?page=${page}&page_size=${size}`)).json(),
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/games/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete game (${response.status}): ${text}`);
    }
  },
};
