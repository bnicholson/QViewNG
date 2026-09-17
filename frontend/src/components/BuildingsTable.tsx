import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoomGroupAPI, type RoomGroupTS, type RoomGroupRowTS } from "../features/RoomGroupAPI";
import { BuildingEditorDialog } from "./BuildingEditorDialog";
import { useAuth } from "../hooks/useAuth";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", { year: "numeric", month: "short", day: "numeric" });
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

function buildingColumns(
  showAuditColumns: boolean,
  showEditButton: boolean,
  onEdit: (row: RoomGroupRowTS) => void,
): ColumnDef<RoomGroupRowTS>[] {
  return [
    {
      header: "Name",
      render: (b) => <EntityLink to={`/building/${b.roomgroupid}/overview`}>{b.name}</EntityLink>,
    },
    {
      header: "Notes",
      render: (b) => <span style={{ color: "#374151" }}>{b.notes || "—"}</span>,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (b: RoomGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(b.created_date)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (b: RoomGroupRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(b.last_modified_date)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (b: RoomGroupRowTS) => (
          <EntityLink to={`/user/${b.last_modified_user_id}/overview`}>{b.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
    ...(showEditButton ? [{
      header: "Edit",
      render: (b: RoomGroupRowTS) => (
        <button style={editButtonStyle()} onClick={() => onEdit(b)}>Edit</button>
      ),
    }] : []),
  ];
}

export default function BuildingsTable({ tid, showCreateButton = true, showEditButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: {
  tid: string;
  showCreateButton?: boolean;
  showEditButton?: boolean;
  showDeleteButton?: boolean;
  showAuditColumns?: boolean;
  /** Column headers to omit — lets a consumer hide a column that's redundant in its context. */
  hiddenColumns?: string[];
}) {
  const { accessToken } = useAuth();
  const [rows, setRows] = useState<RoomGroupRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingBuilding, setEditingBuilding] = useState<RoomGroupTS | null>(null);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadBuildings = useCallback((p: number, ps: number) => {
    setLoading(true);
    RoomGroupAPI.getRowsByTournament(tid, p, ps)
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load buildings"))
      .finally(() => setLoading(false));
  }, [tid]);

  useEffect(() => {
    loadBuildings(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadBuildings(newPage, pageSize);
  }, [pageSize, loadBuildings]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadBuildings(0, newSize);
  }, [loadBuildings]);

  const handleDelete = useCallback(async (row: RoomGroupRowTS): Promise<void> => {
    await RoomGroupAPI.delete(row.roomgroupid, accessToken);
    loadBuildings(page, pageSize);
  }, [loadBuildings, page, pageSize, accessToken]);

  const handleCreate = useCallback(() => {
    setEditingBuilding(null);
    setEditorIsOpen(true);
  }, []);

  const handleEdit = useCallback((row: RoomGroupRowTS) => {
    setEditingBuilding({
      roomgroupid: row.roomgroupid,
      tournamentid: row.tournamentid,
      type: row.type,
      name: row.name,
      notes: row.notes,
      created_date: row.created_date,
      creator_userid: "",
      last_modified_date: row.last_modified_date,
      last_modified_userid: row.last_modified_user_id,
    });
    setEditorIsOpen(true);
  }, []);

  const handleSave = useCallback((_building: RoomGroupTS): void => {
    setEditorIsOpen(false);
    setEditingBuilding(null);
    loadBuildings(page, pageSize);
  }, [loadBuildings, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoomGroupRowTS>
        loading={loading}
        key={tid}
        entityLabel="Building"
        title="Buildings"
        createLabel="Create Building"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={handleCreate}
        columns={buildingColumns(showAuditColumns, showEditButton, handleEdit).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(b) => b.roomgroupid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <BuildingEditorDialog
        tid={tid}
        building={editingBuilding}
        isOpen={editorIsOpen}
        onCancel={() => { setEditorIsOpen(false); setEditingBuilding(null); }}
        onSave={handleSave}
      />
    </>
  );
}
