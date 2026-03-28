import { useState, useCallback, useEffect, useRef } from 'react';
import { BoolBadge, DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { AdminAPI } from '../features/AdminAPI';
import { type UserTS } from '../features/UserAPI';
import { AdminEditorDialog } from './AdminEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

const adminColumns: ColumnDef<UserTS>[] = [
  {
    header: 'Full Name',
    render: (u) => `${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`,
  },
  {
    header: 'Username',
    render: (u) => u.username,
  },
  {
    header: 'Email',
    render: (u) => u.email,
  },
  {
    header: 'Activated',
    render: (u) => <BoolBadge value={u.activated} />,
  },
  {
    header: 'Created',
    render: (u) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.created_at)}</span>
    ),
  },
  {
    header: 'Last Modified',
    render: (u) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.updated_at)}</span>
    ),
  },
];

export default function AdminsTable({ tid, showCreateButton = true, showDeleteButton = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean }) {
  const [admins, setAdmins] = useState<UserTS[]>([]);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadAdmins = useCallback((p: number, ps: number) => {
    AdminAPI.getByTournament(tid, p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setAdmins(result);
      })
      .catch(() => console.error('Failed to load admins'));
  }, [tid]);

  useEffect(() => {
    loadAdmins(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadAdmins(newPage, pageSize);
  }, [pageSize, loadAdmins]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setAdmins(prev => prev.slice(0, newSize));
    } else {
      loadAdmins(0, newSize);
    }
  }, [pageSize, page, loadAdmins]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    await AdminAPI.delete(tid, row.id);
    setAdmins(prev => prev.filter(a => a.id !== row.id));
  }, [tid]);

  const handleSave = useCallback((_admin: UserTS): void => {
    setEditorIsOpen(false);
    loadAdmins(page, pageSize);
  }, [loadAdmins, page, pageSize]);

  const totalCount = admins.length < pageSize
    ? page * pageSize + admins.length
    : (page + 2) * pageSize;

  return (
    <>
      <DataTableTemplate<UserTS>
        key={tid}
        entityLabel="Admin"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={adminColumns}
        rows={admins}
        totalCount={totalCount}
        getId={(u) => u.id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <AdminEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
