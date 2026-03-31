// ── GearSet (EquipmentSet) ───────────────────────────────────────────────────

export interface GearSetTS {
  id: number;
  equipmentownerid: string;
  is_active: boolean;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewGearSetPayload {
  equipmentownerid: string;
  is_active: boolean;
  name: string;
  description: string | null;
}

export interface GearSetChangeset {
  name?: string;
  description?: string | null;
  is_active?: boolean;
}

// ── Equipment DBO (bridge table) ─────────────────────────────────────────────

export interface EquipmentDboTS {
  id: number;
  computerid: number | null;
  jumppadid: number | null;
  interfaceboxid: number | null;
  monitorid: number | null;
  microphonerecorderid: number | null;
  projectorid: number | null;
  powerstripid: number | null;
  extensioncordid: number | null;
  misc_note: string | null;
  equipmentsetid: number;
  created_at: string;
  updated_at: string;
}

// ── Gear types ───────────────────────────────────────────────────────────────

export type GearType =
  | 'Computer'
  | 'JumpPad'
  | 'InterfaceBox'
  | 'Monitor'
  | 'MicrophoneRecorder'
  | 'Projector'
  | 'PowerStrip'
  | 'ExtensionCord';

export const GEAR_TYPE_LABELS: Record<GearType, string> = {
  Computer: 'Computer',
  JumpPad: 'Jump Pad',
  InterfaceBox: 'Interface Box',
  Monitor: 'Monitor',
  MicrophoneRecorder: 'Microphone / Recorder',
  Projector: 'Projector',
  PowerStrip: 'Power Strip',
  ExtensionCord: 'Extension Cord',
};

export const GEAR_TYPE_ENDPOINTS: Record<GearType, string> = {
  Computer: 'computers',
  JumpPad: 'jumppads',
  InterfaceBox: 'interfaceboxes',
  Monitor: 'monitors',
  MicrophoneRecorder: 'microphonerecorders',
  Projector: 'projectors',
  PowerStrip: 'powerstrips',
  ExtensionCord: 'extensioncords',
};

export function detectGearType(dbo: EquipmentDboTS): GearType | null {
  if (dbo.computerid !== null) return 'Computer';
  if (dbo.jumppadid !== null) return 'JumpPad';
  if (dbo.interfaceboxid !== null) return 'InterfaceBox';
  if (dbo.monitorid !== null) return 'Monitor';
  if (dbo.microphonerecorderid !== null) return 'MicrophoneRecorder';
  if (dbo.projectorid !== null) return 'Projector';
  if (dbo.powerstripid !== null) return 'PowerStrip';
  if (dbo.extensioncordid !== null) return 'ExtensionCord';
  return null;
}

// ── Full typed equipment models ───────────────────────────────────────────────

export interface ComputerTS {
  equipmentid: number;
  computerid: number;
  equipmentsetid: number;
  brand: string;
  operating_system: string;
  quizmachine_version: string;
  wifi_capabilities: string;
  login_username: string;
  login_password: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  has_usb_port: boolean;
  misc_note: string | null;
  clientkey: string;
  created_at: string;
  updated_at: string;
}

export interface JumpPadTS {
  equipmentid: number;
  jumppadid: number;
  equipmentsetid: number;
  color: string;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface InterfaceBoxTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  type_: string;
  serial_number: string | null;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface MonitorTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  size: string;
  brand: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface MicrophoneRecorderTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  type_: string; // 'External' | 'Built-In'
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface ProjectorTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  brand: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface PowerStripTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  make: string;
  model: string;
  color: string;
  num_of_plugs: number;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

export interface ExtensionCordTS {
  equipmentid: number;
  id: number;
  equipmentsetid: number;
  color: string;
  length: string;
  misc_note: string | null;
  created_at: string;
  updated_at: string;
}

// Tagged-union returned by GET /api/equipment/{id}
export type EquipmentDetail =
  | { Computer: ComputerTS }
  | { JumpPad: JumpPadTS }
  | { InterfaceBox: InterfaceBoxTS }
  | { Monitor: MonitorTS }
  | { MicrophoneRecorder: MicrophoneRecorderTS }
  | { Projector: ProjectorTS }
  | { PowerStrip: PowerStripTS }
  | { ExtensionCord: ExtensionCordTS };

export function extractDetailType(detail: EquipmentDetail): GearType {
  return Object.keys(detail)[0] as GearType;
}

export function extractDetailData(detail: EquipmentDetail): any {
  return Object.values(detail)[0];
}

export function getGearDescription(detail: EquipmentDetail): string {
  if ('Computer' in detail) {
    const c = detail.Computer;
    return [c.brand, c.operating_system].filter(Boolean).join(' · ') || 'Computer';
  }
  if ('JumpPad' in detail) return `Color: ${detail.JumpPad.color}`;
  if ('InterfaceBox' in detail) {
    const ib = detail.InterfaceBox;
    return ib.serial_number ? `${ib.type_} · S/N: ${ib.serial_number}` : ib.type_;
  }
  if ('Monitor' in detail) {
    const m = detail.Monitor;
    return `${m.size}" ${m.brand}`.trim() || 'Monitor';
  }
  if ('MicrophoneRecorder' in detail) return detail.MicrophoneRecorder.type_ || 'Mic/Recorder';
  if ('Projector' in detail) return detail.Projector.brand || 'Projector';
  if ('PowerStrip' in detail) {
    const ps = detail.PowerStrip;
    return `${ps.make} ${ps.model} (${ps.num_of_plugs} plugs)`.trim();
  }
  if ('ExtensionCord' in detail) {
    const ec = detail.ExtensionCord;
    return `${ec.color}, ${ec.length}`.trim();
  }
  return 'Unknown';
}

// ── New payload types ────────────────────────────────────────────────────────

export interface NewComputerPayload {
  equipmentsetid: number;
  brand: string;
  operating_system: string;
  quizmachine_version: string;
  wifi_capabilities: string;
  login_username: string;
  login_password: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  has_usb_port: boolean;
  misc_note: string | null;
  clientkey: string;
}

export interface NewJumpPadPayload {
  equipmentsetid: number;
  color: string;
  misc_note: string | null;
}

export interface NewInterfaceBoxPayload {
  equipmentsetid: number;
  type_: string;
  serial_number: string | null;
  misc_note: string | null;
}

export interface NewMonitorPayload {
  equipmentsetid: number;
  size: string;
  brand: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  misc_note: string | null;
}

export interface NewMicrophoneRecorderPayload {
  equipmentsetid: number;
  type_: string;
  misc_note: string | null;
}

export interface NewProjectorPayload {
  equipmentsetid: number;
  brand: string;
  has_vga_out_port: boolean;
  has_dvi_out_port: boolean;
  has_hdmi_out_port: boolean;
  has_display_port_out: boolean;
  misc_note: string | null;
}

export interface NewPowerStripPayload {
  equipmentsetid: number;
  make: string;
  model: string;
  color: string;
  num_of_plugs: number;
  misc_note: string | null;
}

export interface NewExtensionCordPayload {
  equipmentsetid: number;
  color: string;
  length: string;
  misc_note: string | null;
}

export type NewGearPayload =
  | NewComputerPayload
  | NewJumpPadPayload
  | NewInterfaceBoxPayload
  | NewMonitorPayload
  | NewMicrophoneRecorderPayload
  | NewProjectorPayload
  | NewPowerStripPayload
  | NewExtensionCordPayload;

// ── Changeset types ───────────────────────────────────────────────────────────

export interface ComputerChangeset {
  equipmentsetid?: number;
  brand?: string;
  operating_system?: string;
  quizmachine_version?: string;
  wifi_capabilities?: string;
  login_username?: string;
  login_password?: string;
  has_vga_out_port?: boolean;
  has_dvi_out_port?: boolean;
  has_hdmi_out_port?: boolean;
  has_display_port_out?: boolean;
  has_usb_port?: boolean;
  misc_note?: string | null;
  clientkey?: string;
}

export interface JumpPadChangeset {
  equipmentsetid?: number;
  color?: string;
  misc_note?: string | null;
}

// NOTE: type_ is required (non-optional) in the backend InterfaceBoxChangeSet
export interface InterfaceBoxChangeset {
  type_: string;
  serial_number?: string | null;
  equipmentsetid?: number;
  misc_note?: string | null;
}

export interface MonitorChangeset {
  equipmentsetid?: number;
  size?: string;
  brand?: string;
  has_vga_out_port?: boolean;
  has_dvi_out_port?: boolean;
  has_hdmi_out_port?: boolean;
  has_display_port_out?: boolean;
  misc_note?: string | null;
}

// NOTE: type_ is required (non-optional) in the backend MicrophoneRecorderChangeSet
export interface MicrophoneRecorderChangeset {
  type_: string;
  equipmentsetid?: number;
  misc_note?: string | null;
}

export interface ProjectorChangeset {
  equipmentsetid?: number;
  brand?: string;
  has_vga_out_port?: boolean;
  has_dvi_out_port?: boolean;
  has_hdmi_out_port?: boolean;
  has_display_port_out?: boolean;
  misc_note?: string | null;
}

export interface PowerStripChangeset {
  equipmentsetid?: number;
  make?: string;
  model?: string;
  color?: string;
  num_of_plugs?: number;
  misc_note?: string | null;
}

export interface ExtensionCordChangeset {
  equipmentsetid?: number;
  color?: string;
  length?: string;
  misc_note?: string | null;
}

// ── API ───────────────────────────────────────────────────────────────────────

export const EquipmentSetAPI = {
  // GearSet CRUD
  getByOwner: async (userId: string): Promise<GearSetTS[]> => {
    const res = await fetch(`/api/users/${userId}/equipmentsets`);
    if (!res.ok) throw new Error(`Failed to load gear sets (${res.status})`);
    return res.json();
  },

  getById: async (id: number): Promise<GearSetTS> => {
    const res = await fetch(`/api/equipmentsets/${id}`);
    if (!res.ok) throw new Error(`GearSet not found (${res.status})`);
    return res.json();
  },

  create: async (payload: NewGearSetPayload): Promise<GearSetTS> => {
    const res = await fetch('/api/equipmentsets', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to create gear set (${res.status}): ${text}`);
    }
    const result = await res.json();
    return result.data;
  },

  update: async (id: number, changeset: GearSetChangeset): Promise<GearSetTS> => {
    const res = await fetch(`/api/equipmentsets/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to update gear set (${res.status}): ${text}`);
    }
    const result = await res.json();
    return result.data;
  },

  delete: async (id: number): Promise<void> => {
    const res = await fetch(`/api/equipmentsets/${id}`, { method: 'DELETE' });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to delete gear set (${res.status}): ${text}`);
    }
  },

