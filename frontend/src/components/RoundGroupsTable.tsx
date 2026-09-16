import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoundGroupAPI, type RoundGroupTS, type RoundGroupRowTS } from "../features/RoundGroupAPI";
import { RoundGroupEditorDialog } from "./RoundGroupEditorDialog";

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

function roundGroupColumns(
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: RoundGroupRowTS) => void,
): ColumnDef<RoundGroupRowTS>[] {
  return [
    {
      header: "Division",
      render: (s) => <EntityLink to={`/division/${s.did}/overview`}>{s.division_name || s.did}</EntityLink>,
    },
    {
      header: "Session",
      render: (s) => <EntityLink to={`/division-session/${s.roundgroup_id}/overview`}>{s.name}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (s: RoundGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(s.created_date)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (s: RoundGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(s.last_modified_date)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (s: RoundGroupRowTS) => (
          <EntityLink to={`/user/${s.last_modified_user_id}/overview`}>{s.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: "Edit",
      render: (s: RoundGroupRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(s)}>Edit</button>
      ),
    }] : []),
  ];
}

export default function RoundGroupsTable({ tid, did, showCreateButton = true, showEditButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: {
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
  const [rows, setRows] = useState<RoundGroupRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingRoundGroup, setEditingRoundGroup] = useState<RoundGroupTS | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadRoundGroups = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = did
      ? RoundGroupAPI.getRowsByDivision(did, p, ps)
      : RoundGroupAPI.getRowsByTournament(tid, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load roundgroups"))
      .finally(() => setLoading(false));
  }, [tid, did]);

  useEffect(() => {
    loadRoundGroups(0, pageSizeRef.current);
  }, [tid, did]);

  const handlePageChange = useCallback((newPage: number) => {
    loadRoundGroups(newPage, pageSize);
  }, [pageSize, loadRoundGroups]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadRoundGroups(0, newSize);
  }, [loadRoundGroups]);

  const handleDelete = useCallback(async (row: RoundGroupRowTS): Promise<void> => {
    await RoundGroupAPI.delete(row.roundgroup_id);
    loadRoundGroups(page, pageSize);
  }, [loadRoundGroups, page, pageSize]);

  const handleCreate = useCallback(() => {
    setEditingRoundGroup(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: RoundGroupRowTS) => {
    setEditingRoundGroup({
      roundgroup_id: row.roundgroup_id,
      did: row.did,
      name: row.name,
      created_date: row.created_date,
      creator_userid: "",
      last_modified_date: row.last_modified_date,
      last_modified_userid: row.last_modified_user_id,
    });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback((_roundgroup: RoundGroupTS): void => {
    setEditorIsOpen(false);
    setEditingRoundGroup(null);
    loadRoundGroups(page, pageSize);
  }, [loadRoundGroups, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoundGroupRowTS>
        loading={loading}
        key={did ?? tid}
        entityLabel="Session"
        title="Sessions"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={roundGroupColumns(showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(s) => s.roundgroup_id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <RoundGroupEditorDialog
        tid={tid}
        lockedDivisionId={did}
        roundgroup={editingRoundGroup}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingRoundGroup(null); }}
        onSave={handleSave}
      />
    </>
  );
}
