import { useState, useCallback, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import {
  CreateTournamentApplicantAPI,
  type CreateTournamentApplicantTS,
} from '../features/CreateTournamentApplicantAPI';
import { UserAPI } from '../features/UserAPI';
import { CreateTournamentApplicantEditorDialog } from './CreateTournamentApplicantEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function StatusBadge({ status }: { status: string }) {
  const styles: Record<string, React.CSSProperties> = {
    pending:  { background: '#fef3c7', color: '#92400e', border: '1px solid #fde68a' },
    approved: { background: '#e6f4ea', color: '#1e7e34', border: '1px solid #a8d5b5' },
    declined: { background: '#fce8e6', color: '#c0392b', border: '1px solid #f5c6cb' },
  };
  const labels: Record<string, string> = {
    pending: 'Pending', approved: 'Approved', declined: 'Declined',
  };
  const style = styles[status] ?? { background: '#f3f4f6', color: '#6b7280', border: '1px solid #e5e7eb' };
  return (
    <span style={{
      display: 'inline-block', padding: '2px 10px', borderRadius: 12,
      fontSize: 12, fontWeight: 600, letterSpacing: '0.03em', whiteSpace: 'nowrap',
      ...style,
    }}>
      {labels[status] ?? status}
    </span>
  );
}

function applicantColumns(
  onEdit: (a: CreateTournamentApplicantTS) => void,
  userNames: Record<string, string>,
): ColumnDef<CreateTournamentApplicantTS>[] {
  return [
    {
      header: 'Name',
      render: (a) => {
        const name = userNames[a.user_id];
        return (
          <Link
            to={`/user/${a.user_id}/overview`}
            style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
            onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
            onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
          >
            {name ?? '—'}
          </Link>
        );
      },
    },
    {
      header: 'Status',
      render: (a) => {
        const editable = a.status === 'pending';
        return editable ? (
          <button
            onClick={() => onEdit(a)}
            style={{ background: 'none', border: 'none', padding: 0, cursor: 'pointer' }}
            title="Edit applicant"
          >
            <StatusBadge status={a.status} />
          </button>
        ) : (
          <StatusBadge status={a.status} />
        );
      },
    },
    {
      header: 'Request Context',
      render: (a) => (
        <span style={{ color: a.request_context ? '#374151' : '#9ca3af', fontStyle: a.request_context ? 'normal' : 'italic' }}>
          {a.request_context ?? 'None'}
        </span>
      ),
    },
    {
      header: 'Applied',
      render: (a) => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(a.created_at)}</span>,
    },
    {
      header: 'Last Modified',
      render: (a) => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(a.modified_at)}</span>,
    },
    {
      header: 'Last Modified By',
      render: (a) => {
        const name = userNames[a.last_modified_user_id];
        return (
          <Link
            to={`/user/${a.last_modified_user_id}/overview`}
            style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
            onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
            onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
          >
            {name ?? '—'}
          </Link>
        );
      },
    },
  ];
}

interface Props {
  currentUserId: string;
}

export default function CreateTournamentApplicantsTable({ currentUserId }: Props) {
  const [items, setItems] = useState<CreateTournamentApplicantTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editing, setEditing] = useState<CreateTournamentApplicantTS | undefined>(undefined);
  const [userNames, setUserNames] = useState<Record<string, string>>({});

  const loadUserNames = useCallback((applicants: CreateTournamentApplicantTS[]) => {
    const ids = [...new Set([
      ...applicants.map(a => a.user_id),
      ...applicants.map(a => a.last_modified_user_id),
    ])];
    Promise.all(ids.map(id => UserAPI.getById(id).catch(() => null))).then(users => {
      const names: Record<string, string> = {};
      users.forEach((u, i) => {
        if (u) names[ids[i]] = [u.fname, u.mname, u.lname].filter(Boolean).join(' ');
      });
      setUserNames(prev => ({ ...prev, ...names }));
    });
  }, []);

  const load = useCallback((p: number, ps: number) => {
    CreateTournamentApplicantAPI.get(p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.count);
        setItems(result.items);
        loadUserNames(result.items);
      })
      .catch(() => console.error('Failed to load applicants'));
  }, [loadUserNames]);

  useEffect(() => { load(0, DEFAULT_PAGE_SIZE); }, [load]);

  const handlePageChange = useCallback((newPage: number) => {
    load(newPage, pageSize);
  }, [pageSize, load]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setItems(prev => prev.slice(0, newSize));
    } else {
      load(0, newSize);
    }
  }, [pageSize, page, load]);

  const handleDelete = useCallback(async (row: CreateTournamentApplicantTS): Promise<void> => {
    await CreateTournamentApplicantAPI.delete(row.id);
    setItems(prev => prev.filter(a => a.id !== row.id));
    setTotalCount(prev => prev - 1);
  }, []);

  const handleEditSave = useCallback((updated: CreateTournamentApplicantTS) => {
    setEditing(undefined);
    setItems(prev => prev.map(a => a.id === updated.id ? updated : a));
  }, []);

  return (
    <>
      <DataTableTemplate<CreateTournamentApplicantTS>
        entityLabel="Applicant"
        showCreateButton={false}
        showDeleteButton={false}
        columns={applicantColumns((a) => setEditing(a), userNames)}
        rows={items}
        totalCount={totalCount}
        getId={(a) => a.id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />

      <CreateTournamentApplicantEditorDialog
        applicant={editing}
        isOpen={editing !== undefined}
        currentUserId={currentUserId}
        onCancel={() => setEditing(undefined)}
        onSave={handleEditSave}
      />
    </>
  );
}
