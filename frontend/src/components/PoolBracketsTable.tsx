import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { PoolBracketAPI, type PoolBracketTS, type PoolBracketRowTS } from "../features/PoolBracketAPI";
import { PoolBracketEditorDialog } from "./PoolBracketEditorDialog";

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

function bracketColumns(
  entityLabel: string,
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: PoolBracketRowTS) => void,
): ColumnDef<PoolBracketRowTS>[] {
  return [
    {
      header: "Division",
      render: (b) => <EntityLink to={`/division/${b.did}/overview`}>{b.division_name || b.did}</EntityLink>,
    },
    {
      header: entityLabel,
      render: (b) => <EntityLink to={`/pool-bracket/${b.pool_bracket_id}/overview`}>{b.name}</EntityLink>,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (b: PoolBracketRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(b.created_date)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (b: PoolBracketRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(b.last_modified_date)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (b: PoolBracketRowTS) => (
          <EntityLink to={`/user/${b.last_modified_user_id}/overview`}>{b.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: "Edit",
      render: (b: PoolBracketRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(b)}>Edit</button>
      ),
    }] : []),
  ];
}

/**
 * Data table of pool_brackets of one `type` (e.g. "pool" or "bracket") within a division, with
 * Add / Edit / Delete. `entityLabel`/`title` let the same component render the Pools and Brackets
 * sections.
 */
export default function PoolBracketsTable({
  tid,
  did,
  type,
  entityLabel,
  title,
  showCreateButton = true,
  showEditButton = true,
  showDeleteButton = true,
  showAuditColumns = true,
  hiddenColumns = [],
}: {
  tid: string;
  /** When set, rows are scoped to this division; when omitted, to the whole tournament (`tid`). */
  did?: string;
  type: string;
  entityLabel: string;
  title: string;
  showCreateButton?: boolean;
  showEditButton?: boolean;
  showDeleteButton?: boolean;
  showAuditColumns?: boolean;
  hiddenColumns?: string[];
}) {
  const [rows, setRows] = useState<PoolBracketRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingBracket, setEditingBracket] = useState<PoolBracketTS | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadBrackets = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = did
      ? PoolBracketAPI.getRowsByDivision(did, type, p, ps)
      : PoolBracketAPI.getRowsByTournament(tid, type, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load pool brackets"))
      .finally(() => setLoading(false));
  }, [tid, did, type]);

  useEffect(() => {
    loadBrackets(0, pageSizeRef.current);
  }, [tid, did, type]);

  const handlePageChange = useCallback((newPage: number) => {
    loadBrackets(newPage, pageSize);
  }, [pageSize, loadBrackets]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadBrackets(0, newSize);
  }, [loadBrackets]);

  const handleDelete = useCallback(async (row: PoolBracketRowTS): Promise<void> => {
    await PoolBracketAPI.delete(row.pool_bracket_id);
    loadBrackets(page, pageSize);
  }, [loadBrackets, page, pageSize]);

  const handleCreate = useCallback(() => {
    setEditingBracket(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: PoolBracketRowTS) => {
    setEditingBracket({
      pool_bracket_id: row.pool_bracket_id,
      divisionid: row.did,
      type: row.type,
      created_date: row.created_date,
      creator_userid: "",
      last_modified_date: row.last_modified_date,
      last_modified_userid: row.last_modified_user_id,
      name: row.name,
    });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback((_bracket: PoolBracketTS): void => {
    setEditorIsOpen(false);
    setEditingBracket(null);
    loadBrackets(page, pageSize);
  }, [loadBrackets, page, pageSize]);

  return (
    <>
      <DataTableTemplate<PoolBracketRowTS>
        loading={loading}
        key={`${did ?? tid}-${type}`}
        entityLabel={entityLabel}
        title={title}
        createLabel={`Create ${entityLabel}`}
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={bracketColumns(entityLabel, showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(b) => b.pool_bracket_id}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <PoolBracketEditorDialog
        tid={tid}
        did={did}
        type={type}
        entityLabel={entityLabel}
        bracket={editingBracket}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingBracket(null); }}
        onSave={handleSave}
      />
    </>
  );
}
