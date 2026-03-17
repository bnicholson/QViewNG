import { useState, useCallback, useEffect } from 'react';
import { BoolBadge, DataTableTemplate, type ColumnDef } from './DataTableTemplate';
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

const QUIZZERS_PAGE = 0;
const QUIZZERS_PAGE_SIZE = 30;

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
  const [isLoading, setIsLoading] = useState(false);
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadQuizzers = () => {
    if (externalRows !== undefined) return;
    setIsLoading(true);
    UserAPI.get(QUIZZERS_PAGE, QUIZZERS_PAGE_SIZE)
      .then(setQuizzers)
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => {
    if (externalRows !== undefined) return;
    loadQuizzers();
  }, []);

  const rows = externalRows ?? quizzers;

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    if (onDelete) return onDelete(row);
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, [onDelete]);

  const handleSave = useCallback((_user: UserTS): void => {
    setEditorIsOpen(false);
    loadQuizzers();
  }, []);

  if (isLoading) return <div>Loading quizzers...</div>;

  return (
    <>
      <DataTableTemplate<UserTS>
        entityLabel="Quizzer"
        createLabel={createLabel}
        onCreate={onAdd ?? (() => setEditorIsOpen(true))}
        columns={quizzerColumns}
        rows={rows}
        getId={(u) => u.id}
        onDelete={handleDelete}
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
