import { useState, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { BoolBadge, DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from "./DataTableTemplate";
import { DivisionAPI, type DivisionTS } from "../features/DivisionAPI";
import { DivisionEditorDialog } from "./DivisionEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function divisionColumns(tid: string, showSensitiveColumns: boolean): ColumnDef<DivisionTS>[] {
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
    {
      header: "Breadcrumb",
      render: (d) => d.breadcrumb,
    },
    ...(showSensitiveColumns ? [{
      header: "Is Public",
      render: (d: DivisionTS) => <BoolBadge value={d.is_public} />,
    }] : []),
    {
      header: "Short Info",
      render: (d) => d.shortinfo,
    },
    {
      header: "Created",
      render: (d) => (
        <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(d.created_at)}</span>
      ),
    },
    {
      header: "Last Modified",
      render: (d) => (
        <span style={{ whiteSpace: "nowrap", color: "#6b7280" }}>{formatDate(d.updated_at)}</span>
      ),
    },
  ];
}

export default function DivisionsTable({ tid, showCreateButton = true, showDeleteButton = true, showSensitiveColumns = false }: { tid: string; showCreateButton?: boolean; showDeleteButton?: boolean; showSensitiveColumns?: boolean }) {
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  const loadDivisions = useCallback((p: number, ps: number) => {
    DivisionAPI.getByTournament(tid, p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setDivisions(result);
      })
      .catch(() => console.error("Failed to load divisions"));
  }, [tid]);

  useEffect(() => {
    loadDivisions(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadDivisions(newPage, pageSize);
  }, [pageSize, loadDivisions]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setDivisions(prev => prev.slice(0, newSize));
    } else {
      loadDivisions(0, newSize);
    }
  }, [pageSize, page, loadDivisions]);

  const handleDelete = useCallback(async (row: DivisionTS): Promise<void> => {
    await DivisionAPI.delete(row.did);
    setDivisions((prev) => prev.filter((d) => d.did !== row.did));
  }, []);

  const handleSave = useCallback((_division: DivisionTS): void => {
    setEditorIsOpen(false);
    loadDivisions(page, pageSize);
  }, [loadDivisions, page, pageSize]);


  return (
    <>
      <DataTableTemplate<DivisionTS>
        key={tid}
        entityLabel="Division"
        showCreateButton={showCreateButton}
        showDeleteButton={showDeleteButton}
        onCreate={() => setEditorIsOpen(true)}
        columns={divisionColumns(tid, showSensitiveColumns)}
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
