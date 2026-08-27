import type { CSSProperties } from 'react';
import type { RoomMonitorRowTS } from '../features/RoomAPI';

// A room in-progress whose client-reported timestamp is older than this is "late",
// and a room whose last check-in is older than this is treated as no longer reporting.
export const STALE_MS = 2 * 60 * 1000;

// A room is "not communicating" when a game is in progress but its client-reported
// timestamp (ping_client_ts) is more than STALE_MS old.
export function isLate(r: RoomMonitorRowTS, now: number): boolean {
  if (!r.game_in_progress || !r.client_ts) return false;
  const t = new Date(r.client_ts).getTime();
  return !isNaN(t) && now - t > STALE_MS;
}

// A room is "still reporting" via /api/pingmsg when its last check-in (ping_last_checkin_ts)
// is recent (within STALE_MS).
export function isReporting(r: RoomMonitorRowTS | undefined, now: number): boolean {
  if (!r?.check_in) return false;
  const t = new Date(r.check_in).getTime();
  return !isNaN(t) && now - t <= STALE_MS;
}

// All applicable Status-Error messages for a row (both shown when both apply).
export function statusMessages(r: RoomMonitorRowTS, now: number): string[] {
  const messages: string[] = [];
  if (isLate(r, now)) messages.push('Room not communicating?? Late??');
  if (r.data_incomplete) messages.push('Data Incomplete. Resend Advised.');
  return messages;
}

export const resendButtonStyle: CSSProperties = {
  padding: '2px 10px',
  fontSize: '0.75rem',
  fontWeight: 600,
  cursor: 'pointer',
  borderRadius: 4,
  border: '1px solid #d1d5db',
  background: '#ffffff',
  color: '#374151',
};

export const resendButtonDisabledStyle: CSSProperties = {
  ...resendButtonStyle,
  cursor: 'not-allowed',
  background: '#f3f4f6',
  color: '#9ca3af',
  // Use the `border` shorthand (not `borderColor`) to match the base style — mixing the two
  // makes React remove `borderColor` on toggle, which logs a styling-bug warning.
  border: '1px solid #e5e7eb',
};
