import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { BoolBadge, DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTS } from '../features/UserAPI';
import { QuizzerAPI, type QuizzerRowTS, type EntityRef } from '../features/QuizzerAPI';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

/** Render a list of entity references as comma-separated profile links (or "—" when empty). */
function renderRefs(refs: EntityRef[] | undefined, base: 'division' | 'team') {
  if (!refs || refs.length === 0) return '—';
  return refs.map((r, i) => (
    <span key={r.id}>{i > 0 ? ', ' : ''}<EntityLink to={`/${base}/${r.id}/overview`}>{r.name}</EntityLink></span>
  ));
}

function quizzerColumns(
  showSensitiveColumns: boolean,
  showAuditColumns: boolean,
  showTeamAndDivision: boolean,
): ColumnDef<UserTS>[] {
  return [
    ...(showTeamAndDivision ? [
      {
        header: 'Division',
        render: (u: UserTS) => renderRefs((u as QuizzerRowTS).divisions, 'division'),
      },
      {
        header: 'Team',
        render: (u: UserTS) => renderRefs((u as QuizzerRowTS).teams, 'team'),
      },
    ] : []),
    {
      header: 'Full Name',
      render: (u) => (
        <Link
          to={`/user/${u.id}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {`${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`}
        </Link>
      ),
    },
    ...(showSensitiveColumns ? [{
      header: 'Email',
      render: (u: UserTS) => u.email,
    },
    {
      header: 'Activated',
      render: (u: UserTS) => <BoolBadge value={u.activated} />,
    }] : []),
    ...(showAuditColumns ? [
      {
        header: 'Created',
        render: (u: UserTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.created_at)}</span>
        ),
      },
      {
        header: 'Last Modified',
        render: (u: UserTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.updated_at)}</span>
        ),
      },
    ] : []),
  ];
}

interface Props {
  /** Tournament scope: fetches every quizzer in the tournament (one enriched call). */
  tid?: string;
  /** Division scope: fetches every quizzer in the division (one enriched call). */
  did?: string;
  /** If provided, skips internal fetch and displays these rows directly (e.g. a team roster). */
  externalRows?: UserTS[];
  /** When provided, shows a create button that calls this. Omit to hide the button entirely. */
  onAdd?: () => void;
  /** Overrides the delete handler. When omitted, throws (quizzers managed via rosters). */
  onDelete?: (user: UserTS) => Promise<void>;
  /** Overrides the create button label. Only meaningful when onAdd is provided. */
  createLabel?: string;
  showSensitiveColumns?: boolean;
  showAuditColumns?: boolean;
}

export default function QuizzersTable({ tid, did, externalRows, onAdd, onDelete, createLabel, showSensitiveColumns = false, showAuditColumns = true }: Props) {
  // Enriched rows (with divisions/teams) for the whole tournament or division — one API call.
  const [enrichedRows, setEnrichedRows] = useState<QuizzerRowTS[] | undefined>(undefined);
  // Current-page items for server-side pagination (no tid, no did, no externalRows).
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [apiTotalCount, setApiTotalCount] = useState(0);

  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Single enriched fetch for a tournament or a division; re-fetch when the scope changes.
  useEffect(() => {
    if (tid === undefined && did === undefined) return;
    setPage(0);
    setLoading(true);
    setEnrichedRows(undefined);
    const request = tid !== undefined
      ? QuizzerAPI.getByTournament(tid)
      : QuizzerAPI.getByDivision(did!);
    request
      .then(rows => setEnrichedRows(rows))
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setLoading(false));
  }, [tid, did]);

  // externalRows arrive ready to display.
  useEffect(() => {
    if (externalRows !== undefined) setLoading(false);
  }, [externalRows]);

  // Server-side pagination fetch (only used when no tid, no did, and no externalRows).
  const loadQuizzers = useCallback((p: number, ps: number) => {
    if (externalRows !== undefined || tid !== undefined || did !== undefined) return;
    UserAPI.get(p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setApiTotalCount(result.count);
        setQuizzers(result.items);
      })
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setLoading(false));
  }, [externalRows, tid, did]);

  useEffect(() => {
    loadQuizzers(0, pageSizeRef.current);
  }, []);

  // Client-side data: externalRows takes priority, then the enriched tournament/division rows.
  // Enriched rows arrive already sorted by name from the backend query.
  const clientItems: UserTS[] | undefined = externalRows ?? enrichedRows;

  // Slice for the current page when all data is loaded; otherwise use the server-fetched page.
  const rows = clientItems !== undefined
    ? clientItems.slice(page * pageSize, (page + 1) * pageSize)
    : quizzers;

  const totalCount = clientItems !== undefined ? clientItems.length : apiTotalCount;

  const handlePageChange = useCallback((newPage: number) => {
    if (clientItems !== undefined) {
      setPage(newPage);
    } else {
      loadQuizzers(newPage, pageSize);
    }
  }, [clientItems, pageSize, loadQuizzers]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPage(0);
    setPageSize(newSize);
    if (clientItems === undefined) {
      loadQuizzers(0, newSize);
    }
  }, [clientItems, loadQuizzers]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    if (onDelete) return onDelete(row);
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, [onDelete]);

  return (
    <DataTableTemplate<UserTS>
      loading={loading}
      entityLabel="Quizzer"
      createLabel={createLabel}
      onCreate={onAdd}
      showCreateButton={!!onAdd}
      showDeleteButton={!!onDelete}
      columns={quizzerColumns(showSensitiveColumns, showAuditColumns, tid !== undefined || did !== undefined)}
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
