import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI';
import { TeamEditorDialog, type EditableTeam } from './TeamEditorDialog';
import { useAuth } from '../hooks/useAuth';

function editButtonStyle(): React.CSSProperties {
  return {
    padding: '3px 10px', borderRadius: 5, border: '1px solid #e0e0e0',
    background: 'transparent', color: '#2563eb', fontSize: 12, fontWeight: 600,
    cursor: 'pointer', whiteSpace: 'nowrap',
  };
}

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

function teamColumns(
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: TeamRowTS) => void,
): ColumnDef<TeamRowTS>[] {
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
      {
        header: 'Last Modified By',
        render: (t: TeamRowTS) => (
          <EntityLink to={`/user/${t.last_modified_user_id}/overview`}>{t.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: 'Edit',
      render: (t: TeamRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(t)}>Edit</button>
      ),
    }] : []),
  ];
}

export default function TeamsTable({ tid, did, poolBracketId, showCreateButton = true, showEditButton = false, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: { tid: string; did?: string;
  /** When set, rows are the teams associated with this pool bracket (via its teamgroup), not a
   *  division/tournament's teams. Creating a team here also associates it with the bracket, and the
   *  row delete removes it from the bracket (the team itself is untouched). */
  poolBracketId?: string;
  showCreateButton?: boolean; showEditButton?: boolean; showDeleteButton?: boolean; showAuditColumns?: boolean;
  /** Column headers to omit. Lets a consumer hide a column that's redundant in its context —
   *  e.g. the Division profile hides "Division" since every row is the same division. */
  hiddenColumns?: string[] }) {
  const { accessToken } = useAuth();
  // Current page of enriched rows plus the total count — paginated server-side.
  const [rows, setRows] = useState<TeamRowTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingTeam, setEditingTeam] = useState<EditableTeam | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadTeams = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = poolBracketId
      ? TeamAPI.getRowsByPoolBracket(poolBracketId, p, ps)
      : did
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
  }, [tid, did, poolBracketId]);

  useEffect(() => {
    loadTeams(0, pageSizeRef.current);
  }, [tid, did, poolBracketId]);

  const handlePageChange = useCallback((newPage: number) => {
    loadTeams(newPage, pageSize);
  }, [pageSize, loadTeams]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadTeams(0, newSize);
  }, [loadTeams]);

  const handleDelete = useCallback(async (row: TeamRowTS): Promise<void> => {
    // In a pool-bracket context, "delete" removes the team from the bracket (deletes the
    // association only). Elsewhere it deletes the team itself.
    if (poolBracketId) {
      await TeamAPI.removeFromPoolBracket(poolBracketId, row.teamid, accessToken);
    } else {
      await TeamAPI.delete(row.teamid, accessToken);
    }
    // Reload the current page so the count and page contents stay correct.
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize, poolBracketId, accessToken]);

  const handleCreate = useCallback(() => {
    setEditingTeam(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: TeamRowTS) => {
    setEditingTeam({ teamid: row.teamid, name: row.name, did: row.did, coachid: row.coachid });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback(async (team: TeamTS): Promise<void> => {
    // A team created in a pool-bracket context is associated with the bracket so it shows up here.
    if (poolBracketId && !editingTeam) {
      try {
        await TeamAPI.addToPoolBracket(poolBracketId, team.teamid, accessToken);
      } catch {
        console.error('Team was created but could not be added to the pool bracket');
      }
    }
    setEditorIsOpen(false);
    setEditingTeam(null);
    loadTeams(page, pageSize);
  }, [loadTeams, page, pageSize, poolBracketId, editingTeam, accessToken]);

  return (
    <>
      <DataTableTemplate<TeamRowTS>
        loading={loading}
        key={did ?? tid}
        entityLabel="Team"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={teamColumns(showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
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
        team={editingTeam}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingTeam(null); }}
        onSave={handleSave}
      />
    </>
  );
}
