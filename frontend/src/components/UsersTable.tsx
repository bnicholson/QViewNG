import { useState, useCallback, useEffect } from 'react';
import { BoolBadge, DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTS } from '../features/UserAPI';
import { UserEditorDialog } from './UserEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function userColumns(onEdit: (user: UserTS) => void): ColumnDef<UserTS>[] {
  return [
    {
      header: 'Full Name',
      render: (u) => (
        <button
          onClick={() => onEdit(u)}
          style={{
            background: 'none', border: 'none', padding: 0,
            color: '#2563eb', fontWeight: 500, cursor: 'pointer',
            textDecoration: 'none', whiteSpace: 'nowrap',
          }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {u.fname} {u.mname ? u.mname + ' ' : ''}{u.lname}
        </button>
      ),
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
}

const USERS_PAGE = 0;
const USERS_PAGE_SIZE = 50;

export default function UsersTable() {
  const [isLoading, setIsLoading] = useState(false);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [createOpen, setCreateOpen] = useState(false);
  const [editingUser, setEditingUser] = useState<UserTS | undefined>(undefined);

  const loadUsers = () => {
    setIsLoading(true);
    UserAPI.get(USERS_PAGE, USERS_PAGE_SIZE)
      .then(setUsers)
      .catch(() => console.error('Failed to load users'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => { loadUsers(); }, []);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    await UserAPI.delete(row.id);
    setUsers(prev => prev.filter(u => u.id !== row.id));
  }, []);

  const handleCreateSave = useCallback((_user: UserTS): void => {
    setCreateOpen(false);
    loadUsers();
  }, []);

  const handleEditSave = useCallback((updated: UserTS): void => {
    setEditingUser(undefined);
    setUsers(prev => prev.map(u => u.id === updated.id ? updated : u));
  }, []);

  if (isLoading) return <div>Loading users...</div>;

  return (
    <>
      <DataTableTemplate<UserTS>
        entityLabel="User"
        onCreate={() => setCreateOpen(true)}
        columns={userColumns((u) => setEditingUser(u))}
        rows={users}
        getId={(u) => u.id}
        onDelete={handleDelete}
      />

      {/* Create dialog */}
      <UserEditorDialog
        isOpen={createOpen}
        onCancel={() => setCreateOpen(false)}
        onSave={handleCreateSave}
      />

      {/* Edit dialog — opens when a user's name is clicked */}
      <UserEditorDialog
        initialUser={editingUser}
        isOpen={editingUser !== undefined}
        onCancel={() => setEditingUser(undefined)}
        onSave={handleEditSave}
      />
    </>
  );
}
