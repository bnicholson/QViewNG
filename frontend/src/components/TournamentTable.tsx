import { Link } from 'react-router-dom'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate'
import type { TournamentTS } from '../features/TournamentAPI'

function tournamentColumns(): ColumnDef<TournamentTS>[] {
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
  ]
}

interface Props {
  tournaments: TournamentTS[]
  totalCount: number
  page: number
  pageSize: number
  showDeleteButton?: boolean
  onDelete: (t: TournamentTS) => Promise<void>
  onPageChange: (page: number) => void
  onPageSizeChange: (size: number) => void
}

export default function TournamentTable({
  tournaments,
  totalCount,
  page,
  pageSize,
  showDeleteButton = true,
  onDelete,
  onPageChange,
  onPageSizeChange,
}: Props) {
  return (
    <DataTableTemplate<TournamentTS>
      entityLabel="Tournament"
      showCreateButton={false}
      showDeleteButton={showDeleteButton}
      columns={tournamentColumns()}
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
