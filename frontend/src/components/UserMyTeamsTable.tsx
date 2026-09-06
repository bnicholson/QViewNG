import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTeamRowTS } from '../features/UserAPI';

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
  (e.currentTarget as HTMLElement).style.textDecoration = enter ? 'underline' : 'none';
}

// Audit columns (Created By / Last Modified / Last Modified By) show only on the user's own profile.
function makeColumns(showAuditColumns: boolean): ColumnDef<UserTeamRowTS>[] {
  return [
    {
      header: 'Tournament',
      render: (r) => (
        <Link to={`/tournament/${r.tournament_id}/overview`} style={linkStyle()} onMouseEnter={(e) => onHover(e, true)} onMouseLeave={(e) => onHover(e, false)}>
          {r.tournament_name}
        </Link>
      ),
    },
    {
      header: 'Date(s)',
      render: (r) => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDateRange(r.tournament_fromdate, r.tournament_todate)}</span>,
    },
    {
      header: 'Team',
      render: (r) => (
        <Link to={`/team/${r.teamid}/overview`} style={linkStyle()} onMouseEnter={(e) => onHover(e, true)} onMouseLeave={(e) => onHover(e, false)}>
          {r.name}
        </Link>
      ),
    },
    {
      header: 'Division',
      render: (r) => (
        <Link to={`/division/${r.did}/overview`} style={linkStyle()} onMouseEnter={(e) => onHover(e, true)} onMouseLeave={(e) => onHover(e, false)}>
          {r.division_name}
        </Link>
      ),
    },
    {
      header: 'Role',
      render: (r) => <span style={{ whiteSpace: 'nowrap' }}>{r.role}</span>,
    },
    {
      header: 'Coach',
      render: (r) => (
        <Link to={`/user/${r.coachid}/overview`} style={linkStyle()} onMouseEnter={(e) => onHover(e, true)} onMouseLeave={(e) => onHover(e, false)}>
          {r.coach_name}
        </Link>
      ),
    },
    {
      header: 'Quizzers',
      render: (r) => r.quizzers.length === 0 ? (
        <span style={{ color: '#6b7280' }}>—</span>
      ) : (
        <span>
          {r.quizzers.map((q, i) => (
            <span key={q.id}>
              {i > 0 && ', '}
              <Link to={`/user/${q.id}/overview`} style={linkStyle()} onMouseEnter={(e) => onHover(e, true)} onMouseLeave={(e) => onHover(e, false)}>
                {q.name}
              </Link>
            </span>
          ))}
        </span>
      ),
    },
    ...(showAuditColumns ? [
      {
        header: 'Created By',
        render: (r: UserTeamRowTS) => <EntityLink to={`/user/${r.creator_id}/overview`}>{r.creator_name}</EntityLink>,
      },
      {
        header: 'Last Modified',
        render: (r: UserTeamRowTS) => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(r.updated_at)}</span>,
      },
      {
        header: 'Last Modified By',
        render: (r: UserTeamRowTS) => <EntityLink to={`/user/${r.last_modified_user_id}/overview`}>{r.last_modified_user_name}</EntityLink>,
      },
    ] : []),
  ];
}

export default function UserMyTeamsTable({
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
  const [rows, setRows] = useState<UserTeamRowTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [loading, setLoading] = useState(true);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Single scoped call to the enriched, paginated endpoint.
  const load = useCallback((p: number, ps: number) => {
    setLoading(true);
    UserAPI.getTeamRows(userId, p, ps)
      .then(({ count, items }) => { setPage(p); setPageSize(ps); setTotalCount(count); setRows(items); })
      .catch(() => console.error('Failed to load my teams'))
      .finally(() => setLoading(false));
  }, [userId]);

  useEffect(() => { load(0, pageSizeRef.current); }, [userId, load]);

  return (
    <DataTableTemplate<UserTeamRowTS>
      loading={loading}
      key={userId}
      entityLabel="Team"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={makeColumns(showAuditColumns)}
      rows={rows}
      totalCount={totalCount}
      getId={(r) => r.teamid}
      onDelete={async () => {}}
      page={page}
      pageSize={pageSize}
      onPageChange={(p) => load(p, pageSize)}
      onPageSizeChange={(ps) => load(0, ps)}
    />
  );
}
