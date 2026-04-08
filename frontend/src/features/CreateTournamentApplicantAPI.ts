export interface CreateTournamentApplicantTS {
  id: string;
  user_id: string;
  request_context: string | null;
  status: string;
  created_at: string;
  modified_at: string;
  last_modified_user_id: string;
}

export interface CreateTournamentApplicantChangeset {
  status?: string;
  request_context?: string | null;
  last_modified_user_id: string;
}

export interface PagedCreateTournamentApplicants {
  count: number;
  items: CreateTournamentApplicantTS[];
}

export const CreateTournamentApplicantAPI = {
  get: async (page: number, size: number): Promise<PagedCreateTournamentApplicants> =>
    (await fetch(`/api/createtournamentapplicants?page=${page}&page_size=${size}`)).json(),

  update: async (id: string, changeset: CreateTournamentApplicantChangeset): Promise<CreateTournamentApplicantTS> => {
    const response = await fetch(`/api/createtournamentapplicants/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update applicant (${response.status}): ${text}`);
    }
    const body = await response.json();
    return body.data;
  },

  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/createtournamentapplicants/${id}`, { method: 'DELETE' });
    if (!response.ok) throw new Error(`Failed to delete applicant (${response.status})`);
  },
};
