import { useState, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoomAPI, type RoomTS } from "../features/RoomAPI";
import { RoomEditorDialog } from "./RoomEditorDialog";
import { UserAPI } from "../features/UserAPI";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

const userLinkStyle: React.CSSProperties = {
  color: "#2563eb",
  textDecoration: "none",
  whiteSpace: "nowrap",
};

function UserLink({ userId, userNames }: { userId: string | null; userNames: Record<string, string> }) {
  if (!userId) return <span style={{ color: "#9ca3af" }}>—</span>;
  const name = userNames[userId];
  if (!name) return <span style={{ color: "#9ca3af" }}>—</span>;
  return (
    <Link
      to={`/user/${userId}/overview`}
      style={userLinkStyle}
      onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
      onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
    >
      {name}
    </Link>
  );
}

function roomColumns(userNames: Record<string, string>): ColumnDef<RoomTS>[] {
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
    {
      header: "Building",
      render: (r) => r.building,
    },
    {
      header: "Quizmaster",
      render: (r) => <UserLink userId={r.quizmaster_id} userNames={userNames} />,
    },
    {
      header: "Content Judge",
      render: (r) => <UserLink userId={r.contentjudge_id} userNames={userNames} />,
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

export default function RoomsTable({ tid, showCreateButton = true, showDeleteButton = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean }) {
  const [rooms, setRooms] = useState<RoomTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [userNames, setUserNames] = useState<Record<string, string>>({});
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadUserNames = useCallback((roomList: RoomTS[]) => {
    const ids = [...new Set(
      roomList.flatMap(r => [r.quizmaster_id, r.contentjudge_id]).filter((id): id is string => !!id)
    )];
    if (ids.length === 0) return;
    Promise.all(ids.map(id => UserAPI.getById(id).catch(() => null))).then(users => {
      const names: Record<string, string> = {};
      users.forEach((u, i) => { if (u) names[ids[i]] = `${u.fname} ${u.lname}`; });
      setUserNames(prev => ({ ...prev, ...names }));
    });
  }, []);

  const loadRooms = useCallback((p: number, ps: number) => {
    RoomAPI.getByTournament(tid, p, ps)
      .then((result) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setRooms(result);
        loadUserNames(result);
      })
      .catch(() => console.error("Failed to load rooms"));
  }, [tid, loadUserNames]);

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
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={roomColumns(userNames)}
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
