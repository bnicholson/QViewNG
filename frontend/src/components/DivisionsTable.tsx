import { useState, useCallback, useEffect } from "react";
import { BoolBadge, DataTableTemplate, type ColumnDef } from "./DataTableTemplate";
import { DivisionAPI, type DivisionTS } from "../features/DivisionAPI";
import { DivisionEditorDialog } from "../features/DivisionEditorDialog";

function formatDate(iso: string | null | undefined): string {
  if (!iso) return "—";
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function divisionColumns(tid: string): ColumnDef<DivisionTS>[] {
  return [
    {
      header: "Name",
      render: (d) => (
        <a
          href={`/tournament/${tid}/division/${d.did}`}
          style={{ color: "#2563eb", textDecoration: "none", fontWeight: 500, whiteSpace: "nowrap" }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = "underline")}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = "none")}
        >
          {d.dname}
        </a>
      ),
    },
    {
      header: "Breadcrumb",
      render: (d) => d.breadcrumb,
    },
    {
      header: "Is Public",
      render: (d) => <BoolBadge value={d.is_public} />,
    },
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

const DIVISIONS_PAGE = 0
const DIVISIONS_PAGE_SIZE = 30

export default function DivisionsTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  useEffect(() => {
    setIsLoading(true)
    DivisionAPI.get(DIVISIONS_PAGE, DIVISIONS_PAGE_SIZE)
      .then((result: DivisionTS[]) => {
        setDivisions(result)
      })
      .catch(() => {
        console.error("Failed to load divisions")
      })
      .finally(() => setIsLoading(false))
  }, [tid])

  const handleDelete = useCallback(async (row: DivisionTS): Promise<void> => {
    await DivisionAPI.delete(row.did);
    setDivisions((prev) => prev.filter((d) => d.did !== row.did));
  }, []);

  const handleSave = useCallback((_division: DivisionTS): void => {
    setEditorIsOpen(false);
    setIsLoading(true);
    DivisionAPI.get(DIVISIONS_PAGE, DIVISIONS_PAGE_SIZE)
      .then((result: DivisionTS[]) => setDivisions(result))
      .catch(() => console.error("Failed to reload divisions"))
      .finally(() => setIsLoading(false));
  }, []);

  if (isLoading) return <div>Loading divisions...</div>;

  return (
    <>
      <DataTableTemplate<DivisionTS>
        entityLabel="Division"
        onCreate={() => setEditorIsOpen(true)}
        columns={divisionColumns(tid)}
        rows={divisions}
        getId={(d) => d.did}
        onDelete={handleDelete}
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
