import { useEffect, useRef, useState } from 'react';
import { DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { RoomAPI, type RoomMonitorRowTS } from '../features/RoomAPI';
import { GameAPI } from '../features/GameAPI';
import { useAuth } from '../hooks/useAuth';
import { useTheme } from '@mui/material';

const POLL_MS = 30_000;                 // refresh the monitor every 30 seconds
const STALE_MS = 2 * 60 * 1000;         // a room in-progress whose client_ts is > 2 min old is "late"

// A countdown "pie" that drains over `durationMs` and refills whenever `resetKey` changes
// (i.e. when a server response arrives). Runs its own animation frame loop so only this
// small SVG re-renders, not the whole table.
function CountdownPie({ resetKey, durationMs, size = 20 }: { resetKey: number; durationMs: number; size?: number }) {
  const [remaining, setRemaining] = useState(1); // fraction left, 1 → full, 0 → empty
  const startRef = useRef<number>(performance.now());
  const theme = useTheme();

  // Refill and restart the cycle each time a response comes back.
  useEffect(() => {
    startRef.current = performance.now();
    setRemaining(1);
  }, [resetKey]);

  // Smoothly drain the pie.
  useEffect(() => {
    let raf = 0;
    const tick = () => {
      const elapsed = performance.now() - startRef.current;
      setRemaining(Math.max(0, 1 - elapsed / durationMs));
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [durationMs]);

  const c = size / 2;
  const r = size / 2;
  // Wedge path for the remaining fraction. The empty portion grows clockwise from the top
  // (12 o'clock), so the filled wedge runs clockwise from its moving edge back up to the top.
  let wedge = '';
  if (remaining > 0 && remaining < 1) {
    const angle = remaining * 360;
    const rad = (deg: number) => (deg * Math.PI) / 180;
    const startDeg = -90 + (360 - angle); // moving edge — sweeps clockwise from the top
    const x1 = c + r * Math.cos(rad(startDeg));
    const y1 = c + r * Math.sin(rad(startDeg));
    const x2 = c + r * Math.cos(rad(-90)); // fixed edge at the top
    const y2 = c + r * Math.sin(rad(-90));
    const largeArc = angle > 180 ? 1 : 0;
    wedge = `M ${c} ${c} L ${x1} ${y1} A ${r} ${r} 0 ${largeArc} 1 ${x2} ${y2} Z`;
  }

  return (
    <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} aria-label="Time until next refresh">
      {/* Track */}
      <circle cx={c} cy={c} r={r} fill="#e5e7eb" />
      {/* Remaining time */}
      {remaining >= 1
        ? <circle cx={c} cy={c} r={r} fill={theme.palette.primary.main}/>  // "#2563eb" />
        : wedge && <path d={wedge} fill={theme.palette.primary.main}/>  // "#2563eb" />}
      }
    </svg>
  );
}

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
  // Use the `border` shorthand (not `borderColor`) to match the base style — mixing the two
  // makes React remove `borderColor` on toggle, which logs a styling-bug warning.
  border: '1px solid #e5e7eb',
};

export default function RoomMonitorTable({ tid }: { tid: string }) {
  const { accessToken, refresh } = useAuth();
  const [rows, setRows] = useState<RoomMonitorRowTS[]>([]);
  // Bumped each time a server response arrives; drives the countdown pie's refill.
  const [refreshCount, setRefreshCount] = useState(0);
  // A ticking value so the "late" evaluation re-renders even between polls.
  const [, setTick] = useState(0);
  // Rooms whose resend request is in flight, and those that have been requested (awaiting the next ping).
  const [requesting, setRequesting] = useState<Set<string>>(new Set());
  const [requested, setRequested] = useState<Set<string>>(new Set());

  useEffect(() => {
    // The endpoint is auth-gated (owner/admin/super user). On a fresh page load the
    // access token is repopulated asynchronously, so wait for it before fetching —
    // otherwise the first requests fire without a token and get a 403.
    if (!accessToken) {
      // The endpoint is auth-gated. Rather than wait indefinitely, actively seed the token
      // from the refresh cookie; when it lands, this effect re-runs (accessToken dep) and polls.
      console.log('Room Monitor: no access token yet — seeding it from the refresh session.');
      refresh();
      return;
    }
    let cancelled = false;
    const load = () => {
      console.log(`Room Monitor: fetch request sent to server for tournament ${tid} at ${new Date().toLocaleString()}.`);
      RoomAPI.getMonitorByTournament(tid, accessToken)
        .then((result) => {
          if (cancelled) return;
          // Guard against a non-array payload so a bad/error response can't crash the render.
          setRows(Array.isArray(result) ? result : []);
          // Clear the transient "requested" notes on each refresh.
          setRequested(new Set());
          // Response is back — refill the countdown pie.
          setRefreshCount((n) => n + 1);
        })
        .catch(() => { if (!cancelled) console.error('Failed to load room monitor data'); });
    };
    load();
    const poll = setInterval(load, POLL_MS);
    const tick = setInterval(() => setTick((t) => t + 1), POLL_MS);
    return () => {
      cancelled = true;
      clearInterval(poll);
      clearInterval(tick);
    };
  }, [tid, accessToken]);

  const handleResend = async (r: RoomMonitorRowTS) => {
    if (!r.game_id) return;
    setRequesting((prev) => new Set(prev).add(r.roomid));
    try {
      console.log(`Resend update request sent to server for game ${r.game_id} (room ${r.room ?? r.roomid}).`);
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
    <>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'flex-end', gap: 8, marginBottom: 8 }}>
        <span style={{ fontSize: 12, color: '#6b7280' }}>Next refresh</span>
        <CountdownPie resetKey={refreshCount} durationMs={POLL_MS} />
      </div>
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
    </>
  );
}
