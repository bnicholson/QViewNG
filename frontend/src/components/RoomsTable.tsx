import { useState, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoomAPI, type RoomTS, type RoomRowTS } from "../features/RoomAPI";
import { RoomEditorDialog } from "./RoomEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function roomColumns(showAuditColumns: boolean): ColumnDef<RoomRowTS>[] {
  return [
    {
      header: "Name",
      render: (r) => (
        <Link
          to={`/room/${r.roomid}/overview`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {r.name}
        </Link>
      ),
    },
    ...(showAuditColumns ? [
      {
        header: "Comments",
        render: (r: RoomRowTS) => r.comments,
      },
      {
        header: "Created",
        render: (r: RoomRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.created_at)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (r: RoomRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.updated_at)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (r: RoomRowTS) => (
          <EntityLink to={`/user/${r.last_modified_user_id}/overview`}>{r.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
  ];
}

export default function RoomsTable({ tid, showCreateButton = true, showDeleteButton = true, showAuditColumns = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean; showAuditColumns?: boolean }) {
  const [rooms, setRooms] = useState<RoomRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadRooms = useCallback((p: number, ps: number) => {
    setLoading(true);
    RoomAPI.getRowsByTournament(tid, p, ps)
      .then(({ count, items }) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(count);
        setRooms(items);
      })
      .catch(() => console.error("Failed to load rooms"))
      .finally(() => setLoading(false));
  }, [tid]);

  useEffect(() => {
    loadRooms(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadRooms(newPage, pageSize);
  }, [pageSize, loadRooms]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadRooms(0, newSize);
  }, [loadRooms]);

  const handleDelete = useCallback(async (row: RoomRowTS): Promise<void> => {
    await RoomAPI.delete(row.roomid);
    loadRooms(page, pageSize);
  }, [loadRooms, page, pageSize]);

  const handleSave = useCallback((_room: RoomTS): void => {
    setEditorIsOpen(false);
    loadRooms(page, pageSize);
  }, [loadRooms, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoomRowTS>
        key={tid}
        entityLabel="Room"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        loading={loading}
        columns={roomColumns(showAuditColumns)}
        rows={rooms}
        totalCount={totalCount}
        getId={(r) => r.roomid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <RoomEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
