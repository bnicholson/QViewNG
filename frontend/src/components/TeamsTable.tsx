import { useState, useCallback, useEffect } from 'react';
import { DataTableTemplate, type ColumnDef } from './DataTableTemplate';
import { TeamAPI, type TeamTS } from '../features/TeamAPI';
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI';
import { UserAPI, type UserTS } from '../features/UserAPI';
import { TeamEditorDialog } from './TeamEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function teamColumns(
  tid: string,
  divisionMap: Map<string, string>,
  coachMap: Map<string, string>,
): ColumnDef<TeamTS>[] {
  return [
    {
      header: 'Name',
      render: (t) => (
        <a
          href={`/tournament/${tid}/team/${t.teamid}`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {t.name}
        </a>
      ),
    },
    {
      header: 'Division',
      render: (t) => divisionMap.get(t.did) ?? t.did,
    },
    {
      header: 'Coach',
      render: (t) => coachMap.get(t.coachid) ?? t.coachid,
    },
    {
      header: 'Created',
      render: (t) => (
        <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(t.created_at)}</span>
      ),
    },
    {
      header: 'Last Modified',
      render: (t) => (
        <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(t.updated_at)}</span>
      ),
    },
  ];
}

const TEAMS_PAGE = 0;
const TEAMS_PAGE_SIZE = 50;

export default function TeamsTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState(false);
  const [teams, setTeams] = useState<TeamTS[]>([]);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [coachMap, setCoachMap] = useState<Map<string, string>>(new Map());
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadTeams = () => {
    setIsLoading(true);
    Promise.all([
      TeamAPI.get(TEAMS_PAGE, TEAMS_PAGE_SIZE),
      DivisionAPI.get(0, 100),
      UserAPI.get(0, 200),
    ])
      .then(([teamResult, divisionResult, userResult]: [TeamTS[], DivisionTS[], UserTS[]]) => {
        setTeams(teamResult);
        setDivisionMap(new Map(divisionResult.map(d => [d.did, d.dname])));
        setCoachMap(new Map(userResult.map(u => [
          u.id,
          [u.fname, u.mname, u.lname].filter(Boolean).join(' '),
        ])));
      })
      .catch(() => console.error('Failed to load teams'))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => { loadTeams(); }, [tid]);

  const handleDelete = useCallback(async (row: TeamTS): Promise<void> => {
    await TeamAPI.delete(row.teamid);
    setTeams((prev) => prev.filter((t) => t.teamid !== row.teamid));
  }, []);

  const handleSave = useCallback((_team: TeamTS): void => {
    setEditorIsOpen(false);
    loadTeams();
  }, []);

  if (isLoading) return <div>Loading teams...</div>;

  return (
    <>
      <DataTableTemplate<TeamTS>
        entityLabel="Team"
        onCreate={() => setEditorIsOpen(true)}
        columns={teamColumns(tid, divisionMap, coachMap)}
        rows={teams}
        getId={(t) => t.teamid}
        onDelete={handleDelete}
      />
      <TeamEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
