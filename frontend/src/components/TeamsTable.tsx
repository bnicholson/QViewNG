import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { TeamAPI, type TeamTS, type TeamWithCoachTS } from '../features/TeamAPI';
import { DivisionAPI } from '../features/DivisionAPI';
import { UserAPI } from '../features/UserAPI';
import { TeamEditorDialog } from './TeamEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function teamColumns(
  divisionMap: Map<string, string>,
): ColumnDef<TeamWithCoachTS>[] {
  return [
    {
      header: 'Name',
      render: (t) => (
        <Link
          to={`/team/${t.teamid}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {t.name}
        </Link>
      ),
    },
    {
      header: 'Division',
      render: (t) => divisionMap.get(t.did) ?? t.did,
    },
    {
      header: 'Coach',
      render: (t) => (
        <Link
          to={`/user/${t.coachid}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {t.coach_name}
        </Link>
      ),
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

export default function TeamsTable({ tid, did, showCreateButton = true, showDeleteButton = true }: { tid: string; did?: string; showCreateButton?: boolean; showDeleteButton?: boolean }) {
  const [teams, setTeams] = useState<TeamWithCoachTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    if (did) {
      Promise.all([
        TeamAPI.getByDivision(did, p, ps),
        DivisionAPI.getByTournament(tid, 0, 100),
        UserAPI.get(0, 500),
      ])
        .then(([teamItems, divisionResult, userResult]) => {
          const userMap = new Map(userResult.items.map(u => [u.id, [u.fname, u.mname, u.lname].filter(Boolean).join(' ')]));
          const enriched: TeamWithCoachTS[] = teamItems.map(t => ({ ...t, coach_name: userMap.get(t.coachid) ?? t.coachid }));
          setPage(p);
          setPageSize(ps);
          setTotalCount(teamItems.length < ps ? p * ps + teamItems.length : (p + 2) * ps);
          setTeams(enriched);
          setDivisionMap(new Map(divisionResult.map(d => [d.did, d.dname])));
        })
        .catch(() => console.error('Failed to load teams'));
    } else {
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
    }
  }, [tid, did]);

  useEffect(() => {
    loadTeams(0, pageSizeRef.current);
  }, [tid, did]);

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
        key={did ?? tid}
        entityLabel="Team"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={teamColumns(divisionMap)}
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
