// ── Permission names ─────────────────────────────────────────────────────────
//
// The canonical permission strings, matching the rows the backend seeds (see
// `models/permission.rs` — `AppResource` × `AppAction` plus `AppSpecialPermission`). The UI gates
// on these and never on a role name: roles only decide which permissions a user is granted, so
// moving a capability between roles is a data change, not a code change.

/** CRUD permissions, as `"{resource}:{action}"`. */
export const PERMISSION = {
  TOURNAMENT_CREATE: 'tournament:create',
  TOURNAMENT_READ: 'tournament:read',
  TOURNAMENT_UPDATE: 'tournament:update',
  TOURNAMENT_DELETE: 'tournament:delete',

  TOURNAMENTADMIN_CREATE: 'tournamentadmin:create',
  TOURNAMENTADMIN_READ: 'tournamentadmin:read',
  TOURNAMENTADMIN_UPDATE: 'tournamentadmin:update',
  TOURNAMENTADMIN_DELETE: 'tournamentadmin:delete',

  TOURNAMENTGROUP_CREATE: 'tournamentgroup:create',
  TOURNAMENTGROUP_READ: 'tournamentgroup:read',
  TOURNAMENTGROUP_UPDATE: 'tournamentgroup:update',
  TOURNAMENTGROUP_DELETE: 'tournamentgroup:delete',

  DIVISION_CREATE: 'division:create',
  DIVISION_READ: 'division:read',
  DIVISION_UPDATE: 'division:update',
  DIVISION_DELETE: 'division:delete',

  ROUNDGROUP_CREATE: 'roundgroup:create',
  ROUNDGROUP_READ: 'roundgroup:read',
  ROUNDGROUP_UPDATE: 'roundgroup:update',
  ROUNDGROUP_DELETE: 'roundgroup:delete',

  POOLBRACKETGROUP_CREATE: 'poolbracketgroup:create',
  POOLBRACKETGROUP_READ: 'poolbracketgroup:read',
  POOLBRACKETGROUP_UPDATE: 'poolbracketgroup:update',
  POOLBRACKETGROUP_DELETE: 'poolbracketgroup:delete',

  POOLBRACKET_CREATE: 'poolbracket:create',
  POOLBRACKET_READ: 'poolbracket:read',
  POOLBRACKET_UPDATE: 'poolbracket:update',
  POOLBRACKET_DELETE: 'poolbracket:delete',

  ROUND_CREATE: 'round:create',
  ROUND_READ: 'round:read',
  ROUND_UPDATE: 'round:update',
  ROUND_DELETE: 'round:delete',

  ROOMGROUP_CREATE: 'roomgroup:create',
  ROOMGROUP_READ: 'roomgroup:read',
  ROOMGROUP_UPDATE: 'roomgroup:update',
  ROOMGROUP_DELETE: 'roomgroup:delete',

  ROOM_CREATE: 'room:create',
  ROOM_READ: 'room:read',
  ROOM_UPDATE: 'room:update',
  ROOM_DELETE: 'room:delete',

  ROOMMONITOR_READ: 'roommonitor:read',

  GAME_CREATE: 'game:create',
  GAME_READ: 'game:read',
  GAME_UPDATE: 'game:update',
  GAME_DELETE: 'game:delete',

  TEAM_CREATE: 'team:create',
  TEAM_READ: 'team:read',
  TEAM_UPDATE: 'team:update',
  TEAM_DELETE: 'team:delete',

  EQUIPMENT_CREATE: 'equipment:create',
  EQUIPMENT_READ: 'equipment:read',
  EQUIPMENT_UPDATE: 'equipment:update',
  EQUIPMENT_DELETE: 'equipment:delete',

  STATSGROUP_CREATE: 'statsgroup:create',
  STATSGROUP_READ: 'statsgroup:read',
  STATSGROUP_UPDATE: 'statsgroup:update',
  STATSGROUP_DELETE: 'statsgroup:delete',

  USER_CREATE: 'user:create',
  USER_READ: 'user:read',
  USER_UPDATE: 'user:update',
  USER_DELETE: 'user:delete',

  // ── Permissions that are not a plain CRUD action ──
  /** Unrestricted access. Satisfies every other permission check (see `Session.can`). */
  SYSTEM_ADMINISTER: 'system:administer',
  /** The /dev page and the developer tooling it links to. */
  SYSTEM_ACCESS_DEV_TOOLS: 'system:access_dev_tools',
  /** The Superuser Console (/su-console); everyone without it gets the 404 page there. */
  SYSTEM_ACCESS_SUPERUSER_CONSOLE: 'system:access_superuser_console',
  /** A tournament's pairing code; the backend strips it from the payload without this. */
  TOURNAMENT_READ_PAIRING_CODE: 'tournament:read_pairing_code',
  /** "Created" / "Last Modified" audit columns on a tournament's entity tables. */
  TOURNAMENT_READ_AUDIT: 'tournament:read_audit',
  /** The schedule editor, rather than the read-only schedule view. */
  TOURNAMENT_UPDATE_SCHEDULE: 'tournament:update_schedule',
  /** Any user's account, not just one's own. */
  USER_READ_ANY: 'user:read_any',
  /** The users participating in tournaments one manages. */
  USER_READ_TOURNAMENT_PARTICIPANTS: 'user:read_tournament_participants',
} as const

export type PermissionName = (typeof PERMISSION)[keyof typeof PERMISSION]
