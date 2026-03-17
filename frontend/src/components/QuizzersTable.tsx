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

const QUIZZERS_PAGE = 0;
const QUIZZERS_PAGE_SIZE = 30;

// Note: quizzers are associated with tournaments through rosters (roster → rosters_quizzers → user).
// There is no direct tournament-level quizzer endpoint in the backend. This table shows all
// registered users in the system. Quizzer-to-roster associations are managed separately.
export default function QuizzersTable({ tid: _tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState(false);
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadQuizzers = () => {
    setIsLoading(true);
    UserAPI.get(QUIZZERS_PAGE, QUIZZERS_PAGE_SIZE)
      .then(setQuizzers)
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => { loadQuizzers(); }, []);

  // Quizzers are not directly deletable from a tournament — they are managed via rosters.
  const handleDelete = useCallback(async (_row: UserTS): Promise<void> => {
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, []);

  const handleSave = useCallback((_user: UserTS): void => {
    setEditorIsOpen(false);
    loadQuizzers();
  }, []);

  if (isLoading) return <div>Loading quizzers...</div>;

  return (
    <>
      <DataTableTemplate<UserTS>
        entityLabel="Quizzer"
        onCreate={() => setEditorIsOpen(true)}
        columns={quizzerColumns}
        rows={quizzers}
        getId={(u) => u.id}
        onDelete={handleDelete}
      />
      <QuizzerEditorDialog
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
