import { useState, useEffect } from 'react';
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
  /** Superuser or the tournament's owner. */
  isOwnerOrSuperUser: boolean;
  /** Superuser, owner, or one of this tournament's designated admins (resolved via API). */
  canViewAdmins: boolean;
  /** Superuser, tournament_manager, or tournament_admin (role-based). */
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
    if (session.hasRole('super_user') || session.userId === ownerId) {
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

  const isOwnerOrSuperUser =
    (session?.hasRole('super_user') ?? false) ||
    (session?.userId === ownerId);

  const canViewPairingCode =
    (session?.hasRole('super_user') ?? false) ||
    (session?.hasRole('tournament_manager') ?? false) ||
    (session?.hasRole('tournament_admin') ?? false);

  const canViewAuditColumns =
    canViewAdmins ||
    (session?.hasRole('tournament_manager') ?? false) ||
    (session?.hasRole('tournament_admin') ?? false);

  const canCreate = (permission: string): boolean =>
    canViewAdmins && (session?.hasPermission(permission) ?? false);

  return { isOwnerOrSuperUser, canViewAdmins, canViewPairingCode, canViewAuditColumns, canCreate };
}
