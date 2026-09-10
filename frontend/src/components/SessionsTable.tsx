import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { DivisionSessionAPI, type DivisionSessionTS, type DivisionSessionRowTS } from "../features/DivisionSessionAPI";
import { DivisionSessionEditorDialog } from "./DivisionSessionEditorDialog";

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

function sessionColumns(
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: DivisionSessionRowTS) => void,
): ColumnDef<DivisionSessionRowTS>[] {
  return [
    {
      header: "Division",
      render: (s) => <EntityLink to={`/division/${s.did}/overview`}>{s.division_name || s.did}</EntityLink>,
    },
    {
      header: "Session",
      render: (s) => <EntityLink to={`/division-session/${s.division_session_id}/overview`}>{s.name}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (s: DivisionSessionRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(s.created_date)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (s: DivisionSessionRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(s.last_modified_date)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (s: DivisionSessionRowTS) => (
          <EntityLink to={`/user/${s.last_modified_user_id}/overview`}>{s.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: "Edit",
      render: (s: DivisionSessionRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(s)}>Edit</button>
      ),
    }] : []),
  ];
}

export default function SessionsTable({ tid, did, showCreateButton = true, showEditButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: {
  tid: string;
  did: string;
  showCreateButton?: boolean;
  showEditButton?: boolean;
  showDeleteButton?: boolean;
  showAuditColumns?: boolean;
  /** Column headers to omit — lets a consumer hide a column that's redundant in its context. */
  hiddenColumns?: string[];
}) {
  const [rows, setRows] = useState<DivisionSessionRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingSession, setEditingSession] = useState<DivisionSessionTS | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadSessions = useCallback((p: number, ps: number) => {
    setLoading(true);
    DivisionSessionAPI.getRowsByDivision(did, p, ps)
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load sessions"))
      .finally(() => setLoading(false));
  }, [did]);

  useEffect(() => {
    loadSessions(0, pageSizeRef.current);
  }, [did]);

  const handlePageChange = useCallback((newPage: number) => {
    loadSessions(newPage, pageSize);
  }, [pageSize, loadSessions]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadSessions(0, newSize);
  }, [loadSessions]);

  const handleDelete = useCallback(async (row: DivisionSessionRowTS): Promise<void> => {
    await DivisionSessionAPI.delete(row.division_session_id);
    loadSessions(page, pageSize);
  }, [loadSessions, page, pageSize]);

  const handleCreate = useCallback(() => {
    setEditingSession(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: DivisionSessionRowTS) => {
    setEditingSession({
      division_session_id: row.division_session_id,
      did: row.did,
      name: row.name,
      created_date: row.created_date,
      creator_userid: "",
      last_modified_date: row.last_modified_date,
      last_modified_userid: row.last_modified_user_id,
    });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback((_session: DivisionSessionTS): void => {
    setEditorIsOpen(false);
    setEditingSession(null);
    loadSessions(page, pageSize);
  }, [loadSessions, page, pageSize]);

  return (
    <>
      <DataTableTemplate<DivisionSessionRowTS>
        loading={loading}
        key={did}
        entityLabel="Session"
        title="Sessions"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={sessionColumns(showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(s) => s.division_session_id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <DivisionSessionEditorDialog
        tid={tid}
        lockedDivisionId={did}
        session={editingSession}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingSession(null); }}
        onSave={handleSave}
      />
    </>
  );
}
