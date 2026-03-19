import { useState } from 'react';
import { BoolBadge } from './DataTableTemplate';
import { TeamAPI, type TeamTS } from '../features/TeamAPI';
import type { UserTS } from '../features/UserAPI';

const SLOT_FIELDS = [
  'quizzer_one_id',
  'quizzer_two_id',
  'quizzer_three_id',
  'quizzer_four_id',
  'quizzer_five_id',
  'quizzer_six_id',
] as const;

type SlotField = typeof SLOT_FIELDS[number];

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function RemoveButton({ onRemove }: { onRemove: () => Promise<void> }) {
  const [confirming, setConfirming] = useState(false);
  const [removing, setRemoving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleClick = async () => {
    if (!confirming) {
      setConfirming(true);
      setError(null);
      return;
    }
    setRemoving(true);
    try {
      await onRemove();
    } catch (err: any) {
      setError(err?.message ?? 'Remove failed');
      setConfirming(false);
    } finally {
      setRemoving(false);
    }
  };

  return (
    <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
      <button
        onClick={handleClick}
        disabled={removing}
        style={{
          padding: '3px 10px',
          borderRadius: 5,
          border: `1px solid ${confirming ? '#c0392b' : '#e0e0e0'}`,
          background: confirming ? '#c0392b' : 'transparent',
          color: confirming ? '#fff' : '#c0392b',
          fontSize: 12,
          fontWeight: 600,
          cursor: removing ? 'not-allowed' : 'pointer',
          opacity: removing ? 0.6 : 1,
          transition: 'all .15s',
          whiteSpace: 'nowrap',
        }}
      >
        {removing ? 'Removing…' : confirming ? 'Confirm' : 'Remove'}
      </button>
      {confirming && !removing && (
        <button
          onClick={() => { setConfirming(false); setError(null); }}
          style={{
            padding: '3px 8px',
            borderRadius: 5,
            border: '1px solid #e0e0e0',
            background: 'transparent',
            color: '#555',
            fontSize: 12,
            cursor: 'pointer',
          }}
        >
          Cancel
        </button>
      )}
      {error && <span style={{ fontSize: 12, color: '#c0392b' }}>{error}</span>}
    </div>
  );
}

interface Props {
  team: TeamTS;
  teamId: string;
  assignedUsers: UserTS[];
  onAdd: () => void;
  onRemoved: (updatedTeam: TeamTS) => void;
}

export default function TeamQuizzersTable({ team, teamId, assignedUsers, onAdd, onRemoved }: Props) {
  const handleRemove = async (user: UserTS) => {
    const slotField = SLOT_FIELDS.find((f): f is SlotField => team[f] === user.id);
    if (!slotField) return;
    const updatedTeam = await TeamAPI.update(teamId, { [slotField]: null });
    onRemoved(updatedTeam);
  };

  const colSpan = 6;

  return (
    <div>
      {/* ── Header row ── */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 16 }}>
        <div>
          <h2 style={{ margin: 0, fontSize: 20, fontWeight: 600, letterSpacing: '-0.01em' }}>
            Quizzers
          </h2>
          <p style={{ margin: '2px 0 0', fontSize: 13, color: '#666' }}>
            {assignedUsers.length} quizzer{assignedUsers.length !== 1 ? 's' : ''}
          </p>
        </div>
        <button
          onClick={onAdd}
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: 6,
            padding: '8px 16px',
            borderRadius: 7,
            background: '#2563eb',
            color: '#fff',
            border: 'none',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 600,
            boxShadow: '0 1px 3px rgba(37,99,235,.25)',
            transition: 'background .15s',
          }}
          onMouseEnter={(e) => (e.currentTarget.style.background = '#1d4ed8')}
          onMouseLeave={(e) => (e.currentTarget.style.background = '#2563eb')}
        >
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
            <path d="M7 1v12M1 7h12" stroke="#fff" strokeWidth="2" strokeLinecap="round" />
          </svg>
          Add Quizzers
        </button>
      </div>

      {/* ── Table ── */}
      <div style={{ overflowX: 'auto', borderRadius: 10, border: '1px solid #e5e7eb' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
          <thead>
            <tr style={{ background: '#f9fafb', borderBottom: '1px solid #e5e7eb' }}>
              {['Full Name', 'Email', 'Activated', 'Created', 'Last Modified'].map(h => (
                <th
                  key={h}
                  style={{
                    padding: '10px 14px',
                    textAlign: 'center',
                    fontWeight: 600,
                    fontSize: 12,
                    color: '#6b7280',
                    letterSpacing: '0.05em',
                    textTransform: 'uppercase',
                    whiteSpace: 'nowrap',
                  }}
                >
                  {h}
                </th>
              ))}
              <th style={{ padding: '10px 14px' }} />
            </tr>
          </thead>
          <tbody>
            {assignedUsers.length === 0 ? (
              <tr>
                <td
                  colSpan={colSpan}
                  style={{ padding: '32px 14px', textAlign: 'center', color: '#9ca3af' }}
                >
                  No quizzers assigned.
                </td>
              </tr>
            ) : (
              assignedUsers.map((user, i) => (
                <tr
                  key={user.id}
                  style={{
                    background: i % 2 === 0 ? '#fff' : '#fafafa',
                    borderBottom: '1px solid #f3f4f6',
                    transition: 'background .1s',
                  }}
                  onMouseEnter={(e) => (e.currentTarget.style.background = '#f0f7ff')}
                  onMouseLeave={(e) =>
                    (e.currentTarget.style.background = i % 2 === 0 ? '#fff' : '#fafafa')
                  }
                >
                  <td style={{ padding: '12px 14px', color: '#374151' }}>
                    {[user.fname, user.mname, user.lname].filter(Boolean).join(' ')}
                  </td>
                  <td style={{ padding: '12px 14px', color: '#374151' }}>{user.email}</td>
                  <td style={{ padding: '12px 14px', textAlign: 'center' }}>
                    <BoolBadge value={user.activated} />
                  </td>
                  <td style={{ padding: '12px 14px', color: '#6b7280', whiteSpace: 'nowrap' }}>
                    {formatDate(user.created_at)}
                  </td>
                  <td style={{ padding: '12px 14px', color: '#6b7280', whiteSpace: 'nowrap' }}>
                    {formatDate(user.updated_at)}
                  </td>
                  <td style={{ padding: '12px 14px' }}>
                    <RemoveButton onRemove={() => handleRemove(user)} />
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
