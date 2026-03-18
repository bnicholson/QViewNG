import { useState, useCallback, useEffect, useRef } from 'react';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
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

export default function TeamsTable({ tid }: { tid: string }) {
  const [teams, setTeams] = useState<TeamTS[]>([]);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [coachMap, setCoachMap] = useState<Map<string, string>>(new Map());
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    Promise.all([
      TeamAPI.get(p, ps),
      DivisionAPI.get(0, 100),
      UserAPI.get(0, 200),
    ])
      .then(([teamResult, divisionResult, userResult]: [TeamTS[], DivisionTS[], UserTS[]]) => {
        setPage(p);
        setPageSize(ps);
        setTeams(teamResult);
        setDivisionMap(new Map(divisionResult.map(d => [d.did, d.dname])));
        setCoachMap(new Map(userResult.map(u => [
          u.id,
          [u.fname, u.mname, u.lname].filter(Boolean).join(' '),
        ])));
      })
      .catch(() => console.error('Failed to load teams'));
  }, [tid]);

  useEffect(() => {
    loadTeams(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadTeams(newPage, pageSize);
  }, [pageSize, loadTeams]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setTeams(prev => prev.slice(0, newSize));
    } else {
      loadTeams(0, newSize);
    }
  }, [pageSize, page, loadTeams]);

  const handleDelete = useCallback(async (row: TeamTS): Promise<void> => {
    await TeamAPI.delete(row.teamid);
    setTeams((prev) => prev.filter((t) => t.teamid !== row.teamid));
  }, []);

  const handleSave = useCallback((_team: TeamTS): void => {
    setEditorIsOpen(false);
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize]);

  const totalCount = teams.length < pageSize
    ? page * pageSize + teams.length
    : (page + 2) * pageSize;

  return (
    <>
      <DataTableTemplate<TeamTS>
        key={tid}
        entityLabel="Team"
        onCreate={() => setEditorIsOpen(true)}
        columns={teamColumns(tid, divisionMap, coachMap)}
        rows={teams}
        totalCount={totalCount}
        getId={(t) => t.teamid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
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
