import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type TournamentForUserTS } from '../features/UserAPI';

function formatDateRange(from: string, to: string): string {
  const fmt = (s: string) =>
    new Date(s).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
  const f = fmt(from);
  const t = fmt(to);
  return f === t ? f : `${f} – ${t}`;
}

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
}

function linkStyle(): React.CSSProperties {
  return { color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' };
}

function onHover(e: React.MouseEvent<HTMLElement>, enter: boolean) {
  e.currentTarget.style.textDecoration = enter ? 'underline' : 'none';
}

// Audit columns (Created By / Last Modified / Last Modified By) show only on the user's own profile.
function makeColumns(showAuditColumns: boolean): ColumnDef<TournamentForUserTS>[] {
  return [
    {
      header: 'Tournament',
      render: (t) => (
        <Link
          to={`/tournament/${t.tid}/overview`}
          style={linkStyle()}
          onMouseEnter={(e) => onHover(e, true)}
          onMouseLeave={(e) => onHover(e, false)}
        >
          {t.tname}
        </Link>
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
        const parts = [t.venue, t.city, t.state, t.country].filter(Boolean);
        return <span style={{ color: '#374151' }}>{parts.join(', ')}</span>;
      },
    },
    ...(showAuditColumns ? [
      {
        header: 'Created By',
        render: (t: TournamentForUserTS) => <EntityLink to={`/user/${t.creator_id}/overview`}>{t.creator_name}</EntityLink>,
      },
      {
        header: 'Last Modified',
        render: (t: TournamentForUserTS) => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(t.updated_at)}</span>,
      },
      {
        header: 'Last Modified By',
        render: (t: TournamentForUserTS) => <EntityLink to={`/user/${t.last_modified_user_id}/overview`}>{t.last_modified_user_name}</EntityLink>,
      },
    ] : []),
  ];
}

export default function UserTournamentsAsAdminTable({
  userId,
  showCreateButton = false,
  showDeleteButton = false,
  showAuditColumns = false,
}: {
  userId: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
  showAuditColumns?: boolean;
}) {
  const [tournaments, setTournaments] = useState<TournamentForUserTS[]>([]);
  const [loading, setLoading] = useState(true);
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
      .catch(() => console.error('Failed to load admin tournaments'))
      .finally(() => setLoading(false));
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
      loading={loading}
      key={userId}
      entityLabel="Tournament"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={makeColumns(showAuditColumns)}
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
