import type { UserTS } from './UserAPI';

export interface NewTournamentAdminPayload {
  tournamentid: string;
  adminid: string;
  role_description: string;
  access_lvl: number;
}

export const AdminAPI = {
  getByTournament: async (tid: string, page: number, size: number): Promise<UserTS[]> =>
    (await fetch(`/api/tournaments/${tid}/admins?page=${page}&page_size=${size}`)).json(),
  create: async (tid: string, payload: NewTournamentAdminPayload): Promise<UserTS> => {
    const response = await fetch(`/api/tournaments/${tid}/admins`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to add admin (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (tid: string, userId: string): Promise<void> => {
    const response = await fetch(`/api/tournaments/${tid}/admins/${userId}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to remove admin (${response.status}): ${text}`);
    }
  },
}
