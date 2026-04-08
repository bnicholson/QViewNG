import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import CircularProgress from '@mui/material/CircularProgress'
import dayjs from 'dayjs'
import TournamentTable, { DEFAULT_PAGE_SIZE } from '../components/TournamentTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import {
  CreateTournamentApplicantAPI,
  type CreateTournamentApplicantTS,
} from '../features/CreateTournamentApplicantAPI'

interface Props {
  userId: string
  canCreate: boolean
  canDelete: boolean
  isTournamentManager: boolean
  isSuperUser: boolean
  targetIsSuperUser: boolean
  targetIsTournamentManager: boolean
}

// ── Status badge (mirrors the one in CreateTournamentApplicantsTable) ──────────

function StatusBadge({ status }: { status: string }) {
  const styles: Record<string, React.CSSProperties> = {
    pending:  { background: '#fef3c7', color: '#92400e', border: '1px solid #fde68a' },
    approved: { background: '#e6f4ea', color: '#1e7e34', border: '1px solid #a8d5b5' },
    declined: { background: '#fce8e6', color: '#c0392b', border: '1px solid #f5c6cb' },
  }
  const labels: Record<string, string> = {
    pending: 'Applied', approved: 'Approved', declined: 'Declined',
  }
  const style = styles[status] ?? { background: '#f3f4f6', color: '#6b7280', border: '1px solid #e5e7eb' }
  return (
    <span style={{
      display: 'inline-block', padding: '2px 10px', borderRadius: 12,
      fontSize: 12, fontWeight: 600, letterSpacing: '0.03em', whiteSpace: 'nowrap',
      ...style,
    }}>
      {labels[status] ?? status}
    </span>
  )
}

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

// ── Apply view (shown when user is not a tournament_manager) ──────────────────

