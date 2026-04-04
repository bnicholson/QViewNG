import { useState, useCallback, useEffect, useRef } from 'react';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type TeamWithTournamentInfoTS } from '../features/UserAPI';
import { Link } from 'react-router-dom';

function formatDateRange(from: string, to: string): string {
  const fmt = (s: string) =>
    new Date(s).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
  const f = fmt(from);
  const t = fmt(to);
  return f === t ? f : `${f} – ${t}`;
}

function linkStyle(extra?: React.CSSProperties): React.CSSProperties {
  return {
    color: '#2563eb',
    textDecoration: 'none',
    fontWeight: 500,
    whiteSpace: 'nowrap',
    ...extra,
  };
}

function onHover(e: React.MouseEvent<HTMLAnchorElement>, enter: boolean) {
  e.currentTarget.style.textDecoration = enter ? 'underline' : 'none';
}

const columns: ColumnDef<TeamWithTournamentInfoTS>[] = [
  {
    header: 'Tournament',
    render: (t) => (
      <Link
        to={`/tournament/${t.tournament_id}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {t.tournament_name}
      </Link>
    ),
  },
  {
    header: 'Date(s)',
    render: (t) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>
        {formatDateRange(t.tournament_fromdate, t.tournament_todate)}
      </span>
    ),
  },
  {
    header: 'Team',
    render: (t) => (
      <Link
        to={`/team/${t.teamid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {t.name}
      </Link>
    ),
  },
  {
    header: 'Coach',
    render: (t) => (
      <Link
        to={`/user/${t.coachid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {t.coach_name}
      </Link>
    ),
  },
];

export default function UserTeamsAsQuizzerTable({
  userId,
  showCreateButton = false,
  showDeleteButton = false,
}: {
  userId: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}) {
  const [teams, setTeams] = useState<TeamWithTournamentInfoTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    UserAPI.getTeamsAsQuizzer(userId, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setTeams(result);
      })
      .catch(() => console.error('Failed to load quizzer teams'));
  }, [userId]);

  useEffect(() => { loadTeams(0, pageSizeRef.current); }, [userId]);

  const handlePageChange = useCallback((newPage: number) => {
    loadTeams(newPage, pageSize);
  }, [pageSize, loadTeams]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setTeams((prev) => prev.slice(0, newSize));
    } else {
      loadTeams(0, newSize);
    }
  }, [pageSize, page, loadTeams]);

  const handleDelete = useCallback(async (_row: TeamWithTournamentInfoTS): Promise<void> => {
    // Delete not implemented for user-role team view
  }, []);

  return (
    <DataTableTemplate<TeamWithTournamentInfoTS>
      key={userId}
      entityLabel="Team"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={columns}
      rows={teams}
      totalCount={totalCount}
      getId={(t) => t.teamid}
      onDelete={handleDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={handlePageChange}
      onPageSizeChange={handlePageSizeChange}
    />
  );
}
