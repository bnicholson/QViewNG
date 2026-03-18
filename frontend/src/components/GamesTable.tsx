import { useState, useCallback, useEffect } from 'react';
import { BoolBadge, DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { GameAPI, type GameTS } from '../features/GameAPI';
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI';
import { RoomAPI, type RoomTS } from '../features/RoomAPI';
import { RoundAPI, type RoundTS } from '../features/RoundAPI';
import { TeamAPI, type TeamTS } from '../features/TeamAPI';
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

interface LookupMaps {
  divisions: Map<string, string>;
  rooms: Map<string, string>;
  rounds: Map<string, string | null>;
  teams: Map<string, string>;
}

function gameColumns(tid: string, maps: LookupMaps): ColumnDef<GameTS>[] {
  return [
    {
      header: 'Division',
      render: (g) => maps.divisions.get(g.divisionid) ?? g.divisionid,
    },
    {
      header: 'Round',
      render: (g) => (
        <a
          href={`/tournament/${tid}/game/${g.gid}`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {formatDateTime(maps.rounds.get(g.roundid))}
        </a>
      ),
    },
    {
      header: 'Room',
      render: (g) => maps.rooms.get(g.roomid) ?? g.roomid,
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
    {
      header: 'Ruleset',
      render: (g) => g.ruleset,
    },
    {
      header: 'Ignore',
      render: (g) => <BoolBadge value={g.ignore} />,
    },
    {
      header: 'Created',
      render: (g) => (
        <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.created_at)}</span>
      ),
    },
    {
      header: 'Last Modified',
      render: (g) => (
        <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.updated_at)}</span>
      ),
    },
  ];
}

const GAMES_PAGE = 0;
const GAMES_PAGE_SIZE = 50;

export default function GamesTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState(false);
  const [games, setGames] = useState<GameTS[]>([]);
  const [maps, setMaps] = useState<LookupMaps>({
    divisions: new Map(),
    rooms: new Map(),
    rounds: new Map(),
    teams: new Map(),
  });
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadGames = () => {
    setIsLoading(true);
    Promise.all([
      GameAPI.get(GAMES_PAGE, GAMES_PAGE_SIZE),
      DivisionAPI.get(0, 100),
      RoomAPI.get(0, 100),
      RoundAPI.get(0, 200),
      TeamAPI.get(0, 200),
    ])
      .then(([gameResult, divResult, roomResult, roundResult, teamResult]:
        [GameTS[], DivisionTS[], RoomTS[], RoundTS[], TeamTS[]]) => {
        setGames(gameResult);
        setMaps({
          divisions: new Map(divResult.map(d => [d.did, d.dname])),
          rooms: new Map(roomResult.map(r => [r.roomid, r.name])),
          rounds: new Map(roundResult.map(r => [r.roundid, r.scheduled_start_time])),
          teams: new Map(teamResult.map(t => [t.teamid, t.name])),
        });
      })
      .catch(() => console.error('Failed to load games'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => { loadGames(); }, [tid]);

  const handleDelete = useCallback(async (row: GameTS): Promise<void> => {
    await GameAPI.delete(row.gid);
    setGames(prev => prev.filter(g => g.gid !== row.gid));
  }, []);

  const handleSave = useCallback((_game: GameTS): void => {
    setEditorIsOpen(false);
    loadGames();
  }, []);

  if (isLoading) return <div>Loading games...</div>;

  return (
    <>
      <DataTableTemplate<GameTS>
        entityLabel="Game"
        onCreate={() => setEditorIsOpen(true)}
        columns={gameColumns(tid, maps)}
        rows={games}
        getId={(g) => g.gid}
        onDelete={handleDelete}
      />
      <GameEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
