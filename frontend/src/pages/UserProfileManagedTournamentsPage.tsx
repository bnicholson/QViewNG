import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import dayjs from 'dayjs'
import TournamentTable, { DEFAULT_PAGE_SIZE } from '../components/TournamentTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'

interface Props {
  userId: string
  canCreate: boolean
  canDelete: boolean
}

export const UserProfileManagedTournamentsPage = ({ userId, canCreate, canDelete }: Props) => {
  const [allRows, setAllRows] = useState<TournamentTS[]>([])
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const [loading, setLoading] = useState(true)
  const [editorOpen, setEditorOpen] = useState(false)

  useEffect(() => {
    setLoading(true)
    TournamentAPI.get(0, '500')
      .then(result => {
        const owned = result.items
          .filter(t => t.owner_id === userId)
          .map(t => ({
            ...t,
            fromdate: t.fromdate ? dayjs(t.fromdate as any) : null,
            todate: t.todate ? dayjs(t.todate as any) : null,
          }))
        setAllRows(owned)
        setPage(0)
      })
      .catch(() => console.error('Failed to load managed tournaments'))
      .finally(() => setLoading(false))
  }, [userId])

  const handleDelete = async (t: TournamentTS): Promise<void> => {
    await TournamentAPI.delete(t.tid)
    setAllRows(prev => prev.filter(r => r.tid !== t.tid))
  }

  const handleCreated = (t: TournamentTS) => {
    setEditorOpen(false)
    setAllRows(prev => [...prev, t])
  }

  const visibleRows = allRows.slice(page * pageSize, page * pageSize + pageSize)

  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: 'left' }}>
        This page shows all tournaments that you are the owner and manager of.
      </Typography>
      <TournamentTable
        tournaments={loading ? [] : visibleRows}
        totalCount={allRows.length}
        page={page}
        pageSize={pageSize}
        showCreateButton={canCreate}
        onCreate={() => setEditorOpen(true)}
        showDeleteButton={canDelete}
        onDelete={handleDelete}
        onPageChange={p => setPage(p)}
        onPageSizeChange={ps => { setPageSize(ps); setPage(0) }}
      />
      <TournamentEditorDialog
        isOpen={editorOpen}
        onCancel={() => setEditorOpen(false)}
        onSave={handleCreated}
      />
    </Box>
  )
}
