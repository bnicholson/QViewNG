import { useState, useCallback, useEffect, useRef } from 'react';
import { BoolBadge, DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTS } from '../features/UserAPI';
import { QuizzerEditorDialog } from './QuizzerEditorDialog';

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
  /** Overrides the create button action. When omitted, opens QuizzerEditorDialog. */
  onAdd?: () => void;
  /** Overrides the delete handler. When omitted, throws (quizzers managed via rosters). */
  onDelete?: (user: UserTS) => Promise<void>;
  /** Overrides the create button label. */
  createLabel?: string;
}

// Note: quizzers are associated with tournaments through rosters (roster → rosters_quizzers → user).
// There is no direct tournament-level quizzer endpoint in the backend. This table shows all
// registered users in the system. Quizzer-to-roster associations are managed separately.
export default function QuizzersTable({ tid: _tid, externalRows, onAdd, onDelete, createLabel }: Props) {
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadQuizzers = useCallback((p: number, ps: number) => {
    if (externalRows !== undefined) return;
    UserAPI.get(p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setQuizzers(result);
      })
      .catch(() => console.error('Failed to load quizzers'));
  }, [externalRows]);

  useEffect(() => {
    if (externalRows !== undefined) return;
    loadQuizzers(0, pageSizeRef.current);
  }, []);

  const rows = externalRows ?? quizzers;

  const handlePageChange = useCallback((newPage: number) => {
    loadQuizzers(newPage, pageSize);
  }, [pageSize, loadQuizzers]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (externalRows !== undefined) return;
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setQuizzers(prev => prev.slice(0, newSize));
    } else {
      loadQuizzers(0, newSize);
    }
  }, [pageSize, page, externalRows, loadQuizzers]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    if (onDelete) return onDelete(row);
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, [onDelete]);

  const handleSave = useCallback((_user: UserTS): void => {
    setEditorIsOpen(false);
    loadQuizzers(page, pageSize);
  }, [loadQuizzers, page, pageSize]);

  const totalCount = externalRows !== undefined
    ? externalRows.length
    : quizzers.length < pageSize
      ? page * pageSize + quizzers.length
      : (page + 2) * pageSize;

  return (
    <>
      <DataTableTemplate<UserTS>
        entityLabel="Quizzer"
        createLabel={createLabel}
        onCreate={onAdd ?? (() => setEditorIsOpen(true))}
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
      {!onAdd && (
        <QuizzerEditorDialog
          isOpen={editorIsOpen}
          onCancel={() => setEditorIsOpen(false)}
          onSave={handleSave}
        />
      )}
    </>
  );
}
