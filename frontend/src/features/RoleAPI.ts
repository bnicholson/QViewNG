export interface RoleTS {
  id: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

/** `tournament_manager` → "Tournament Manager". */
export const roleDisplayName = (name: string): string =>
  name.split('_').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')

const authHeaders = (accessToken?: string): Record<string, string> =>
  accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}

export const RoleAPI = {
  getAll: async (): Promise<RoleTS[]> => {
    const response = await fetch('/api/roles')
    if (!response.ok) throw new Error(`Failed to load roles (${response.status})`)
    return (await response.json()).items
  },
  /** The ids of the roles assigned to a user. Superuser-only. */
  getRoleIdsForUser: async (userId: string, accessToken?: string): Promise<string[]> => {
    const response = await fetch(`/api/usersroles/users/${userId}`, { headers: authHeaders(accessToken) })
    if (!response.ok) throw new Error(`Failed to load the user's roles (${response.status})`)
    return (await response.json()).role_ids
  },
  /** Make `roleIds` exactly the user's roles. Superuser-only. */
  setRoleIdsForUser: async (userId: string, roleIds: string[], accessToken?: string): Promise<string[]> => {
    const response = await fetch(`/api/usersroles/users/${userId}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json', ...authHeaders(accessToken) },
      body: JSON.stringify({ role_ids: roleIds }),
    })
    if (!response.ok) {
      const body = await response.json().catch(() => ({}))
      throw new Error(body.error ?? `Failed to save roles (${response.status})`)
    }
    return (await response.json()).role_ids
  },
}
