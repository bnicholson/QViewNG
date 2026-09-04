import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { BoolBadge, DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { GameAPI, type GameTS } from '../features/GameAPI';
import { DivisionAPI } from '../features/DivisionAPI';
import { RoomAPI } from '../features/RoomAPI';
import { RoundAPI } from '../features/RoundAPI';
import { TeamAPI } from '../features/TeamAPI';
import { GameEditorDialog } from './GameEditorDialog';
import { computeRoomRoundSequence } from '../utils/gameRoundSequence';

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

interface LookupMaps {
  divisions: Map<string, string>;
  rooms: Map<string, string>;
  rounds: Map<string, string | null>;
  teams: Map<string, string>;
}

function gameColumns(tid: string, maps: LookupMaps, roomSequence: Map<string, number>, showSensitiveColumns: boolean, showAuditColumns: boolean): ColumnDef<GameTS>[] {
  return [
    {
      header: '',
      render: (g) => (
        <Link
          to={`/game/${g.gid}/overview`}
          style={{ textDecoration: 'none' }}
        >
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
      render: (g) => maps.divisions.get(g.divisionid) ?? g.divisionid,
    },
    {
      header: 'Room',
      render: (g) => maps.rooms.get(g.roomid) ?? g.roomid,
    },
    {
      header: 'Round',
      render: (g) => roomSequence.get(g.gid) ?? '—',
    },
    {
      header: 'Start Time',
      render: (g) => (
        <span style={{ whiteSpace: 'nowrap' }}>{formatDateTime(maps.rounds.get(g.roundid))}</span>
      ),
    },
    {
      header: 'Left Team',
      render: (g) => maps.teams.get(g.leftteamid) ?? g.leftteamid,
    },
    {
      header: 'Center Team',
      render: (g) => g.centerteamid ? (maps.teams.get(g.centerteamid) ?? g.centerteamid) : '—',
    },
    {
      header: 'Right Team',
      render: (g) => maps.teams.get(g.rightteamid) ?? g.rightteamid,
    },
    ...(showSensitiveColumns ? [{
      header: 'Ignore',
      render: (g: GameTS) => <BoolBadge value={g.ignore} />,
    }] : []),
    ...(showAuditColumns ? [
      {
        header: 'Created',
        render: (g: GameTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.created_at)}</span>
        ),
      },
      {
        header: 'Last Modified',
        render: (g: GameTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.updated_at)}</span>
        ),
      },
    ] : []),
  ];
}

export default function GamesTable({ tid, did, roundid, roomid, showCreateButton = true, showDeleteButton = true, showSensitiveColumns = false, showAuditColumns = true }: { tid: string; did?: string; roundid?: string; roomid?: string; showCreateButton?: boolean; showDeleteButton?: boolean; showSensitiveColumns?: boolean; showAuditColumns?: boolean }) {
  const [games, setGames] = useState<GameTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [maps, setMaps] = useState<LookupMaps>({
    divisions: new Map(),
    rooms: new Map(),
    rounds: new Map(),
    teams: new Map(),
  });
  // game id -> its 1-based ordinal among its room's games (by start time). Built
  // from ALL of the tournament's games so it's correct regardless of the page shown.
  const [roomSequence, setRoomSequence] = useState<Map<string, number>>(new Map());
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadGames = useCallback((p: number, ps: number) => {
    const gamesPromise = roundid
      ? GameAPI.getByRound(roundid, p, ps).then(items => ({ items, count: null as null | number }))
      : roomid
        ? GameAPI.getByRoom(roomid, p, ps).then(items => ({ items, count: null as null | number }))
        : did
          ? GameAPI.getByDivision(did, p, ps).then(items => ({ items, count: null as null | number }))
          : GameAPI.getByTournament(tid, p, ps).then(r => ({ items: r.items, count: r.count }));

    Promise.all([
      gamesPromise,
      DivisionAPI.get(0, 100),
      RoomAPI.get(0, 100),
      RoundAPI.get(0, 200),
      TeamAPI.get(0, 200),
      // All of the tournament's games, used only to number each game within its room.
      GameAPI.getByTournament(tid, 0, 1000),
    ])
      .then(([gameResult, divResult, roomResult, roundResult, teamResult, allGamesResult]) => {
        setPage(p);
        setPageSize(ps);
        const { items, count } = gameResult;
        setTotalCount(count ?? (items.length < ps ? p * ps + items.length : (p + 2) * ps));
        setGames(items);
        const roundStartById = new Map(roundResult.items.map(r => [r.roundid, r.scheduled_start_time]));
        setMaps({
          divisions: new Map(divResult.items.map(d => [d.did, d.dname])),
          rooms: new Map(roomResult.items.map(r => [r.roomid, r.name])),
          rounds: roundStartById,
          teams: new Map(teamResult.items.map(t => [t.teamid, t.name])),
        });
        setRoomSequence(computeRoomRoundSequence(allGamesResult.items, roundStartById));
      })
      .catch(() => console.error('Failed to load games'))
      .finally(() => setLoading(false));
  }, [tid, did, roundid, roomid]);

  useEffect(() => {
    loadGames(0, pageSizeRef.current);
  }, [tid, did, roundid, roomid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGames(newPage, pageSize);
  }, [pageSize, loadGames]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setGames(prev => prev.slice(0, newSize));
    } else {
      loadGames(0, newSize);
    }
  }, [pageSize, page, loadGames]);

  const handleDelete = useCallback(async (row: GameTS): Promise<void> => {
    await GameAPI.delete(row.gid);
    setGames(prev => prev.filter(g => g.gid !== row.gid));
  }, []);

  const handleSave = useCallback((_game: GameTS): void => {
    setEditorIsOpen(false);
    loadGames(page, pageSize);
  }, [loadGames, page, pageSize]);

  return (
    <>
      <DataTableTemplate<GameTS>
        key={roundid ?? roomid ?? did ?? tid}
        entityLabel="Game"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        loading={loading}
        columns={gameColumns(tid, maps, roomSequence, showSensitiveColumns, showAuditColumns)}
        rows={games}
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
