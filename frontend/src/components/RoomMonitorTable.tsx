import { useEffect, useState } from 'react';
import { DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { RoomAPI, type RoomMonitorRowTS } from '../features/RoomAPI';
import { GameAPI } from '../features/GameAPI';

const POLL_MS = 60_000;                 // refresh the monitor every 60 seconds
const STALE_MS = 2 * 60 * 1000;         // a room in-progress whose client_ts is > 2 min old is "late"

function formatDateTime(iso: string | null): string {
  if (!iso) return '—';
  const d = new Date(iso);
  return isNaN(d.getTime()) ? iso : d.toLocaleString();
}

function orDash(v: string | number | null): string | number {
  return v === null || v === undefined || v === '' ? '—' : v;
}

// A room is "not communicating" when a game is in progress but its client-reported
// timestamp (ping_client_ts) is more than STALE_MS old.
function isLate(r: RoomMonitorRowTS, now: number): boolean {
  if (!r.game_in_progress || !r.client_ts) return false;
  const t = new Date(r.client_ts).getTime();
  return !isNaN(t) && now - t > STALE_MS;
}

// All applicable Status-Error messages for a row (both shown when both apply).
function statusMessages(r: RoomMonitorRowTS, now: number): string[] {
  const messages: string[] = [];
  if (isLate(r, now)) messages.push('Room not communicating?? Late??');
  if (r.data_incomplete) messages.push('Data Incomplete. Resend Advised.');
  return messages;
}

// Whether the row should be highlighted (red background / white text).
function isAlert(r: RoomMonitorRowTS, now: number): boolean {
  return isLate(r, now) || r.data_incomplete;
}

const resendButtonStyle: React.CSSProperties = {
  padding: '2px 10px',
  fontSize: '0.75rem',
  fontWeight: 600,
  cursor: 'pointer',
  borderRadius: 4,
  border: '1px solid #d1d5db',
  background: '#ffffff',
  color: '#374151',
};

const resendButtonDisabledStyle: React.CSSProperties = {
  ...resendButtonStyle,
  cursor: 'not-allowed',
  background: '#f3f4f6',
  color: '#9ca3af',
  borderColor: '#e5e7eb',
};

export default function RoomMonitorTable({ tid }: { tid: string }) {
  const [rows, setRows] = useState<RoomMonitorRowTS[]>([]);
  // A ticking value so the "late" evaluation re-renders even between polls.
  const [, setTick] = useState(0);
  // Rooms whose resend request is in flight, and those that have been requested (awaiting the next ping).
  const [requesting, setRequesting] = useState<Set<string>>(new Set());
  const [requested, setRequested] = useState<Set<string>>(new Set());

  useEffect(() => {
    let cancelled = false;
    const load = () => {
      RoomAPI.getMonitorByTournament(tid)
        .then((result) => {
          if (cancelled) return;
          setRows(result);
          // Clear the transient "requested" notes on each refresh.
          setRequested(new Set());
        })
        .catch(() => console.error('Failed to load room monitor data'));
    };
    load();
    const poll = setInterval(load, POLL_MS);
    const tick = setInterval(() => setTick((t) => t + 1), POLL_MS);
    return () => {
      cancelled = true;
      clearInterval(poll);
      clearInterval(tick);
    };
  }, [tid]);

  const handleResend = async (r: RoomMonitorRowTS) => {
    if (!r.game_id) return;
    setRequesting((prev) => new Set(prev).add(r.roomid));
    try {
      await GameAPI.requestResend(r.game_id);
      setRequested((prev) => new Set(prev).add(r.roomid));
    } catch {
      alert('Failed to request resend.');
    } finally {
      setRequesting((prev) => {
        const next = new Set(prev);
        next.delete(r.roomid);
        return next;
      });
    }
  };

  const columns: ColumnDef<RoomMonitorRowTS>[] = [
    { header: 'Check In',     render: (r) => formatDateTime(r.check_in) },
    { header: 'Room',         render: (r) => orDash(r.room)             },
    { header: 'Round',        render: (r) => orDash(r.round)            },
    { header: 'Question',     render: (r) => orDash(r.question)         },
    { header: 'Host IP',      render: (r) => orDash(r.host_ip)          },
    { header: 'QMVersion',    render: (r) => orDash(r.qm_version)       },
    { header: 'Pending',      render: (r) => orDash(r.pending)          },
    {
      header: 'Status-Error',
      render: (r) => statusMessages(r, Date.now()).map((m, i) => <div key={i}>{m}</div>),
    },
    {
      header: 'Resend',
      render: (r) => {
        // Only actionable when the round data is incomplete (a resend is warranted).
        const disabled = !r.game_id || requesting.has(r.roomid) || !r.data_incomplete;
        const title = !r.game_id
          ? 'No game associated with this room yet'
          : !r.data_incomplete
            ? 'Round data is complete — no resend needed'
            : 'Resend all events on the next ping';
        return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 4, alignItems: 'center' }}>
          <button
            style={disabled ? resendButtonDisabledStyle : resendButtonStyle}
            disabled={disabled}
            onClick={() => handleResend(r)}
            title={title}
          >
            {requesting.has(r.roomid) ? '…' : 'Resend'}
          </button>
          {requested.has(r.roomid) && (
            <span style={{ fontSize: 12 }}>Requested — awaiting next ping</span>
          )}
          {/* The resend response is only relevant while data is still incomplete. */}
          {r.data_incomplete && r.resend && <span style={{ fontSize: 12 }}>Response: {r.resend}</span>}
        </div>
        );
      },
    },
  ];

  // Only show rooms that have historically checked in, ordered by room name.
  const visibleRows = rows
    .filter((r) => r.check_in)
    .sort((a, b) => (a.room ?? '').localeCompare(b.room ?? '', undefined, { numeric: true }));

  return (
    <DataTableTemplate<RoomMonitorRowTS>
      entityLabel="Room Monitor"
      showCreateButton={false}
      showDeleteButton={false}
      dense
      columns={columns}
      rows={visibleRows}
      totalCount={visibleRows.length}
      getId={(r) => r.roomid}
      page={0}
      pageSize={visibleRows.length || 1}
      onPageChange={() => { }}
      onPageSizeChange={() => { }}
      onDelete={async () => { }}
      getRowStyle={(r) => (isAlert(r, Date.now()) ? { background: '#c0392b', color: '#ffffff' } : undefined)}
    />
  );
}
