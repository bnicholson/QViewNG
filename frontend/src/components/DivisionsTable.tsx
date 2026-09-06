import { useState, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { BoolBadge, DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { DivisionAPI, type DivisionTS, type DivisionRowTS } from "../features/DivisionAPI";
import { DivisionEditorDialog } from "./DivisionEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function divisionColumns(showSensitiveColumns: boolean, showAuditColumns: boolean): ColumnDef<DivisionRowTS>[] {
  return [
    {
      header: "Name",
      render: (d) => (
        <Link
          to={`/division/${d.did}/overview`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {d.dname}
        </Link>
      ),
    },
    ...(showSensitiveColumns ? [{
      header: "Is Public",
      render: (d: DivisionRowTS) => <BoolBadge value={d.is_public} />,
    }] : []),
    {
      header: "Short Info",
      render: (d) => d.shortinfo,
    },
    ...(showAuditColumns ? [
      {
        header: "Created",
        render: (d: DivisionRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(d.created_at)}</span>
        ),
      },
      {
        header: "Last Modified",
        render: (d: DivisionRowTS) => (
          <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(d.updated_at)}</span>
        ),
      },
      {
        header: "Last Modified By",
        render: (d: DivisionRowTS) => (
          <EntityLink to={`/user/${d.last_modified_user_id}/overview`}>{d.last_modified_user_name}</EntityLink>
        ),
      }
    ] : [])
  ];
}

export default function DivisionsTable({ tid, showCreateButton = true, showDeleteButton = true, showSensitiveColumns = false, showAuditColumns = true }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean; showSensitiveColumns?: boolean; showAuditColumns?: boolean }) {
  const [divisions, setDivisions] = useState<DivisionRowTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadDivisions = useCallback((p: number, ps: number) => {
    setLoading(true);
    DivisionAPI.getRowsByTournament(tid, p, ps)
      .then(({ count, items }) => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(count);
        setDivisions(items);
      })
      .catch(() => console.error("Failed to load divisions"))
      .finally(() => setLoading(false));
  }, [tid]);

  useEffect(() => {
    loadDivisions(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadDivisions(newPage, pageSize);
  }, [pageSize, loadDivisions]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    loadDivisions(0, newSize);
  }, [loadDivisions]);

  const handleDelete = useCallback(async (row: DivisionRowTS): Promise<void> => {
    await DivisionAPI.delete(row.did);
    loadDivisions(page, pageSize);
  }, [loadDivisions, page, pageSize]);

  const handleSave = useCallback((_division: DivisionTS): void => {
    setEditorIsOpen(false);
    loadDivisions(page, pageSize);
  }, [loadDivisions, page, pageSize]);


  return (
    <>
      <DataTableTemplate<DivisionRowTS>
        loading={loading}
        key={tid}
        entityLabel="Division"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={divisionColumns(showSensitiveColumns, showAuditColumns)}
        rows={divisions}
        totalCount={totalCount}
        getId={(d) => d.did}
        onDelete={handleDelete}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />
      <DivisionEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
