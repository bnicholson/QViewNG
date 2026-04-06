import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import { Link } from 'react-router-dom'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from '../components/DataTableTemplate'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'
import { TournamentGroupEditorDialog } from '../components/TournamentGroupEditorDialog'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

const columns: ColumnDef<TournamentGroupTS>[] = [
  {
    header: 'Name',
    render: g => (
      <Link
        to={`/tournament-group/${g.tgid}/overview`}
        style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
        onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
        onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
      >
        {g.name}
      </Link>
    ),
  },
  {
    header: 'Description',
    render: g => g.description || <span style={{ color: '#9ca3af' }}>—</span>,
  },
  {
    header: 'Created',
    render: g => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.created_at)}</span>,
  },
  {
    header: 'Last Modified',
    render: g => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.updated_at)}</span>,
  },
]

interface Props {
  userId: string
  canCreate: boolean
  canDelete: boolean
}

export const UserProfileManagedTournamentGroupsPage = ({ userId, canCreate, canDelete }: Props) => {
  const [allRows, setAllRows] = useState<TournamentGroupTS[]>([])
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const [loading, setLoading] = useState(true)
  const [editorOpen, setEditorOpen] = useState(false)

  useEffect(() => {
    setLoading(true)
    TournamentGroupAPI.getAll(0, 500)
      .then(result => {
        setAllRows(result.items.filter(g => g.owner_id === userId))
        setPage(0)
      })
      .catch(() => console.error('Failed to load managed tournament groups'))
      .finally(() => setLoading(false))
  }, [userId])

  const handleDelete = async (g: TournamentGroupTS): Promise<void> => {
    await TournamentGroupAPI.delete(g.tgid)
    setAllRows(prev => prev.filter(r => r.tgid !== g.tgid))
  }

  const handleCreated = (g: TournamentGroupTS) => {
    setEditorOpen(false)
    setAllRows(prev => [...prev, g])
  }

  const visibleRows = allRows.slice(page * pageSize, page * pageSize + pageSize)

  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: 'left' }}>
        This page shows all tournament groups that you are the owner and manager of.
      </Typography>
      <DataTableTemplate<TournamentGroupTS>
        entityLabel="Tournament Group"
        showCreateButton={canCreate}
        onCreate={() => setEditorOpen(true)}
        showDeleteButton={canDelete}
        columns={columns}
        rows={loading ? [] : visibleRows}
        totalCount={allRows.length}
        getId={g => g.tgid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={p => setPage(p)}
        onPageSizeChange={ps => { setPageSize(ps); setPage(0) }}
      />
      <TournamentGroupEditorDialog
        isOpen={editorOpen}
        onCancel={() => setEditorOpen(false)}
        onSave={handleCreated}
      />
    </Box>
  )
}
