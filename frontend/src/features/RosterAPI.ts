export interface RosterTS {
  rosterid: string;
  name: string;
  description: string | null;
  created_by_userid: string;
  created_at: string;
  updated_at: string;
}

export interface NewRosterPayload {
  name: string;
  description: string | null;
  created_by_userid: string;
}

export interface RosterChangeset {
  name: string;
  description: string | null;
}

export interface PagedRosters {
  count: number;
  items: RosterTS[];
}

export const RosterAPI = {
  getByCoach: async (coachId: string): Promise<RosterTS[]> => {
    const response = await fetch(`/api/users/${coachId}/rosters-of-coach?page=0&page_size=100`);
    if (!response.ok) throw new Error(`Failed to load rosters (${response.status})`);
    return response.json();
  },
  getById: async (id: string): Promise<RosterTS> => {
    const response = await fetch(`/api/rosters/${id}`);
    if (!response.ok) throw new Error(`Roster not found (${response.status})`);
    return response.json();
  },
  create: async (coachId: string, payload: NewRosterPayload): Promise<RosterTS> => {
    const response = await fetch(`/api/users/${coachId}/rosters`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create roster (${response.status}): ${text}`);
    }
    const result = await response.json();
    return result.data;
  },
  update: async (id: string, changeset: RosterChangeset): Promise<RosterTS> => {
    const response = await fetch(`/api/rosters/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update roster (${response.status}): ${text}`);
    }
    const result = await response.json();
    return result.data;
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/rosters/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete roster (${response.status}): ${text}`);
    }
  },
  getQuizzers: async (rosterId: string): Promise<import('./UserAPI').UserTS[]> => {
    const response = await fetch(`/api/rosters/${rosterId}/quizzers?page=0&page_size=100`);
    if (!response.ok) throw new Error(`Failed to load quizzers (${response.status})`);
    return response.json();
  },
  addQuizzer: async (rosterId: string, quizzerId: string): Promise<void> => {
    const response = await fetch(`/api/rosters/${rosterId}/quizzers/${quizzerId}`, {
      method: 'POST',
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to add quizzer (${response.status}): ${text}`);
    }
  },
  removeQuizzer: async (rosterId: string, quizzerId: string): Promise<void> => {
    const response = await fetch(`/api/rosters/${rosterId}/quizzers/${quizzerId}`, {
      method: 'DELETE',
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to remove quizzer (${response.status}): ${text}`);
    }
  },
  getCoaches: async (rosterId: string): Promise<import('./UserAPI').UserTS[]> => {
    const response = await fetch(`/api/rosters/${rosterId}/coaches?page=0&page_size=100`);
    if (!response.ok) throw new Error(`Failed to load coaches (${response.status})`);
    return response.json();
  },
  addCoach: async (rosterId: string, coachId: string): Promise<void> => {
    const response = await fetch(`/api/rosters/${rosterId}/coaches`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ coachid: coachId, rosterid: rosterId }),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to add coach (${response.status}): ${text}`);
    }
  },
  removeCoach: async (rosterId: string, coachId: string): Promise<void> => {
    const response = await fetch(`/api/rosters/${rosterId}/coaches/${coachId}`, {
      method: 'DELETE',
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to remove coach (${response.status}): ${text}`);
    }
  },
};
