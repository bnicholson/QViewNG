import { useState, useCallback, useEffect, useRef } from 'react';
import { BoolBadge, DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTS } from '../features/UserAPI';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

const quizzerColumns: ColumnDef<UserTS>[] = [
  {
    header: 'Full Name',
    render: (u) => `${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`,
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

interface Props {
  tid?: string;
  /** If provided, skips internal fetch and displays these rows directly. */
  externalRows?: UserTS[];
  /** When provided, shows a create button that calls this. Omit to hide the button entirely. */
  onAdd?: () => void;
  /** Overrides the delete handler. When omitted, throws (quizzers managed via rosters). */
  onDelete?: (user: UserTS) => Promise<void>;
  /** Overrides the create button label. Only meaningful when onAdd is provided. */
  createLabel?: string;
}

export default function QuizzersTable({ tid, externalRows, onAdd, onDelete, createLabel }: Props) {
  // All items for client-side pagination (tid or externalRows mode)
  const [allTournamentQuizzers, setAllTournamentQuizzers] = useState<UserTS[] | undefined>(undefined);
  // Current-page items for server-side pagination (no tid, no externalRows)
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [apiTotalCount, setApiTotalCount] = useState(0);

  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Fetch all quizzers for the tournament once; re-fetch when tid changes
  useEffect(() => {
    if (tid === undefined) return;
    setPage(0);
    setAllTournamentQuizzers(undefined);
    UserAPI.getByTournament(tid)
      .then(result => setAllTournamentQuizzers(result.items))
      .catch(() => console.error('Failed to load quizzers'));
  }, [tid]);

  // Server-side pagination fetch (only used when no tid and no externalRows)
  const loadQuizzers = useCallback((p: number, ps: number) => {
    if (externalRows !== undefined || tid !== undefined) return;
    UserAPI.get(p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setApiTotalCount(result.count);
        setQuizzers(result.items);
      })
      .catch(() => console.error('Failed to load quizzers'));
  }, [externalRows, tid]);

  useEffect(() => {
    loadQuizzers(0, pageSizeRef.current);
  }, []);

  // Client-side data: externalRows takes priority, then tournament quizzers
  const clientItems = externalRows ?? allTournamentQuizzers;

  // Slice for the current page when all data is loaded; otherwise use the server-fetched page
  const rows = clientItems !== undefined
    ? clientItems.slice(page * pageSize, (page + 1) * pageSize)
    : quizzers;

  const totalCount = clientItems !== undefined ? clientItems.length : apiTotalCount;

  const handlePageChange = useCallback((newPage: number) => {
    if (externalRows !== undefined || allTournamentQuizzers !== undefined) {
      setPage(newPage);
    } else {
      loadQuizzers(newPage, pageSize);
    }
  }, [externalRows, allTournamentQuizzers, pageSize, loadQuizzers]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPage(0);
    setPageSize(newSize);
    if (externalRows === undefined && allTournamentQuizzers === undefined) {
      loadQuizzers(0, newSize);
    }
  }, [externalRows, allTournamentQuizzers, loadQuizzers]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    if (onDelete) return onDelete(row);
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, [onDelete]);

  return (
    <DataTableTemplate<UserTS>
      entityLabel="Quizzer"
      createLabel={createLabel}
      onCreate={onAdd}
      showCreateButton={false}
      columns={quizzerColumns}
      rows={rows}
      totalCount={totalCount}
      getId={(u) => u.id}
      onDelete={handleDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={handlePageChange}
      onPageSizeChange={handlePageSizeChange}
    />
  );
}
