import { useState, useCallback, useEffect, useRef } from "react";
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoundAPI, type RoundTS } from "../features/RoundAPI";
import { RoundEditorDialog } from "./RoundEditorDialog";
import { DivisionAPI } from "../features/DivisionAPI";

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function roundColumns(tid: string, divisionMap: Map<string, string>): ColumnDef<RoundTS>[] {
  return [
    {
      header: "Scheduled Start Time",
      render: (r) => (
        <a
          href={`/tournament/${tid}/round/${r.roundid}`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {formatDateTime(r.scheduled_start_time)}
        </a>
      ),
    },
    {
      header: "Division",
      render: (r) => divisionMap.get(r.did) ?? r.did,
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

export default function RoundsTable({ tid, showCreateButton = true, showDeleteButton = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean }) {
  const [rounds, setRounds] = useState<RoundTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadRounds = useCallback((p: number, ps: number) => {
    Promise.all([
      RoundAPI.getByTournament(tid, p, ps),
      DivisionAPI.get(0, 100),
    ])
      .then(([roundResult, divisionResult]) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(roundResult.length < ps ? p * ps + roundResult.length : (p + 2) * ps);
        setRounds(roundResult);
        setDivisionMap(new Map(divisionResult.items.map(d => [d.did, d.dname])));
      })
      .catch(() => console.error("Failed to load rounds"));
  }, [tid]);

  useEffect(() => {
    loadRounds(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadRounds(newPage, pageSize);
  }, [pageSize, loadRounds]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setRounds(prev => prev.slice(0, newSize));
    } else {
      loadRounds(0, newSize);
    }
  }, [pageSize, page, loadRounds]);

  const handleDelete = useCallback(async (row: RoundTS): Promise<void> => {
    await RoundAPI.delete(row.roundid);
    setRounds((prev) => prev.filter((r) => r.roundid !== row.roundid));
  }, []);

  const handleSave = useCallback((_round: RoundTS): void => {
    setEditorIsOpen(false);
    loadRounds(page, pageSize);
  }, [loadRounds, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoundTS>
        key={tid}
        entityLabel="Round"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={roundColumns(tid, divisionMap)}
        rows={rounds}
        totalCount={totalCount}
        getId={(r) => r.roundid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <RoundEditorDialog
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
