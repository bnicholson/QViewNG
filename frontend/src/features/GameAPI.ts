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
