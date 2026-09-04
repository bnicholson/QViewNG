import { useState, useCallback, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { BoolBadge, DataTableTemplate, EntityLink, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type UserTS } from '../features/UserAPI';
import { TeamAPI } from '../features/TeamAPI';
import { DivisionAPI } from '../features/DivisionAPI';

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
  });
}

/** A referenced entity (division or team) the quizzer belongs to. */
interface EntityRef { id: string; name: string; }

/** Render a list of entity references as comma-separated profile links (or "—" when empty). */
function renderRefs(refs: EntityRef[] | undefined, base: 'division' | 'team') {
  if (!refs || refs.length === 0) return '—';
  return refs.map((r, i) => (
    <span key={r.id}>{i > 0 ? ', ' : ''}<EntityLink to={`/${base}/${r.id}/overview`}>{r.name}</EntityLink></span>
  ));
}

function quizzerColumns(
  showSensitiveColumns: boolean,
  showAuditColumns: boolean,
  showTeamAndDivision: boolean,
  divisionMap: Map<string, EntityRef[]>,
  teamMap: Map<string, EntityRef[]>,
): ColumnDef<UserTS>[] {
  return [
    ...(showTeamAndDivision ? [
      {
        header: 'Division',
        render: (u: UserTS) => renderRefs(divisionMap.get(u.id), 'division'),
      },
      {
        header: 'Team',
        render: (u: UserTS) => renderRefs(teamMap.get(u.id), 'team'),
      },
    ] : []),
    {
      header: 'Full Name',
      render: (u) => (
        <Link
          to={`/user/${u.id}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={(e) => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={(e) => (e.currentTarget.style.textDecoration = 'none')}
        >
          {`${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`}
        </Link>
      ),
    },
    ...(showSensitiveColumns ? [{
      header: 'Email',
      render: (u: UserTS) => u.email,
    },
    {
      header: 'Activated',
      render: (u: UserTS) => <BoolBadge value={u.activated} />,
    }] : []),
    ...(showAuditColumns ? [
      {
        header: 'Created',
        render: (u: UserTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.created_at)}</span>
        ),
      },
      {
        header: 'Last Modified',
        render: (u: UserTS) => (
          <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(u.updated_at)}</span>
        ),
      },
    ] : []),
  ];
}

interface Props {
  tid?: string;
  /** If provided, skips internal fetch and displays these rows directly. */
  externalRows?: UserTS[];
  /** When provided, shows a create button that calls this. Omit to hide the button entirely. */
  onAdd?: () => void;
  /** Overrides the delete handler. When omitted, throws (quizzers managed via rosters). */
  onDelete?: (user: UserTS) => Promise<void>;
  /** Overrides the create button label. Only meaningful when onAdd is provided. */
  createLabel?: string;
  showSensitiveColumns?: boolean;
  showAuditColumns?: boolean;
}

export default function QuizzersTable({ tid, externalRows, onAdd, onDelete, createLabel, showSensitiveColumns = false, showAuditColumns = true }: Props) {
  // All items for client-side pagination (tid or externalRows mode)
  const [allTournamentQuizzers, setAllTournamentQuizzers] = useState<UserTS[] | undefined>(undefined);
  // Current-page items for server-side pagination (no tid, no externalRows)
  const [quizzers, setQuizzers] = useState<UserTS[]>([]);
  const [loading, setLoading] = useState(true);
  const [apiTotalCount, setApiTotalCount] = useState(0);
  // quizzerId -> the divisions / teams they belong to, built from this tournament's teams (tid mode only)
  const [divisionMap, setDivisionMap] = useState<Map<string, EntityRef[]>>(new Map());
  const [teamMap, setTeamMap] = useState<Map<string, EntityRef[]>>(new Map());

  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Fetch all quizzers for the tournament once; re-fetch when tid changes
  useEffect(() => {
    if (tid === undefined) return;
    setPage(0);
    setAllTournamentQuizzers(undefined);
    Promise.all([
      UserAPI.getByTournament(tid),
      TeamAPI.getByTournament(tid, 0, 500),
      DivisionAPI.getByTournament(tid, 0, 100),
    ])
      .then(([userResult, teamResult, divisionResult]) => {
        setAllTournamentQuizzers(userResult.items);
        // Map each quizzer to the divisions/teams they belong to via the teams' quizzer slots.
        const divNameById = new Map(divisionResult.map(d => [d.did, d.dname]));
        const dMap = new Map<string, EntityRef[]>();
        const tMap = new Map<string, EntityRef[]>();
        for (const team of teamResult.items) {
          const divName = divNameById.get(team.did) ?? team.did;
          const slots = [
            team.quizzer_one_id, team.quizzer_two_id, team.quizzer_three_id,
            team.quizzer_four_id, team.quizzer_five_id, team.quizzer_six_id,
          ];
          for (const qid of slots) {
            if (!qid) continue;
            const tList = tMap.get(qid) ?? [];
            if (!tList.some(e => e.id === team.teamid)) tList.push({ id: team.teamid, name: team.name });
            tMap.set(qid, tList);
            const dList = dMap.get(qid) ?? [];
            if (!dList.some(e => e.id === team.did)) dList.push({ id: team.did, name: divName });
            dMap.set(qid, dList);
          }
        }
        setDivisionMap(dMap);
        setTeamMap(tMap);
      })
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setLoading(false));
  }, [tid]);

  // Server-side pagination fetch (only used when no tid and no externalRows)
  const loadQuizzers = useCallback((p: number, ps: number) => {
    if (externalRows !== undefined || tid !== undefined) return;
    UserAPI.get(p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setApiTotalCount(result.count);
        setQuizzers(result.items);
      })
      .catch(() => console.error('Failed to load quizzers'))
      .finally(() => setLoading(false));
  }, [externalRows, tid]);

  useEffect(() => {
    loadQuizzers(0, pageSizeRef.current);
  }, []);

  // Client-side data: externalRows takes priority, then tournament quizzers.
  // Tournament quizzers arrive already sorted by name from the backend query.
  const clientItems = externalRows ?? allTournamentQuizzers;

  // Slice for the current page when all data is loaded; otherwise use the server-fetched page
  const rows = clientItems !== undefined
    ? clientItems.slice(page * pageSize, (page + 1) * pageSize)
    : quizzers;

  const totalCount = clientItems !== undefined ? clientItems.length : apiTotalCount;

  const handlePageChange = useCallback((newPage: number) => {
    if (externalRows !== undefined || allTournamentQuizzers !== undefined) {
      setPage(newPage);
    } else {
      loadQuizzers(newPage, pageSize);
    }
  }, [externalRows, allTournamentQuizzers, pageSize, loadQuizzers]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPage(0);
    setPageSize(newSize);
    if (externalRows === undefined && allTournamentQuizzers === undefined) {
      loadQuizzers(0, newSize);
    }
  }, [externalRows, allTournamentQuizzers, loadQuizzers]);

  const handleDelete = useCallback(async (row: UserTS): Promise<void> => {
    if (onDelete) return onDelete(row);
    throw new Error('Quizzer-tournament associations are managed through rosters.');
  }, [onDelete]);

  return (
    <DataTableTemplate<UserTS>
      loading={loading}
      entityLabel="Quizzer"
      createLabel={createLabel}
      onCreate={onAdd}
      showCreateButton={!!onAdd}
      showDeleteButton={!!onDelete}
      columns={quizzerColumns(showSensitiveColumns, showAuditColumns, tid !== undefined, divisionMap, teamMap)}
      rows={rows}
      totalCount={totalCount}
      getId={(u) => u.id}
      onDelete={handleDelete}
      page={page}
      pageSize={pageSize}
      onPageChange={handlePageChange}
      onPageSizeChange={handlePageSizeChange}
    />
  );
}
