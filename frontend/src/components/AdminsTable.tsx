import { useState, useCallback, useEffect } from 'react';
import { BoolBadge, DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { AdminAPI } from '../features/AdminAPI';
import { UserAPI, type UserTS } from '../features/UserAPI';
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

const ADMINS_PAGE = 0;
const ADMINS_PAGE_SIZE = 30;

export default function AdminsTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState(false);
  const [admins, setAdmins] = useState<UserTS[]>([]);
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadAdmins = () => {
    setIsLoading(true);
    AdminAPI.getByTournament(tid, ADMINS_PAGE, ADMINS_PAGE_SIZE)
      .then(setAdmins)
      .catch(() => console.error('Failed to load admins'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => { loadAdmins(); }, [tid]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    await AdminAPI.delete(tid, row.id);
    setAdmins(prev => prev.filter(a => a.id !== row.id));
  }, [tid]);

  const handleSave = useCallback((_admin: UserTS): void => {
    setEditorIsOpen(false);
    loadAdmins();
  }, []);

  if (isLoading) return <div>Loading admins...</div>;

  return (
    <>
      <DataTableTemplate<UserTS>
        entityLabel="Admin"
        onCreate={() => setEditorIsOpen(true)}
        columns={adminColumns}
        rows={admins}
        getId={(u) => u.id}
        onDelete={handleDelete}
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
