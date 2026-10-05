// ── Permission names ─────────────────────────────────────────────────────────
//
// The canonical permission strings, matching the rows the backend seeds (see
// `models/permission.rs` — `AppResource` × `AppAction` plus `AppSpecialPermission`). The UI gates
// on these and never on a role name: roles only decide which permissions a user is granted, so
// moving a capability between roles is a data change, not a code change.

/** CRUD permissions, as `"{resource}:{action}"`. */
export const PERMISSION = {
  TOURNAMENT_CREATE: 'tournament:create',
  TOURNAMENT_UPDATE: 'tournament:update',
  TOURNAMENT_DELETE: 'tournament:delete',
  TOURNAMENTGROUP_CREATE: 'tournamentgroup:create',
  TOURNAMENTGROUP_DELETE: 'tournamentgroup:delete',
  TEAM_UPDATE: 'team:update',
  USER_CREATE: 'user:create',
  USER_DELETE: 'user:delete',

  // ── Permissions that are not a plain CRUD action ──
  /** Unrestricted access. Satisfies every other permission check (see `Session.can`). */
  SYSTEM_ADMINISTER: 'system:administer',
  /** The /dev page and the developer tooling it links to. */
  SYSTEM_ACCESS_DEV_TOOLS: 'system:access_dev_tools',
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
