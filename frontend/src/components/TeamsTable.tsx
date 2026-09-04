import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI';
import { TeamEditorDialog } from './TeamEditorDialog';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function teamColumns(showAuditColumns: boolean): ColumnDef<TeamRowTS>[] {
  return [
    {
      header: 'Division',
      render: (t) => <EntityLink to={`/division/${t.did}/overview`}>{t.division_name || t.did}</EntityLink>,
    },
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
      header: 'Coach',
      render: (t) => <EntityLink to={`/user/${t.coachid}/overview`}>{t.coach_name}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: 'Created',
        render: (t: TeamRowTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(t.created_at)}</span>
        ),
      },
      {
        header: 'Last Modified',
        render: (t: TeamRowTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(t.updated_at)}</span>
        ),
      },
    ] : []),
  ];
}

export default function TeamsTable({ tid, did, showCreateButton = true, showDeleteButton = true, showAuditColumns = true }: { tid: string; did?: string; showCreateButton?: boolean; showDeleteButton?: boolean; showAuditColumns?: boolean }) {
  // Current page of enriched rows plus the total count — paginated server-side.
  const [rows, setRows] = useState<TeamRowTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = did
      ? TeamAPI.getRowsByDivision(did, p, ps)
      : TeamAPI.getRowsByTournament(tid, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error('Failed to load teams'))
      .finally(() => setLoading(false));
  }, [tid, did]);

  useEffect(() => {
    loadTeams(0, pageSizeRef.current);
  }, [tid, did]);

  const handlePageChange = useCallback((newPage: number) => {
    loadTeams(newPage, pageSize);
  }, [pageSize, loadTeams]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadTeams(0, newSize);
  }, [loadTeams]);

  const handleDelete = useCallback(async (row: TeamRowTS): Promise<void> => {
    await TeamAPI.delete(row.teamid);
    // Reload the current page so the count and page contents stay correct.
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize]);

  const handleSave = useCallback((_team: TeamTS): void => {
    setEditorIsOpen(false);
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize]);

  return (
    <>
      <DataTableTemplate<TeamRowTS>
        loading={loading}
        key={did ?? tid}
        entityLabel="Team"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={teamColumns(showAuditColumns)}
        rows={rows}
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
        lockedDivisionId={did}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
