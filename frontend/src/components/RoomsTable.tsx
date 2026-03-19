import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoomAPI, type RoomTS } from "../features/RoomAPI";
import { RoomEditorDialog } from "./RoomEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function roomColumns(tid: string): ColumnDef<RoomTS>[] {
  return [
    {
      header: "Name",
      render: (r) => (
        <a
          href={`/tournament/${tid}/room/${r.roomid}`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {r.name}
        </a>
      ),
    },
    {
      header: "Building",
      render: (r) => r.building,
    },
    {
      header: "Client Key",
      render: (r) => r.clientkey,
    },
    {
      header: "Comments",
      render: (r) => r.comments,
    },
    {
      header: "Created",
      render: (r) => (
        <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.created_at)}</span>
      ),
    },
    {
      header: "Last Modified",
      render: (r) => (
        <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.updated_at)}</span>
      ),
    },
  ];
}

export default function RoomsTable({ tid }: { tid: string }) {
  const [rooms, setRooms] = useState<RoomTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadRooms = useCallback((p: number, ps: number) => {
    RoomAPI.getByTournament(tid, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setRooms(result);
      })
      .catch(() => console.error("Failed to load rooms"));
  }, [tid]);

  useEffect(() => {
    loadRooms(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadRooms(newPage, pageSize);
  }, [pageSize, loadRooms]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setRooms(prev => prev.slice(0, newSize));
    } else {
      loadRooms(0, newSize);
    }
  }, [pageSize, page, loadRooms]);

  const handleDelete = useCallback(async (row: RoomTS): Promise<void> => {
    await RoomAPI.delete(row.roomid);
    setRooms((prev) => prev.filter((r) => r.roomid !== row.roomid));
  }, []);

  const handleSave = useCallback((_room: RoomTS): void => {
    setEditorIsOpen(false);
    loadRooms(page, pageSize);
  }, [loadRooms, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoomTS>
        key={tid}
        entityLabel="Room"
        onCreate={() => setEditorIsOpen(true)}
        columns={roomColumns(tid)}
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
