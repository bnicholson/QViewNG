export interface EquipmentRegistrationTS {
  id: number;
  equipmentid: number;
  tournamentid: string;
  roomid: string | null;
  status: string;
  created_at: string;
  updated_at: string;
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
