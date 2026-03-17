import { useState, useCallback, useEffect } from "react";
import { DataTableTemplate, type ColumnDef } from "./DataTableTemplate";
import { RoundAPI, type RoundTS } from "../features/RoundAPI";
import { RoundEditorDialog } from "../features/RoundEditorDialog";
import { DivisionAPI, type DivisionTS } from "../features/DivisionAPI";

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

const ROUNDS_PAGE = 0;
const ROUNDS_PAGE_SIZE = 30;

export default function RoundsTable({ tid }: { tid: string }) {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [rounds, setRounds] = useState<RoundTS[]>([]);
  const [divisionMap, setDivisionMap] = useState<Map<string, string>>(new Map());
  const [editorIsOpen, setEditorIsOpen] = useState(false);

  const loadRounds = () => {
    setIsLoading(true);
    Promise.all([
      RoundAPI.get(ROUNDS_PAGE, ROUNDS_PAGE_SIZE),
      DivisionAPI.get(0, 100),
    ])
      .then(([roundResult, divisionResult]: [RoundTS[], DivisionTS[]]) => {
        setRounds(roundResult);
        setDivisionMap(new Map(divisionResult.map(d => [d.did, d.dname])));
      })
      .catch(() => console.error("Failed to load rounds"))
      .finally(() => setIsLoading(false));
  };

  useEffect(() => {
    loadRounds();
  }, [tid]);

  const handleDelete = useCallback(async (row: RoundTS): Promise<void> => {
    await RoundAPI.delete(row.roundid);
    setRounds((prev) => prev.filter((r) => r.roundid !== row.roundid));
  }, []);

  const handleSave = useCallback((_round: RoundTS): void => {
    setEditorIsOpen(false);
    loadRounds();
  }, []);

  if (isLoading) return <div>Loading rounds...</div>;

  return (
    <>
      <DataTableTemplate<RoundTS>
        entityLabel="Round"
        onCreate={() => setEditorIsOpen(true)}
        columns={roundColumns(tid, divisionMap)}
        rows={rounds}
        getId={(r) => r.roundid}
        onDelete={handleDelete}
      />
      <RoundEditorDialog
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
      />
    </>
  );
}
