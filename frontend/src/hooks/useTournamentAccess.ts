import { useState, useEffect } from 'react';
import { PERMISSION } from '../auth/permissions'
import { useAuth } from './useAuth';
import { AdminAPI } from '../features/AdminAPI';

/**
 * The single source of truth for a tournament's access flags. Every profile that renders
 * a tournament's entity data tables (Tournament, Division, Round, Room, Team) uses this so
 * the same column/button permissions apply no matter which profile the table appears in.
 *
 * Mirrors the logic originally defined inline in TournamentProfile.
 */
export interface TournamentAccess {
  /** Holder of `system:administer`, or the tournament's owner. */
  isOwnerOrSystemAdmin: boolean;
  /** `system:administer`, the owner, or one of this tournament's designated admins (via API). */
  canViewAdmins: boolean;
  /** Holder of `tournament:read_pairing_code`. */
  canViewPairingCode: boolean;
  /** Who may see "Created" / "Last Modified" audit columns on entity tables. */
  canViewAuditColumns: boolean;
  /** True when the user may act as owner/admin AND holds the given resource permission. */
  canCreate: (permission: string) => boolean;
}

export function useTournamentAccess(
  tid: string | undefined,
  ownerId: string | undefined,
): TournamentAccess {
  const { session } = useAuth();
  const [canViewAdmins, setCanViewAdmins] = useState<boolean>(false);

  useEffect(() => {
    if (!tid || !ownerId || !session) { setCanViewAdmins(false); return; }
    if (session.isSystemAdministrator || session.userId === ownerId) {
      setCanViewAdmins(true);
      return;
    }
    let cancelled = false;
    setCanViewAdmins(false);
    AdminAPI.getByTournament(tid, 0, 500)
      .then(admins => { if (!cancelled) setCanViewAdmins(admins.some(a => a.id === session.userId)); })
      .catch(() => { if (!cancelled) setCanViewAdmins(false); });
    return () => { cancelled = true; };
  }, [tid, ownerId, session?.userId]);

  const isOwnerOrSystemAdmin =
    (session?.isSystemAdministrator ?? false) ||
    (session?.userId === ownerId);

  const canViewPairingCode =
    session?.can(PERMISSION.TOURNAMENT_READ_PAIRING_CODE) ?? false;

  const canViewAuditColumns =
    canViewAdmins ||
    (session?.can(PERMISSION.TOURNAMENT_READ_AUDIT) ?? false);

  const canCreate = (permission: string): boolean =>
    canViewAdmins && (session?.can(permission) ?? false);

  return { isOwnerOrSystemAdmin, canViewAdmins, canViewPairingCode, canViewAuditColumns, canCreate };
}
