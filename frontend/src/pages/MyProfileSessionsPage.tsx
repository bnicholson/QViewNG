import { useCallback, useEffect, useRef, useState } from 'react'
import { useAuth } from '../hooks/useAuth'
import type { UserSessionResponse } from '../hooks/useAuth'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from '../components/DataTableTemplate'

type SessionRow = UserSessionResponse['sessions'][number]

function formatDate(iso: string): string {
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit',
  })
}

const sessionColumns: ColumnDef<SessionRow>[] = [
  {
    header: 'Device',
    render: (s) => s.device ?? '—',
  },
  {
    header: 'Created',
    render: (s) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(s.created_at)}</span>
    ),
  },
]

export const MyProfileSessionsPage = () => {
  const auth = useAuth()

  const [sessions, setSessions] = useState<SessionRow[]>([])
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const [isDeletingAll, setDeletingAll] = useState(false)
  const pageSizeRef = useRef(pageSize)
  pageSizeRef.current = pageSize

  const loadSessions = useCallback((p: number, ps: number) => {
    fetch(`/api/auth/sessions?page=${p}&page_size=${ps}`, {
      headers: { Authorization: `Bearer ${auth.accessToken}` },
    })
      .then((r) => r.json())
      .then((data: UserSessionResponse) => {
        setPage(p)
        setPageSize(ps)
        setSessions(data.sessions)
      })
      .catch(() => console.error('Failed to load sessions'))
  }, [auth.accessToken])

  useEffect(() => {
    if (auth.isAuthenticated) loadSessions(0, pageSizeRef.current)
  }, [auth.isAuthenticated])

  const handlePageChange = useCallback((newPage: number) => {
    loadSessions(newPage, pageSize)
  }, [pageSize, loadSessions])

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize)
      setSessions((prev) => prev.slice(0, newSize))
    } else {
      loadSessions(0, newSize)
    }
  }, [pageSize, page, loadSessions])

  const handleDelete = useCallback(async (row: SessionRow): Promise<void> => {
    const response = await fetch(`/api/auth/sessions/${row.id}`, {
      method: 'DELETE',
      headers: { Authorization: `Bearer ${auth.accessToken}` },
    })
    if (!response.ok) throw new Error('Failed to delete session')
    setSessions((prev) => prev.filter((s) => s.id !== row.id))
  }, [auth.accessToken])

  const handleDeleteAll = async () => {
    setDeletingAll(true)
    await fetch('/api/auth/sessions', {
      method: 'DELETE',
      headers: { Authorization: `Bearer ${auth.accessToken}` },
    })
    setDeletingAll(false)
    loadSessions(0, pageSize)
  }

  const totalCount = sessions.length < pageSize
    ? page * pageSize + sessions.length
    : (page + 2) * pageSize

  return (
    <>
      <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 8 }}>
        <button
          disabled={isDeletingAll || sessions.length === 0}
          onClick={handleDeleteAll}
          style={{
            padding: '3px 10px',
            borderRadius: 5,
            border: '1px solid #c0392b',
            background: 'transparent',
            color: '#c0392b',
            fontSize: 12,
            fontWeight: 600,
            cursor: isDeletingAll ? 'not-allowed' : 'pointer',
          }}
        >
          Delete All
        </button>
      </div>
      <DataTableTemplate<SessionRow>
        entityLabel="Session"
        columns={sessionColumns}
        rows={sessions}
        totalCount={totalCount}
        getId={(s) => s.id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
    </>
  )
}
