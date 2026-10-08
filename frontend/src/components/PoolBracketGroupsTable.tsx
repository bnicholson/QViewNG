import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { PoolBracketGroupAPI, type PoolBracketGroupTS, type PoolBracketGroupRowTS } from "../features/PoolBracketGroupAPI";
import { PoolBracketGroupEditorDialog } from "./PoolBracketGroupEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function editButtonStyle(): React.CSSProperties {
  return {
    padding: "3px 10px",
    borderRadius: 5,
    border: "1px solid #e0e0e0",
    background: "transparent",
    color: "#2563eb",
    fontSize: 12,
    fontWeight: 600,
    cursor: "pointer",
    whiteSpace: "nowrap",
  };
}

function poolBracketGroupColumns(
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: PoolBracketGroupRowTS) => void,
): ColumnDef<PoolBracketGroupRowTS>[] {
  return [
    {
      header: "Division",
      render: (g) => <EntityLink to={`/division/${g.did}/overview`}>{g.division_name || g.did}</EntityLink>,
    },
    {
      header: "Pool/Bracket Group",
      render: (g) => <EntityLink to={`/pool-bracket-group/${g.poolbracketgroupid}/overview`}>{g.name}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (g: PoolBracketGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(g.created_date)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (g: PoolBracketGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(g.last_modified_date)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (g: PoolBracketGroupRowTS) => (
          <EntityLink to={`/user/${g.last_modified_user_id}/overview`}>{g.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: "Edit",
      render: (g: PoolBracketGroupRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(g)}>Edit</button>
      ),
    }] : []),
  ];
}

/**
 * Data table of poolbracketgroups — the groupings of pools/brackets that run concurrently within a
 * division — with Add / Edit / Delete. A group that still holds pools or brackets can't be deleted;
 * the server's message surfaces on the row's Delete button.
 */
export default function PoolBracketGroupsTable({ tid, did, showCreateButton = true, showEditButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: {
  tid: string;
  /** When set, rows are scoped to this division; otherwise to the whole tournament (`tid`). */
  did?: string;
  showCreateButton?: boolean;
  showEditButton?: boolean;
  showDeleteButton?: boolean;
  showAuditColumns?: boolean;
  /** Column headers to omit — lets a consumer hide a column that's redundant in its context. */
  hiddenColumns?: string[];
}) {
  const [rows, setRows] = useState<PoolBracketGroupRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingGroup, setEditingGroup] = useState<PoolBracketGroupTS | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadGroups = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = did
      ? PoolBracketGroupAPI.getRowsByDivision(did, p, ps)
      : PoolBracketGroupAPI.getRowsByTournament(tid, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load pool/bracket groups"))
      .finally(() => setLoading(false));
  }, [tid, did]);

  useEffect(() => {
    loadGroups(0, pageSizeRef.current);
  }, [tid, did]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGroups(newPage, pageSize);
  }, [pageSize, loadGroups]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadGroups(0, newSize);
  }, [loadGroups]);

  const handleDelete = useCallback(async (row: PoolBracketGroupRowTS): Promise<void> => {
    await PoolBracketGroupAPI.delete(row.poolbracketgroupid);
    loadGroups(page, pageSize);
  }, [loadGroups, page, pageSize]);

  const handleCreate = useCallback(() => {
    setEditingGroup(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: PoolBracketGroupRowTS) => {
    setEditingGroup({
      poolbracketgroupid: row.poolbracketgroupid,
      divisionid: row.did,
      name: row.name,
      created_date: row.created_date,
      creator_userid: "",
      last_modified_date: row.last_modified_date,
      last_modified_userid: row.last_modified_user_id,
    });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback((_group: PoolBracketGroupTS): void => {
    setEditorIsOpen(false);
    setEditingGroup(null);
    loadGroups(page, pageSize);
  }, [loadGroups, page, pageSize]);

  return (
    <>
      <DataTableTemplate<PoolBracketGroupRowTS>
        loading={loading}
        key={did ?? tid}
        entityLabel="Pool/Bracket Group"
        title="Pool/Bracket Groups"
        createLabel="Create Pool/Bracket Group"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={poolBracketGroupColumns(showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(g) => g.poolbracketgroupid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <PoolBracketGroupEditorDialog
        tid={tid}
        lockedDivisionId={did}
        group={editingGroup}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingGroup(null); }}
        onSave={handleSave}
      />
    </>
  );
}
