import { useEffect, useState } from 'react';
import { DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { RoomAPI, type RoomMonitorRowTS } from '../features/RoomAPI';
import { GameAPI } from '../features/GameAPI';
import { useAuth } from '../hooks/useAuth';
import { useLiveFeed } from '../hooks/useLiveFeed';
import { LiveIndicator } from './LiveIndicator';
import { isLate, statusMessages, resendButtonStyle, resendButtonDisabledStyle } from '../utils/roomMonitorStatus';
import { QuarantineButton, QuarantineDialog, ClearStatusChip, ClearPendingButton, ClearPendingDialog } from './RoomQueueDialogs';

// The monitor is pushed live by the server (see useLiveFeed). This is how often it re-reads the
// backend instead while the live connection is unavailable.
const POLL_MS = 30_000;

// How often the table re-renders between updates, so time-based highlighting ("late") stays current.
const TICK_MS = 5_000;

// The Resend column's note for each server-decided resend state (see `resend_status` in the backend).
const RESEND_NOTICES: Record<NonNullable<RoomMonitorRowTS['resend_status']>, string> = {
  requested: 'Requested — awaiting next ping',
  received: 'Resend request received by room.',
  failed: 'Room failed to resend game events. Please issue another resend request.',
};

function formatDateTime(iso: string | null): string {
  if (!iso) return '—';
  const d = new Date(iso);
  return isNaN(d.getTime()) ? iso : d.toLocaleString();
}

function orDash(v: string | number | null): string | number {
  return v === null || v === undefined || v === '' ? '—' : v;
}

// Whether the row should be highlighted (red background / white text).
function isAlert(r: RoomMonitorRowTS, now: number): boolean {
  return isLate(r, now) || r.data_incomplete;
}

// canClear: whether the user may start/cancel a Clear Pending + Resend (game:update for this tournament).
export default function RoomMonitorTable({ tid, canClear = false }: { tid: string; canClear?: boolean }) {
  const { accessToken, refresh } = useAuth();
  // Live rows. The stream needs a token, so it opens once one is available, and reopens with each
  // refreshed token; the server ends it when the token it opened with expires.
  const { data, status, error } = useLiveFeed<RoomMonitorRowTS[]>({
    url: accessToken ? `/api/tournaments/${tid}/roommonitor/stream` : null,
    token: accessToken,
    poll: () => RoomAPI.getMonitorByTournament(tid, accessToken),
    pollIntervalMs: POLL_MS,
    onExpired: refresh,
  });
  // Guard against a non-array payload so a bad/error response can't crash the render.
  const rows = Array.isArray(data) ? data : [];
  const loading = data === null && status !== 'error';
  // A ticking value so the "late" evaluation re-renders even between updates.
  const [, setTick] = useState(0);
  // Games whose resend request is in flight. Everything after that (requested, received, failed)
  // is decided by the server and arrives with the rows.
  const [requesting, setRequesting] = useState<Set<string>>(new Set());
  // The room whose quarantine dialog is open.
  const [quarantineRoom, setQuarantineRoom] = useState<RoomMonitorRowTS | null>(null);
  // The room whose Clear Pending + Resend dialog is open, and that dialog's request state.
  const [clearRoom, setClearRoom] = useState<RoomMonitorRowTS | null>(null);
  const [clearBusy, setClearBusy] = useState(false);
  const [clearError, setClearError] = useState<string | null>(null);

  useEffect(() => {
    // The stream is auth-gated (owner/admin/super user). On a fresh page load the access token is
    // repopulated asynchronously; rather than wait indefinitely, actively seed it from the
    // refresh cookie. When it lands, the live feed opens.
    if (!accessToken) {
      console.log('Room Monitor: no access token yet — seeding it from the refresh session.');
      refresh();
    }
  }, [accessToken, refresh]);

  useEffect(() => {
    const tick = setInterval(() => setTick((t) => t + 1), TICK_MS);
    return () => clearInterval(tick);
  }, []);

  // requesting is keyed by game id, since a room can now have several rows (its current game
  // plus other games of that room whose data has gaps).
  const handleResend = async (r: RoomMonitorRowTS) => {
    if (!r.game_id) return;
    const gid = r.game_id;
    setRequesting((prev) => new Set(prev).add(gid));
    try {
      console.log(`Resend update request sent to server for game ${gid} (room ${r.room ?? r.roomid}) at ${new Date().toLocaleString()}.`);
      // The server pushes the game's new "requested" state to every open monitor.
      await GameAPI.requestResend(gid, accessToken);
    } catch {
      alert('Failed to request resend.');
    } finally {
      setRequesting((prev) => {
        const next = new Set(prev);
        next.delete(gid);
        return next;
      });
    }
  };

  const handleStartClear = async () => {
    if (!clearRoom) return;
    setClearBusy(true);
    setClearError(null);
    try {
      // The clear's new state arrives with the next live update.
      await RoomAPI.startClearPending(clearRoom.roomid, accessToken);
      setClearRoom(null);
    } catch (e) {
      setClearError((e as Error).message);
    } finally {
      setClearBusy(false);
    }
  };

  const handleCancelClear = async (r: RoomMonitorRowTS) => {
    if (!window.confirm(`Stop clearing ${r.room ?? 'this room'}? Anything already set aside will still be asked for.`)) return;
    try {
      await RoomAPI.cancelClearPending(r.roomid, accessToken);
    } catch (e) {
      alert((e as Error).message);
    }
  };

  // Pending count, plus — on the room's main row — the clear's progress or, when the room looks
  // stuck, the button that starts one.
  const renderPending = (r: RoomMonitorRowTS) => {
    if (!r.is_room_row) return orDash(r.pending);
    let control: React.ReactNode = null;
    if (r.clear_status) {
      control = (
        <>
          <ClearStatusChip status={r.clear_status} setAside={r.clear_cleared_count} />
          {r.clear_status === 'clearing' && canClear && (
            <button style={resendButtonStyle} onClick={() => handleCancelClear(r)}>Cancel</button>
          )}
        </>
      );
    }
    // A finished clear's outcome is shown for a while; if the room is stuck again, offer another.
    const active = r.clear_status === 'clearing' || r.clear_status === 'resending';
    if (r.pending_stalled && !active) {
      control = (
        <>
          {control}
          {canClear
            ? <ClearPendingButton onClick={() => { setClearError(null); setClearRoom(r); }} />
            : <span style={{ fontSize: 12, fontWeight: 600 }} title="Pending has not fallen for 90 seconds">Stuck</span>}
        </>
      );
    }
    return (
      <div style={{ display: 'flex', flexDirection: 'column', gap: 4, alignItems: 'center' }}>
        <span>{orDash(r.pending)}</span>
        {control}
      </div>
    );
  };

  const columns: ColumnDef<RoomMonitorRowTS>[] = [
    { header: 'Check In',     render: (r) => formatDateTime(r.check_in) },
    { header: 'Room',         render: (r) => orDash(r.room)             },
    { header: 'Round',        render: (r) => orDash(r.round)            },
    { header: 'Question',     render: (r) => orDash(r.question)         },
    { header: 'QMVersion',    render: (r) => orDash(r.qm_version)       },
    { header: 'Pending',      render: renderPending                     },
    {
      header: 'Quarantine',
      render: (r) => r.is_room_row
        ? <QuarantineButton count={r.quarantine_count} onClick={() => setQuarantineRoom(r)} />
        : null,
    },
    {
      header: 'Status-Error',
      render: (r) => statusMessages(r, Date.now()).map((m, i) => <div key={i}>{m}</div>),
    },
    {
      header: 'Resend',
      render: (r) => {
        const gid = r.game_id;
        const requestingThis = !!gid && requesting.has(gid);
        // Only actionable when the round data is incomplete (a resend is warranted).
        const disabled = !gid || requestingThis || !r.data_incomplete;
        const title = !gid
          ? 'No game associated with this room yet'
          : !r.data_incomplete
            ? 'Round data is complete — no resend needed'
            : 'Resend all events on the next ping';

        const notice = r.resend_status ? RESEND_NOTICES[r.resend_status] : null;

        return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 4, alignItems: 'center' }}>
          <button
            style={disabled ? resendButtonDisabledStyle : resendButtonStyle}
            disabled={disabled}
            onClick={() => handleResend(r)}
            title={title}
          >
            {requestingThis ? '…' : 'Resend'}
          </button>
          {notice && <span style={{ fontSize: 12 }}>{notice}</span>}
        </div>
        );
      },
    },
  ];

  // Show the rows of any room that has checked in. Only the current-game row carries a
  // check-in timestamp; a room's extra (non-current) resend rows have none of their own, so
  // keep them by matching on roomid. Ordered by room name (then round, then game id) so a
  // room's several rows stay grouped and stably ordered.
  const checkedInRoomIds = new Set(rows.filter((r) => r.check_in).map((r) => r.roomid));
  const visibleRows = rows
    .filter((r) => checkedInRoomIds.has(r.roomid))
    .sort(
      (a, b) =>
        (a.room ?? '').localeCompare(b.room ?? '', undefined, { numeric: true }) ||
        (a.round ?? '').localeCompare(b.round ?? '', undefined, { numeric: true }) ||
        (a.game_id ?? '').localeCompare(b.game_id ?? '')
    );

  return (
    <>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'flex-end', gap: 8, marginBottom: 8 }}>
        {error && status === 'error'
          ? <span style={{ fontSize: 12, color: '#c0392b' }}>{error}</span>
          : <LiveIndicator status={status} pollIntervalMs={POLL_MS} />}
      </div>
      <DataTableTemplate<RoomMonitorRowTS>
        loading={loading}
        entityLabel="Room Monitor"
        showCreateButton={false}
        showDeleteButton={false}
        dense
        columns={columns}
        rows={visibleRows}
        totalCount={visibleRows.length}
        getId={(r) => `${r.roomid}:${r.game_id ?? ''}`}
        page={0}
        pageSize={visibleRows.length || 1}
        onPageChange={() => { }}
        onPageSizeChange={() => { }}
        onDelete={async () => { }}
        getRowStyle={(r) => (isAlert(r, Date.now()) ? { background: '#c0392b', color: '#ffffff' } : undefined)}
      />
      {quarantineRoom && (
        <QuarantineDialog
          roomid={quarantineRoom.roomid}
          roomName={quarantineRoom.room ?? 'Room'}
          isOpen
          onClose={() => setQuarantineRoom(null)}
        />
      )}
      <ClearPendingDialog
        roomName={clearRoom?.room ?? 'Room'}
        pending={clearRoom?.pending ?? null}
        isOpen={!!clearRoom}
        busy={clearBusy}
        error={clearError}
        onCancel={() => setClearRoom(null)}
        onConfirm={handleStartClear}
      />
    </>
  );
}
