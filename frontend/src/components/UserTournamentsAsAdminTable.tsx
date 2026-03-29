import { useState, useCallback, useEffect, useRef } from 'react';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type TournamentForUserTS } from '../features/UserAPI';

function formatDateRange(from: string, to: string): string {
  const fmt = (s: string) =>
    new Date(s).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
  const f = fmt(from);
  const t = fmt(to);
  return f === t ? f : `${f} – ${t}`;
}

function linkStyle(): React.CSSProperties {
  return { color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' };
}

function onHover(e: React.MouseEvent<HTMLAnchorElement>, enter: boolean) {
  e.currentTarget.style.textDecoration = enter ? 'underline' : 'none';
}

const columns: ColumnDef<TournamentForUserTS>[] = [
  {
    header: 'Tournament',
    render: (t) => (
      <a
        href={`/tournament/${t.tid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {t.tname}
      </a>
    ),
  },
  {
    header: 'Date(s)',
    render: (t) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>
        {formatDateRange(t.fromdate, t.todate)}
      </span>
    ),
  },
  {
    header: 'Organization',
    render: (t) => t.organization,
  },
  {
    header: 'Location',
    render: (t) => {
      const parts = [t.venue, t.city, t.region, t.country].filter(Boolean);
      return <span style={{ color: '#374151' }}>{parts.join(', ')}</span>;
    },
  },
];

export default function UserTournamentsAsAdminTable({
  userId,
  showCreateButton = false,
  showDeleteButton = false,
}: {
  userId: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}) {
  const [tournaments, setTournaments] = useState<TournamentForUserTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTournaments = useCallback((p: number, ps: number) => {
    UserAPI.getTournamentsAsAdmin(userId, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setTournaments(result);
      })
      .catch(() => console.error('Failed to load admin tournaments'));
  }, [userId]);

  useEffect(() => { loadTournaments(0, pageSizeRef.current); }, [userId]);

  const handlePageChange = useCallback((newPage: number) => {
    loadTournaments(newPage, pageSize);
  }, [pageSize, loadTournaments]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setTournaments((prev) => prev.slice(0, newSize));
    } else {
      loadTournaments(0, newSize);
    }
  }, [pageSize, page, loadTournaments]);

  const handleDelete = useCallback(async (_row: TournamentForUserTS): Promise<void> => {
    // Delete not implemented for user-role tournament view
  }, []);

  return (
    <DataTableTemplate<TournamentForUserTS>
      key={userId}
      entityLabel="Tournament"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={columns}
      rows={tournaments}
      totalCount={totalCount}
      getId={(t) => t.tid}
      onDelete={handleDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={handlePageChange}
      onPageSizeChange={handlePageSizeChange}
    />
  );
}
