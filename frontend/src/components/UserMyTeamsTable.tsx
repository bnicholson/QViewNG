import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate';
import { UserAPI, type TeamWithTournamentInfoTS } from '../features/UserAPI';
import { TeamAPI } from '../features/TeamAPI';

function formatDateRange(from: string, to: string): string {
  const fmt = (s: string) =>
    new Date(s).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
  const f = fmt(from);
  const t = fmt(to);
  return f === t ? f : `${f} – ${t}`;
}

function linkStyle(): React.CSSProperties {
  return { color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' };
}

function onHover(e: React.MouseEvent<HTMLElement>, enter: boolean) {
  (e.currentTarget as HTMLElement).style.textDecoration = enter ? 'underline' : 'none';
}

interface MyTeamRow {
  teamid: string;
  name: string;
  coachid: string;
  coach_name: string;
  tournament_id: string;
  tournament_name: string;
  tournament_fromdate: string;
  tournament_todate: string;
  role: 'Quizzer' | 'Coach' | 'Coach & Quizzer';
  quizzers: { id: string; name: string }[];
}

const columns: ColumnDef<MyTeamRow>[] = [
  {
    header: 'Tournament',
    render: (r) => (
      <Link
        to={`/tournament/${r.tournament_id}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {r.tournament_name}
      </Link>
    ),
  },
  {
    header: 'Date(s)',
    render: (r) => (
      <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>
        {formatDateRange(r.tournament_fromdate, r.tournament_todate)}
      </span>
    ),
  },
  {
    header: 'Team',
    render: (r) => (
      <Link
        to={`/team/${r.teamid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {r.name}
      </Link>
    ),
  },
  {
    header: 'Role',
    render: (r) => <span style={{ whiteSpace: 'nowrap' }}>{r.role}</span>,
  },
  {
    header: 'Coach',
    render: (r) => (
      <Link
        to={`/user/${r.coachid}/overview`}
        style={linkStyle()}
        onMouseEnter={(e) => onHover(e, true)}
        onMouseLeave={(e) => onHover(e, false)}
      >
        {r.coach_name}
      </Link>
    ),
  },
  {
    header: 'Quizzers',
    render: (r) => r.quizzers.length === 0 ? (
      <span style={{ color: '#6b7280' }}>—</span>
    ) : (
      <span>
        {r.quizzers.map((q, i) => (
          <span key={q.id}>
            {i > 0 && ', '}
            <Link
              to={`/user/${q.id}/overview`}
              style={linkStyle()}
              onMouseEnter={(e) => onHover(e, true)}
              onMouseLeave={(e) => onHover(e, false)}
            >
              {q.name}
            </Link>
          </span>
        ))}
      </span>
    ),
  },
];

export default function UserMyTeamsTable({
  userId,
  showCreateButton = false,
  showDeleteButton = false,
}: {
  userId: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}) {
  const [allRows, setAllRows] = useState<MyTeamRow[]>([]);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    Promise.all([
      UserAPI.getTeamsAsQuizzer(userId, 0, 500),
      UserAPI.getTeamsAsCoach(userId, 0, 500),
      UserAPI.get(0, 500),
    ]).then(async ([quizzerTeams, coachTeams, userResult]) => {
      const userMap = new Map(userResult.items.map(u => [
        u.id,
        [u.fname, u.mname, u.lname].filter(Boolean).join(' '),
      ]));

      // Merge teams, tracking role per teamid
      const roleMap = new Map<string, 'Quizzer' | 'Coach' | 'Coach & Quizzer'>();
      const infoMap = new Map<string, TeamWithTournamentInfoTS>();

      for (const t of quizzerTeams) {
        roleMap.set(t.teamid, 'Quizzer');
        infoMap.set(t.teamid, t);
      }
      for (const t of coachTeams) {
        const existing = roleMap.get(t.teamid);
        roleMap.set(t.teamid, existing === 'Quizzer' ? 'Coach & Quizzer' : 'Coach');
        infoMap.set(t.teamid, t);
      }

      const teamids = Array.from(infoMap.keys());
      const fullTeams = await Promise.all(teamids.map(id => TeamAPI.getById(id)));
      const quizzerSlotsMap = new Map(fullTeams.map(ft => {
        const quizzers = [
          ft.quizzer_one_id,
          ft.quizzer_two_id,
          ft.quizzer_three_id,
          ft.quizzer_four_id,
          ft.quizzer_five_id,
          ft.quizzer_six_id,
        ]
          .filter((id): id is string => !!id)
          .map(id => ({ id, name: userMap.get(id) ?? id }));
        return [ft.teamid, quizzers];
      }));

      const rows: MyTeamRow[] = teamids.map(tid => {
        const info = infoMap.get(tid)!;
        return {
          teamid: info.teamid,
          name: info.name,
          coachid: info.coachid,
          coach_name: info.coach_name,
          tournament_id: info.tournament_id,
          tournament_name: info.tournament_name,
          tournament_fromdate: info.tournament_fromdate,
          tournament_todate: info.tournament_todate,
          role: roleMap.get(tid)!,
          quizzers: quizzerSlotsMap.get(tid) ?? [],
        };
      });

      setAllRows(rows);
      setPage(0);
    }).catch(() => console.error('Failed to load my teams'))
      .finally(() => setLoading(false));
  }, [userId]);

  const visibleRows = allRows.slice(page * pageSize, page * pageSize + pageSize);

  return (
    <DataTableTemplate<MyTeamRow>
      key={userId}
      entityLabel="Team"
      showCreateButton={showCreateButton}
      showDeleteButton={showDeleteButton}
      columns={columns}
      rows={loading ? [] : visibleRows}
      totalCount={allRows.length}
      getId={(r) => r.teamid}
      onDelete={async () => {}}
      page={page}
      pageSize={pageSize}
      onPageChange={(p) => setPage(p)}
      onPageSizeChange={(ps) => { setPageSize(ps); setPage(0); }}
    />
  );
}