function ApplicationHistoryTable({ applications }: { applications: CreateTournamentApplicantTS[] }) {
  return (
    <div style={{ overflowX: 'auto', borderRadius: 10, border: '1px solid #e5e7eb', width: '100%' }}>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
        <thead>
          <tr style={{ background: '#f9fafb', borderBottom: '1px solid #e5e7eb' }}>
            {['Status', 'Request Context', 'Applied', 'Last Modified'].map(h => (
              <th key={h} style={{
                padding: '8px 14px', textAlign: 'center', fontWeight: 600,
                fontSize: 12, color: '#6b7280', letterSpacing: '0.05em',
                textTransform: 'uppercase', whiteSpace: 'nowrap',
              }}>
                {h}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {applications.map((a, i) => (
            <tr key={a.id} style={{ background: i % 2 === 0 ? '#fff' : '#fafafa', borderBottom: '1px solid #f3f4f6' }}>
              <td style={{ padding: '10px 14px', textAlign: 'center' }}>
                <StatusBadge status={a.status} />
              </td>
              <td style={{ padding: '10px 14px', color: a.request_context ? '#374151' : '#9ca3af', fontStyle: a.request_context ? 'normal' : 'italic' }}>
                {a.request_context ?? 'None'}
              </td>
              <td style={{ padding: '10px 14px', color: '#6b7280', whiteSpace: 'nowrap' }}>
                {formatDate(a.created_at)}
              </td>
              <td style={{ padding: '10px 14px', color: '#6b7280', whiteSpace: 'nowrap' }}>
                {formatDate(a.modified_at)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function ApplyView({ userId }: { userId: string }) {
  const [applications, setApplications] = useState<CreateTournamentApplicantTS[] | undefined>(undefined)
  const [requestContext, setRequestContext] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    CreateTournamentApplicantAPI.getByUser(userId)
      .then(records => {
        const sorted = [...records].sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
        setApplications(sorted)
      })
      .catch(() => setApplications([]))
  }, [userId])

  if (applications === undefined) {
    return <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}><CircularProgress /></Box>
  }

  const latest = applications[0]
  const canApply = !latest || latest.status !== 'pending'

  const handleApply = async () => {
    setSubmitting(true)
    setError(null)
    try {
      const created = await CreateTournamentApplicantAPI.create({
        user_id: userId,
        last_modified_user_id: userId,
        request_context: requestContext.trim() || undefined,
      })
      setRequestContext('')
      setApplications(prev => [created, ...(prev ?? [])])
    } catch (err: any) {
      setError(err.message ?? 'Failed to submit application.')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <Box sx={{ mt: 2, maxWidth: 640, mx: 'auto' }}>
      {canApply && (
        <>
          <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
            You do not currently have permission to create or manage tournaments. You may apply below.
          </Typography>
          <TextField
            label="Why would you like to create tournaments?"
            multiline
            minRows={3}
            fullWidth
            value={requestContext}
            onChange={(e) => setRequestContext(e.target.value)}
            disabled={submitting}
            sx={{ mb: 2 }}
          />
          {error && (
            <Typography variant="body2" color="error" sx={{ mb: 2 }}>{error}</Typography>
          )}
          <Button variant="contained" disabled={submitting} onClick={handleApply} sx={{ mb: applications.length > 0 ? 4 : 0 }}>
            {submitting ? 'Submitting…' : 'Apply to become a Tournament Creator, Owner, and Manager'}
          </Button>
          <br/>
        </>
      )}
      {!canApply && (
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          Your application is currently pending review.
        </Typography>
      )}
      {applications.length > 0 && (
        <>
          <Typography variant="subtitle2" sx={{ mb: 1, fontWeight: 600 }}>
            Application History
          </Typography>
          <ApplicationHistoryTable applications={applications} />
        </>
      )}
    </Box>
  )
}

// ── Main export ───────────────────────────────────────────────────────────────

export const UserProfileManagedTournamentsPage = ({ userId, canCreate, canDelete, isTournamentManager, isSuperUser, targetIsSuperUser, targetIsTournamentManager }: Props) => {
  const [allRows, setAllRows] = useState<TournamentTS[]>([])
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const [loading, setLoading] = useState(true)
  const [editorOpen, setEditorOpen] = useState(false)
  const [revoking, setRevoking] = useState(false)
  const [revokeError, setRevokeError] = useState<string | null>(null)
  const [revokeConfirming, setRevokeConfirming] = useState(false)

  useEffect(() => {
    if (!isTournamentManager) { setLoading(false); return }
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
  }, [userId, isTournamentManager])

  const handleRevoke = async () => {
    if (!revokeConfirming) { setRevokeConfirming(true); return }
    setRevoking(true)
    setRevokeError(null)
    try {
      const rolesRes = await fetch('/api/roles')
      const rolesBody: { items: { id: string; name: string }[] } = await rolesRes.json()
      const tmRole = rolesBody.items.find(r => r.name === 'tournament_manager')
      if (!tmRole) throw new Error('tournament_manager role not found')
      const res = await fetch(`/api/usersroles/users/${userId}/roles/${tmRole.id}`, { method: 'DELETE' })
      if (!res.ok) throw new Error(`Failed to revoke role (${res.status})`)
      window.location.reload()
    } catch (err: any) {
      setRevokeError(err.message ?? 'Failed to revoke privileges.')
      setRevokeConfirming(false)
    } finally {
      setRevoking(false)
    }
  }

  if (!isTournamentManager) {
    return <ApplyView userId={userId} />
  }

  const visibleRows = allRows.slice(page * pageSize, page * pageSize + pageSize)

  return (
    <Box>
      {isSuperUser && !targetIsTournamentManager && (
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: "left" }}>
          Note: <em>This user does not have the appropriate permissions to create or manage Tournaments and Tournament Groups.</em>
        </Typography>
      )}
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: 'left' }}>
        This page shows all tournaments that you are the owner and manager of.
      </Typography>
      {isSuperUser && !targetIsSuperUser && targetIsTournamentManager && (
        <Box sx={{ mb: 2, display: 'flex', alignItems: 'center', gap: 2 }}>
          <Button
            variant="outlined"
            color="error"
            size="small"
            disabled={revoking}
            onClick={handleRevoke}
          >
            {revoking ? 'Revoking…' : revokeConfirming ? 'Confirm Revoke?' : 'Revoke Tournament Create/Manage/Owner Privileges'}
          </Button>
          {revokeConfirming && !revoking && (
            <Button size="small" onClick={() => setRevokeConfirming(false)}>Cancel</Button>
          )}
          {revokeError && (
            <Typography variant="body2" color="error">{revokeError}</Typography>
          )}
        </Box>
      )}
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

  function handleDelete(t: TournamentTS): Promise<void> {
    return TournamentAPI.delete(t.tid).then(() => {
      setAllRows(prev => prev.filter(r => r.tid !== t.tid))
    })
  }

  function handleCreated(t: TournamentTS) {
    setEditorOpen(false)
    setAllRows(prev => [...prev, t])
  }
}
