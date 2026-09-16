import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { GameAPI, type GameTS, type GameRowTS } from '../features/GameAPI';
import { GameEditorDialog } from './GameEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: 'numeric', minute: '2-digit',
  });
}

function gameColumns(showAuditColumns: boolean): ColumnDef<GameRowTS>[] {
  return [
    {
      header: '',
      render: (g) => (
        <Link to={`/game/${g.gid}/overview`} style={{ textDecoration: 'none' }}>
          <button style={{
            padding: '2px 10px',
            fontSize: '0.75rem',
            cursor: 'pointer',
            borderRadius: '4px',
            border: '1px solid #d1d5db',
            background: '#f9fafb',
            whiteSpace: 'nowrap',
          }}>
            View
          </button>
        </Link>
      ),
    },
    {
      header: 'Division',
      render: (g) => <EntityLink to={`/division/${g.divisionid}/overview`}>{g.division_name || g.divisionid}</EntityLink>,
    },
    {
      header: 'Room',
      render: (g) => <EntityLink to={`/room/${g.roomid}/overview`}>{g.room_name || g.roomid}</EntityLink>,
    },
    {
      header: 'Round',
      render: (g) => <EntityLink to={`/round/${g.roundid}/overview`}>{g.round_number ?? '—'}</EntityLink>,
    },
    {
      header: 'Start Time',
      render: (g) => (
        <span style={{ whiteSpace: 'nowrap' }}>{formatDateTime(g.scheduled_start_time)}</span>
      ),
    },
    {
      header: 'Left Team',
      render: (g) => <EntityLink to={`/team/${g.leftteamid}/overview`}>{g.left_team_name || g.leftteamid}</EntityLink>,
    },
    {
      header: 'Center Team',
      render: (g) => g.centerteamid ? <EntityLink to={`/team/${g.centerteamid}/overview`}>{g.center_team_name || g.centerteamid}</EntityLink> : '—',
    },
    {
      header: 'Right Team',
      render: (g) => <EntityLink to={`/team/${g.rightteamid}/overview`}>{g.right_team_name || g.rightteamid}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: 'Created',
        render: (g: GameRowTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.created_at)}</span>
        ),
      },
      {
        header: 'Last Modified',
        render: (g: GameRowTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.updated_at)}</span>
        ),
      },
      {
        header: 'Last Modified By',
        render: (g: GameRowTS) => (
          <EntityLink to={`/user/${g.last_modified_user_id}/overview`}>{g.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
  ];
}

export default function GamesTable({ tid, did, roundid, roomid, poolbracketid, roundgroupId, showCreateButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: { tid: string; did?: string; roundid?: string; roomid?: string;
  /** When set, rows are the games whose poolbracket_id matches this pool bracket. */
  poolbracketid?: string;
  /** When set, rows are the games whose round belongs to this roundgroup. */
  roundgroupId?: string;
  showCreateButton?: boolean; showDeleteButton?: boolean; showAuditColumns?: boolean;
  /** Column headers to omit — lets a consumer hide a column that's redundant in its context
   *  (e.g. the Round profile hides "Round", the Room profile hides "Room"). */
  hiddenColumns?: string[] }) {
  // Current page of enriched rows plus the total count — paginated server-side, one call per page.
  const [rows, setRows] = useState<GameRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadGames = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = roundgroupId
      ? GameAPI.getRowsByRoundGroup(roundgroupId, p, ps)
      : poolbracketid
        ? GameAPI.getRowsByPoolBracket(poolbracketid, p, ps)
        : roundid
          ? GameAPI.getRowsByRound(roundid, p, ps)
          : roomid
            ? GameAPI.getRowsByRoom(roomid, p, ps)
            : did
              ? GameAPI.getRowsByDivision(did, p, ps)
              : GameAPI.getRowsByTournament(tid, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error('Failed to load games'))
      .finally(() => setLoading(false));
  }, [tid, did, roundid, roomid, poolbracketid, roundgroupId]);

  useEffect(() => {
    loadGames(0, pageSizeRef.current);
  }, [tid, did, roundid, roomid, poolbracketid, roundgroupId]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGames(newPage, pageSize);
  }, [pageSize, loadGames]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadGames(0, newSize);
  }, [loadGames]);

  const handleDelete = useCallback(async (row: GameRowTS): Promise<void> => {
    await GameAPI.delete(row.gid);
    // Reload the current page so the count and page contents stay correct.
    loadGames(page, pageSize);
  }, [loadGames, page, pageSize]);

  const handleSave = useCallback((_game: GameTS): void => {
    setEditorIsOpen(false);
    loadGames(page, pageSize);
  }, [loadGames, page, pageSize]);

  return (
    <>
      <DataTableTemplate<GameRowTS>
        key={roundid ?? roomid ?? did ?? tid}
        entityLabel="Game"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        loading={loading}
        columns={gameColumns(showAuditColumns).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(g) => g.gid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <GameEditorDialog
        tid={tid}
        lockedDivisionId={did}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
