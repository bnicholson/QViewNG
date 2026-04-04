import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type GameWithNamesTS } from '../features/UserAPI';

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

function onHover(e: React.MouseEvent<HTMLElement>, enter: boolean) {
  e.currentTarget.style.textDecoration = enter ? 'underline' : 'none';
}

const columns: ColumnDef<GameWithNamesTS>[] = [
  {
    header: 'Tournament',
    render: (g) => (
      <Link
        to={`/tournament/${g.tournamentid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {g.tournament_name}
      </Link>
    ),
  },
  {
    header: 'Date(s)',
    render: (g) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>
        {formatDateRange(g.tournament_fromdate, g.tournament_todate)}
      </span>
    ),
  },
  {
    header: 'Left Team',
    render: (g) => (
      <Link
        to={`/team/${g.leftteamid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {g.left_team_name}
      </Link>
    ),
  },
  {
    header: 'Right Team',
    render: (g) => (
      <Link
        to={`/team/${g.rightteamid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {g.right_team_name}
      </Link>
    ),
  },
  {
    header: 'Center Team',
    render: (g) =>
      g.centerteamid ? (
        <Link
          to={`/team/${g.centerteamid}/overview`}
          style={linkStyle()}
          onMouseEnter={(e) => onHover(e, true)}
          onMouseLeave={(e) => onHover(e, false)}
        >
          {g.center_team_name ?? g.centerteamid}
        </Link>
      ) : (
        <span style={{ color: '#9ca3af' }}>—</span>
      ),
  },
  {
    header: 'Ruleset',
    render: (g) => g.ruleset,
  },
];

export default function UserGamesAsContentJudgeTable({
  userId,
  showCreateButton = false,
  showDeleteButton = false,
}: {
  userId: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}) {
  const [games, setGames] = useState<GameWithNamesTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadGames = useCallback((p: number, ps: number) => {
    UserAPI.getGamesAsContentJudge(userId, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setGames(result);
      })
      .catch(() => console.error('Failed to load content judge games'));
  }, [userId]);

  useEffect(() => { loadGames(0, pageSizeRef.current); }, [userId]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGames(newPage, pageSize);
  }, [pageSize, loadGames]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setGames((prev) => prev.slice(0, newSize));
    } else {
      loadGames(0, newSize);
    }
  }, [pageSize, page, loadGames]);

  const handleDelete = useCallback(async (_row: GameWithNamesTS): Promise<void> => {
    // Delete not implemented for user-role game view
  }, []);

  return (
    <DataTableTemplate<GameWithNamesTS>
      key={userId}
      entityLabel="Game"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={columns}
      rows={games}
      totalCount={totalCount}
      getId={(g) => g.gid}
      onDelete={handleDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={handlePageChange}
      onPageSizeChange={handlePageSizeChange}
    />
  );
}
