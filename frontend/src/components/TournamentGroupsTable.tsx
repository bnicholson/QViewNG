import { useState, useCallback, useEffect, useRef } from 'react'
import IconButton from '@mui/material/IconButton'
import EditIcon from '@mui/icons-material/Edit'
import { Link } from 'react-router-dom'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'
import { TournamentGroupEditorDialog } from './TournamentGroupEditorDialog'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
}

function groupColumns(onEdit: (group: TournamentGroupTS) => void): ColumnDef<TournamentGroupTS>[] {
  return [
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
    {
      header: '',
      render: g => (
        <IconButton size="small" onClick={() => onEdit(g)} aria-label="Edit tournament group">
          <EditIcon fontSize="small" />
        </IconButton>
      ),
    },
  ];
}

interface Props {
  tid: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}

export default function TournamentGroupsTable({ tid, showCreateButton = true, showDeleteButton = true }: Props) {
  const [groups, setGroups] = useState<TournamentGroupTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingGroup, setEditingGroup] = useState<TournamentGroupTS | undefined>(undefined);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadGroups = useCallback((p: number, ps: number) => {
    TournamentGroupAPI.getByTournament(tid, p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setGroups(result);
      })
      .catch(() => console.error('Failed to load tournament groups'));
  }, [tid]);

  useEffect(() => {
    loadGroups(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGroups(newPage, pageSize);
  }, [pageSize, loadGroups]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setGroups(prev => prev.slice(0, newSize));
    } else {
      loadGroups(0, newSize);
    }
  }, [pageSize, page, loadGroups]);

  // Delete removes the association between this tournament and the group.
  const handleDelete = useCallback(async (row: TournamentGroupTS): Promise<void> => {
    await TournamentGroupAPI.removeFromTournament(row.tgid, tid);
    setGroups(prev => prev.filter(g => g.tgid !== row.tgid));
  }, [tid]);

  const openCreate = () => {
    setEditingGroup(undefined);
    setEditorIsOpen(true);
  };

  const openEdit = (group: TournamentGroupTS) => {
    setEditingGroup(group);
    setEditorIsOpen(true);
  };

  const handleSave = useCallback((saved: TournamentGroupTS) => {
    setEditorIsOpen(false);
    if (editingGroup) {
      setGroups(prev => prev.map(g => g.tgid === saved.tgid ? saved : g));
    } else {
      loadGroups(page, pageSize);
    }
  }, [editingGroup, loadGroups, page, pageSize]);

  return (
    <>
      <DataTableTemplate<TournamentGroupTS>
        key={tid}
        entityLabel="Tournament Group"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={openCreate}
        columns={groupColumns(openEdit)}
        rows={groups}
        totalCount={totalCount}
        getId={g => g.tgid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <TournamentGroupEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
        initialGroup={editingGroup}
      />
    </>
  );
}
