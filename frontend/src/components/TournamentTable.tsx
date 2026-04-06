import { useState } from 'react'
import { Link } from 'react-router-dom'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate'
import type { TournamentTS } from '../features/TournamentAPI'

function RemoveButton({ onRemove }: { onRemove: () => Promise<void> }) {
  const [confirming, setConfirming] = useState(false)
  const [removing, setRemoving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleClick = async () => {
    if (!confirming) { setConfirming(true); setError(null); return }
    setRemoving(true)
    try {
      await onRemove()
    } catch (err: any) {
      setError(err?.message ?? 'Remove failed')
      setConfirming(false)
    } finally {
      setRemoving(false)
    }
  }

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
          onClick={() => { setConfirming(false); setError(null) }}
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
  )
}

function tournamentColumns(showRemoveButton: boolean, onRemove?: (t: TournamentTS) => Promise<void>): ColumnDef<TournamentTS>[] {
  return [
    {
      header: 'Tournament',
      render: t => (
        <Link
          to={`/tournament/${t.tid}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
        >
          {t.tname}
        </Link>
      ),
    },
    {
      header: 'Dates',
      render: t => (
        <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>
          {t.fromdate?.format('MMM D, YYYY')} – {t.todate?.format('MMM D, YYYY')}
        </span>
      ),
    },
    {
      header: 'Venue',
      render: t => t.venue,
    },
    {
      header: 'Location',
      render: t => [t.city, t.region, t.country].filter(Boolean).join(', '),
    },
    ...(showRemoveButton && onRemove ? [{
      header: '',
      render: (t: TournamentTS) => <RemoveButton onRemove={() => onRemove(t)} />,
    }] : []),
  ]
}

interface Props {
  tournaments: TournamentTS[]
  totalCount: number
  page: number
  pageSize: number
  showCreateButton?: boolean
  showDeleteButton?: boolean
  showRemoveButton?: boolean
  onCreate?: () => void
  onDelete: (t: TournamentTS) => Promise<void>
  onRemove?: (t: TournamentTS) => Promise<void>
  onPageChange: (page: number) => void
  onPageSizeChange: (size: number) => void
}

export default function TournamentTable({
  tournaments,
  totalCount,
  page,
  pageSize,
  showCreateButton = false,
  showDeleteButton = true,
  showRemoveButton = false,
  onCreate,
  onDelete,
  onRemove,
  onPageChange,
  onPageSizeChange,
}: Props) {
  return (
    <DataTableTemplate<TournamentTS>
      entityLabel="Tournament"
      showCreateButton={showCreateButton}
      onCreate={onCreate}
      showDeleteButton={showDeleteButton}
      columns={tournamentColumns(showRemoveButton, onRemove)}
      rows={tournaments}
      totalCount={totalCount}
      getId={t => String(t.tid)}
      onDelete={onDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={onPageChange}
      onPageSizeChange={onPageSizeChange}
    />
  )
}

export { DEFAULT_PAGE_SIZE }