  // Equipment in a set
  getEquipmentInSet: async (setId: number): Promise<EquipmentDboTS[]> => {
    const res = await fetch(`/api/equipmentsets/${setId}/equipment`);
    if (!res.ok) throw new Error(`Failed to load equipment (${res.status})`);
    return res.json();
  },

  // Polymorphic equipment detail
  getEquipmentDetail: async (equipmentId: number): Promise<EquipmentDetail> => {
    const res = await fetch(`/api/equipment/${equipmentId}`);
    if (!res.ok) throw new Error(`Equipment not found (${res.status})`);
    return res.json();
  },

  // Create a gear item of a given type
  createGearItem: async (type: GearType, payload: NewGearPayload): Promise<any> => {
    const endpoint = GEAR_TYPE_ENDPOINTS[type];
    const res = await fetch(`/api/equipment/${endpoint}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to create ${GEAR_TYPE_LABELS[type]} (${res.status}): ${text}`);
    }
    const result = await res.json();
    return result.data;
  },

  // Update a gear item (equipment_id is the id from the equipment DBO table)
  updateGearItem: async (type: GearType, equipmentId: number, changeset: any): Promise<any> => {
    const endpoint = GEAR_TYPE_ENDPOINTS[type];
    const res = await fetch(`/api/equipment/${endpoint}/${equipmentId}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(changeset),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to update ${GEAR_TYPE_LABELS[type]} (${res.status}): ${text}`);
    }
    const result = await res.json();
    return result.data;
  },

  // Delete a gear item
  deleteGearItem: async (type: GearType, equipmentId: number): Promise<void> => {
    const endpoint = GEAR_TYPE_ENDPOINTS[type];
    const res = await fetch(`/api/equipment/${endpoint}/${equipmentId}`, { method: 'DELETE' });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`Failed to delete ${GEAR_TYPE_LABELS[type]} (${res.status}): ${text}`);
    }
  },
};
