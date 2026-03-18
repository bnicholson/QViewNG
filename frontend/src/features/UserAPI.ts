export interface UserTS {
  id: string;
  username: string;
  email: string;
  fname: string;
  mname: string;
  lname: string;
  activated: boolean;
  created_at: string;
  updated_at: string;
}

export interface NewUserPayload {
  fname: string;
  mname: string;
  lname: string;
  username: string;
  email: string;
  hash_password: string;
  activated: boolean;
}

export interface UserChangeset {
  fname?: string;
  mname?: string;
  lname?: string;
  username?: string;
  email?: string;
  activated?: boolean;
}

export interface PagedUsers {
  count: number;
  items: UserTS[];
}

export const UserAPI = {
  get: async (page: number, size: number): Promise<PagedUsers> =>
    (await fetch(`/api/users?page=${page}&page_size=${size}`)).json(),
  getById: async (id: string): Promise<UserTS> => {
    const response = await fetch(`/api/users/${id}`);
    if (!response.ok) throw new Error(`User not found (${response.status})`);
    return response.json();
  },
  create: async (user: NewUserPayload): Promise<UserTS> => {
    const response = await fetch('/api/users', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(user),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to create user (${response.status}): ${text}`);
    }
    return response.json();
  },
  update: async (id: string, changeset: UserChangeset): Promise<UserTS> => {
    const response = await fetch(`/api/users/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to update user (${response.status}): ${text}`);
    }
    return response.json();
  },
  delete: async (id: string): Promise<void> => {
    const response = await fetch(`/api/users/${id}`, { method: 'DELETE' });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Failed to delete user (${response.status}): ${text}`);
    }
  },
}
