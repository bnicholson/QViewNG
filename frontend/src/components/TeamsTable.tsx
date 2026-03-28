import { useState, useCallback, useEffect, useRef } from 'react';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { TeamAPI, type TeamTS, type TeamWithCoachTS } from '../features/TeamAPI';
import { DivisionAPI } from '../features/DivisionAPI';
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
): ColumnDef<TeamWithCoachTS>[] {
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
      render: (t) => t.coach_name,
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

export default function TeamsTable({ tid, showCreateButton = true, showDeleteButton = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean }) {
  const [teams, setTeams] = useState<TeamWithCoachTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    Promise.all([
      TeamAPI.getByTournament(tid, p, ps),
      DivisionAPI.getByTournament(tid, 0, 100),
    ])
      .then(([teamResult, divisionResult]) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(teamResult.count);
        setTeams(teamResult.items);
        setDivisionMap(new Map(divisionResult.map(d => [d.did, d.dname])));
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

  const handleDelete = useCallback(async (row: TeamWithCoachTS): Promise<void> => {
    await TeamAPI.delete(row.teamid);
    setTeams((prev) => prev.filter((t) => t.teamid !== row.teamid));
  }, []);

  const handleSave = useCallback((_team: TeamTS): void => {
    setEditorIsOpen(false);
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize]);

  return (
    <>
      <DataTableTemplate<TeamWithCoachTS>
        key={tid}
        entityLabel="Team"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={teamColumns(tid, divisionMap)}
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
