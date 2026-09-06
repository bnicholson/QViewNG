
export interface DivisionTS {
  did: string;
  tid: string;
  dname: string;
  breadcrumb: string;
  is_public: boolean;
  shortinfo: string;
  created_at: string;
  updated_at: string;
}

export interface NewDivisionPayload {
  tid: string;
  dname: string;
  breadcrumb: string;
  is_public: boolean;
  shortinfo: string;
}

export interface PagedDivisions {
  count: number;
  items: DivisionTS[];
}

/** Enriched division row: the division plus the display name of the user who last modified it. */
export interface DivisionRowTS extends DivisionTS {
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedDivisionRows {
  count: number;
  items: DivisionRowTS[];
}

export const DivisionAPI = {
  get: async (page: number, size: number): Promise<PagedDivisions> =>
    (await fetch(`/api/divisions?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<DivisionTS[]> =>
    (await fetch(`/api/tournaments/${tid}/divisions?page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched division rows (+ last-modified user name), plus total count. */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedDivisionRows> =>
    (await fetch(`/api/tournaments/${tid}/division-rows?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<DivisionTS> => {
    const response = await fetch(`/api/divisions/${id}`);
    if (!response.ok) throw new Error(`Division not found (${response.status})`);
    return response.json();
  },
  create: async (division: NewDivisionPayload, accessToken?: string): Promise<DivisionTS> => {
    const response = await fetch('/api/divisions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(division),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create division (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (id: string) =>
    await fetch(`/api/divisions/${id}`, { method: 'DELETE' }),
  update: async (id: string, division: Partial<NewDivisionPayload>, accessToken?: string): Promise<DivisionTS> => {
    const response = await fetch(`/api/divisions/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(division),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update division (${response.status}): ${text}`);
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
}
