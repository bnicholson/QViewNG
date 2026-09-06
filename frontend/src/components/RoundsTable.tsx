import { useState, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { RoundAPI, type RoundTS, type RoundRowTS } from "../features/RoundAPI";
import { RoundEditorDialog } from "./RoundEditorDialog";

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

function roundColumns(showAuditColumns: boolean): ColumnDef<RoundRowTS>[] {
  return [
    {
      header: "Division",
      render: (r) => <EntityLink to={`/division/${r.did}/overview`}>{r.division_name || r.did}</EntityLink>,
    },
    {
      header: "Round",
      render: (r) => (
        <Link
          to={`/round/${r.roundid}/overview`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {r.name}
        </Link>
      ),
    },
    {
      header: "Scheduled Start Time",
      render: (r) => (
        <span style={{ whiteSpace: "nowrap" }}>{formatDateTime(r.scheduled_start_time)}</span>
      ),
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (r: RoundRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.created_at)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (r: RoundRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(r.updated_at)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (r: RoundRowTS) => (
          <EntityLink to={`/user/${r.last_modified_user_id}/overview`}>{r.last_modified_user_name}</EntityLink>
        ),
      },
    ] : []),
  ];
}

export default function RoundsTable({ tid, did, showCreateButton = true, showDeleteButton = true, showAuditColumns = true, hiddenColumns = [] }: { tid: string; did?: string; showCreateButton?: boolean; showDeleteButton?: boolean; showAuditColumns?: boolean;
  /** Column headers to omit — lets a consumer hide a column that's redundant in its context. */
  hiddenColumns?: string[] }) {
  // Current page of enriched rows plus the total count — paginated server-side, one call per page.
  const [rows, setRows] = useState<RoundRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadRounds = useCallback((p: number, ps: number) => {
    setLoading(true);
    const request = did
      ? RoundAPI.getRowsByDivision(did, p, ps)
      : RoundAPI.getRowsByTournament(tid, p, ps);
    request
      .then(({ count, items }) => {
        setRows(items);
        setTotalCount(count);
        setPage(p);
        setPageSize(ps);
      })
      .catch(() => console.error("Failed to load rounds"))
      .finally(() => setLoading(false));
  }, [tid, did]);

  useEffect(() => {
    loadRounds(0, pageSizeRef.current);
  }, [tid, did]);

  const handlePageChange = useCallback((newPage: number) => {
    loadRounds(newPage, pageSize);
  }, [pageSize, loadRounds]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadRounds(0, newSize);
  }, [loadRounds]);

  const handleDelete = useCallback(async (row: RoundRowTS): Promise<void> => {
    await RoundAPI.delete(row.roundid);
    // Reload the current page so the count and page contents stay correct.
    loadRounds(page, pageSize);
  }, [loadRounds, page, pageSize]);

  const handleSave = useCallback((_round: RoundTS): void => {
    setEditorIsOpen(false);
    loadRounds(page, pageSize);
  }, [loadRounds, page, pageSize]);

  return (
    <>
      <DataTableTemplate<RoundRowTS>
        loading={loading}
        key={did ?? tid}
        entityLabel="Round"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={roundColumns(showAuditColumns).filter(c => !hiddenColumns.includes(c.header))}
        rows={rows}
        totalCount={totalCount}
        getId={(r) => r.roundid}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <RoundEditorDialog
        tid={tid}
        lockedDivisionId={did}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
