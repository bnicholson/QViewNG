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
  // externalRows (a fixed roster) paginate client-side; everything else paginates server-side.
  const usesExternal = externalRows !== undefined;
  const usesEnriched = tid !== undefined || did !== undefined;

  // Current page of rows (server/enriched modes) plus the total count.
  const [pageRows, setPageRows] = useState<UserTS[]>([]);
  const [serverCount, setServerCount] = useState(0);
  const [loading, setLoading] = useState(true);

  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Fetch one page from the server: the enriched tournament/division endpoint, or the plain
  // users endpoint when the table is unscoped. Skipped entirely in externalRows mode.
  const loadPage = useCallback((p: number, ps: number) => {
    if (usesExternal) return;
    setLoading(true);
    const request: Promise<{ count: number; items: UserTS[] }> = usesEnriched
      ? (tid !== undefined ? QuizzerAPI.getByTournament(tid, p, ps) : QuizzerAPI.getByDivision(did!, p, ps))
      : UserAPI.get(p, ps);
    request
      .then(({ count, items }) => {
        setPageRows(items);
        setServerCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setLoading(false));
  }, [usesExternal, usesEnriched, tid, did]);

  // (Re)load the first page whenever the scope changes.
  useEffect(() => {
    loadPage(0, pageSizeRef.current);
  }, [tid, did]);

  // externalRows arrive ready to display.
  useEffect(() => {
    if (usesExternal) setLoading(false);
  }, [usesExternal, externalRows]);

  const rows = usesExternal
    ? externalRows!.slice(page * pageSize, (page + 1) * pageSize)
    : pageRows;

  const totalCount = usesExternal ? externalRows!.length : serverCount;

  const handlePageChange = useCallback((newPage: number) => {
    if (usesExternal) {
      setPage(newPage);
    } else {
      loadPage(newPage, pageSize);
    }
  }, [usesExternal, pageSize, loadPage]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (usesExternal) {
      setPage(0);
      setPageSize(newSize);
    } else {
      loadPage(0, newSize);
    }
  }, [usesExternal, loadPage]);

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
