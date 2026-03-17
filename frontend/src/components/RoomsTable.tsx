import { useState, useCallback, useEffect } from "react";
import { DataTableTemplate, type ColumnDef } from "./DataTableTemplate";
import { RoomAPI, type RoomTS } from "../features/RoomAPI";
import { RoomEditorDialog } from "../features/RoomEditorDialog";

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

const ROOMS_PAGE = 0;
const ROOMS_PAGE_SIZE = 30;

export default function RoomsTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [rooms, setRooms] = useState<RoomTS[]>([]);
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  useEffect(() => {
    setIsLoading(true);
    RoomAPI.get(ROOMS_PAGE, ROOMS_PAGE_SIZE)
      .then((result: RoomTS[]) => setRooms(result))
      .catch(() => console.error("Failed to load rooms"))
      .finally(() => setIsLoading(false));
  }, [tid]);

  const handleDelete = useCallback(async (row: RoomTS): Promise<void> => {
    await RoomAPI.delete(row.roomid);
    setRooms((prev) => prev.filter((r) => r.roomid !== row.roomid));
  }, []);

  const handleSave = useCallback((_room: RoomTS): void => {
    setEditorIsOpen(false);
    setIsLoading(true);
    RoomAPI.get(ROOMS_PAGE, ROOMS_PAGE_SIZE)
      .then((result: RoomTS[]) => setRooms(result))
      .catch(() => console.error("Failed to reload rooms"))
      .finally(() => setIsLoading(false));
  }, []);

  if (isLoading) return <div>Loading rooms...</div>;

  return (
    <>
      <DataTableTemplate<RoomTS>
        entityLabel="Room"
        onCreate={() => setEditorIsOpen(true)}
        columns={roomColumns(tid)}
        rows={rooms}
        getId={(r) => r.roomid}
        onDelete={handleDelete}
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
