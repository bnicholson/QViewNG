import type { GearSetTS, EquipmentDboTS, EquipmentDetail } from './EquipmentSetAPI'

export interface EquipmentRegistrationTS {
  id: number;
  equipmentid: number;
  tournamentid: string;
  roomid: string | null;
  status: string;
  created_at: string;
  updated_at: string;
}

/** One gear item with its type detail and (if any) its registration for a tournament. */
export interface GearItemWithRegistrationTS {
  dbo: EquipmentDboTS;
  detail: EquipmentDetail | null;
  registration: EquipmentRegistrationTS | null;
}

/** A gear set with its items — the aggregate that populates the Gear registration page in one call. */
export interface GearSetWithItemsTS {
  set: GearSetTS;
  items: GearItemWithRegistrationTS[];
}

export interface NewEquipmentRegistrationPayload {
  equipmentid: number;
  tournamentid: string;
  roomid?: string | null;
  status: string;
}

export interface EquipmentRegistrationChangeset {
  status?: string;
  roomid?: string;
}

export const EquipmentRegistrationAPI = {

  getByTournament: async (tid: string, page: number, size: number): Promise<EquipmentRegistrationTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/equipmentregistrations?page=${page}&page_size=${size}`);
    if (!res.ok) throw new Error(`Failed to load registrations (${res.status})`);
    return res.json();
  },

  update: async (id: number, changeset: EquipmentRegistrationChangeset): Promise<EquipmentRegistrationTS> => {
    const res = await fetch(`/api/equipmentregistrations/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to update registration (${res.status}): ${text}`);
    }
    const result = await res.json();
    return result.data ?? result;
  },

  // Fetch registrations for a specific set of equipment IDs, filtered to one tournament.
  // Queries per-item so results are accurate regardless of total tournament registration count.
  getForEquipmentInTournament: async (
    equipmentIds: number[],
    tid: string,
  ): Promise<EquipmentRegistrationTS[]> => {
    if (equipmentIds.length === 0) return [];
    const perItem = await Promise.all(
      equipmentIds.map(async (id) => {
        const res = await fetch(`/api/equipment/${id}/equipmentregistrations?page=0&page_size=100`);
        if (!res.ok) return [] as EquipmentRegistrationTS[];
        const data = await res.json();
        const regs: EquipmentRegistrationTS[] = Array.isArray(data) ? data : (data.items ?? []);
        return regs.filter(r => r.tournamentid === tid);
      }),
    );
    return perItem.flat();
  },

  // One call that returns the logged-in user's gear sets, each item's detail, and its registration
  // status for this tournament — fully populating the Gear registration page.
  getMyGearRegistration: async (tid: string, accessToken?: string): Promise<GearSetWithItemsTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/my-gear-registration`, {
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!res.ok) throw new Error(`Failed to load your gear (${res.status})`);
    return res.json();
  },

  // Register several pieces of gear for a tournament in one call. Already-registered pieces are
  // skipped server-side. Returns the registrations that were created.
  registerMany: async (tid: string, equipmentIds: number[], accessToken?: string, status?: string): Promise<EquipmentRegistrationTS[]> => {
    const res = await fetch(`/api/tournaments/${tid}/gear-registrations`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
      body: JSON.stringify({ equipment_ids: equipmentIds, status }),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to register gear (${res.status}): ${text}`);
    }
    return res.json();
  },

  create: async (payload: NewEquipmentRegistrationPayload): Promise<EquipmentRegistrationTS> => {
    const res = await fetch('/api/equipmentregistrations', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to register equipment (${res.status}): ${text}`);
    }
    const result = await res.json();
    // POST returns EntityResponse<EquipmentRegistration>
    return result.data ?? result;
  },

  delete: async (id: number): Promise<void> => {
    const res = await fetch(`/api/equipmentregistrations/${id}`, { method: 'DELETE' });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to unregister equipment (${res.status}): ${text}`);
    }
  },
};
