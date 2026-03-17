
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

export const DivisionAPI = {
  get: async (page: number, size: number): Promise<DivisionTS[]> =>
    (await fetch(`/api/divisions?page=${page}&page_size=${size}`)).json(),
  create: async (division: NewDivisionPayload): Promise<DivisionTS> => {
    const response = await fetch('/api/divisions', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
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
  update: async (id: string, division: Partial<NewDivisionPayload>) =>
    await fetch(`/api/divisions/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(division),
    }),
}
