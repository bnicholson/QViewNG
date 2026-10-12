import { useState, useEffect } from 'react'
import { useParams } from 'react-router'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Paper from '@mui/material/Paper'
import { useTheme } from '@mui/material/styles'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from '../components/DataTableTemplate'
import { LiveIndicator } from '../components/LiveIndicator'
import { useLiveFeed } from '../hooks/useLiveFeed'

// The tickertape is pushed live by the server (see useLiveFeed). This is how often it re-reads
// the backend instead while the live connection is unavailable. The backend serves this from the
// Valkey cache, so a short interval is cheap: a warm read does no gameevents queries at all.
const POLL_MS = 15_000

// How often the "Check-in" column re-renders so its ages stay current between updates.
const TICK_MS = 5_000

/** One team's live standing within a round in progress. Mirrors the backend's GamesInProgressTeam. */
export interface GamesInProgressTeam {
  name: string
  score: number
}

/** One actively-reporting room. Mirrors the backend's GamesInProgressRow. */
export interface GamesInProgressRow {
  roomid: string
  tournament: string
  division: string | null
  room: string | null
  round: string | null
  question: number | null
  check_in: string | null
  teams: GamesInProgressTeam[]
}

export const GamesInProgressAPI = {
  get: async (tid: string): Promise<GamesInProgressRow[]> => {
    const res = await fetch(`/api/tournaments/${tid}/gamesinprogress`)
    if (!res.ok) throw new Error(`Failed to load games in progress (${res.status})`)
    return res.json()
  },
}

// Up to three teams per round, so the table has a fixed column for each slot.
const TEAM_SLOTS = [0, 1, 2]

/** "2m ago" / "just now" — how stale a room's last check-in is. */
function sinceLabel(checkIn: string | null): string {
  if (!checkIn) return '—'
  const seconds = Math.floor((Date.now() - new Date(checkIn).getTime()) / 1000)
  if (seconds < 10) return 'just now'
  if (seconds < 60) return `${seconds}s ago`
  const minutes = Math.floor(seconds / 60)
  return `${minutes}m ago`
}

// Every column is centred, headers included (the template centres headers already).
const columns: ColumnDef<GamesInProgressRow>[] = [
  { header: 'Division', align: 'center', render: (r) => r.division ?? '—' },
  { header: 'Room', align: 'center', render: (r) => r.room ?? '—' },
  { header: 'Round', align: 'center', render: (r) => r.round ?? '—' },
  { header: 'Question', align: 'center', render: (r) => r.question ?? '—' },
  // A name and a score per team slot; slots a game doesn't fill stay blank.
  ...TEAM_SLOTS.flatMap((slot): ColumnDef<GamesInProgressRow>[] => [
    { header: `Team #${slot + 1}`, align: 'center', render: (r) => r.teams[slot]?.name ?? '' },
    { header: 'Score', align: 'center', render: (r) => r.teams[slot]?.score ?? '' },
  ]),
  {
    header: 'Check-in',
    align: 'center',
    render: (r) => <span style={{ color: '#6b7280' }}>{sinceLabel(r.check_in)}</span>,
  },
]

/**
 * The live tickertape as a read-only DataTableTemplate. There is no create or delete for anyone:
 * these rows are derived from QuizMachine check-ins, not records a person edits.
 */
export const GamesInProgressTable = ({
  rows,
  loading,
}: {
  rows: GamesInProgressRow[]
  loading: boolean
}) => {
  const theme = useTheme()
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)

  // Updated live, and rooms drop off as they go quiet — so a page that was valid a moment
  // ago can now be past the end. Clamp rather than show an empty page.
  const lastPage = Math.max(0, Math.ceil(rows.length / pageSize) - 1)
  const currentPage = Math.min(page, lastPage)
  const pageRows = rows.slice(currentPage * pageSize, (currentPage + 1) * pageSize)

  return (
    <>
      <DataTableTemplate<GamesInProgressRow>
        entityLabel="Game"
        // The page renders its own heading above the table (see GamesInProgress).
        hideHeader
        // Match the global navbar, which is a default AppBar and so takes the theme's primary colour.
        headerStyle={{ background: theme.palette.primary.main, color: theme.palette.primary.contrastText }}
        showCreateButton={false}
        showDeleteButton={false}
        onDelete={async () => {}}
        loading={loading}
        columns={columns}
        rows={pageRows}
        totalCount={rows.length}
        getId={(r) => r.roomid}
        page={currentPage}
        pageSize={pageSize}
        onPageChange={setPage}
        onPageSizeChange={(size) => { setPageSize(size); setPage(0) }}
      />
      {!loading && rows.length === 0 && (
        <Typography variant="body2" color="text.secondary" sx={{ mt: 1.5 }}>
          No rooms are currently reporting. Rounds appear here while the QuizMachine clients are
          checking in, and drop off about 30 minutes after a room goes quiet.
        </Typography>
      )}
    </>
  )
}

export const GamesInProgress = () => {
  const { tid } = useParams()
  const { data, status, error } = useLiveFeed<GamesInProgressRow[]>({
    url: tid ? `/api/tournaments/${tid}/gamesinprogress/stream` : null,
    poll: () => GamesInProgressAPI.get(tid ?? ''),
    pollIntervalMs: POLL_MS,
  })
  const rows = data ?? []
  const loading = data === null && !error
  // Re-renders between updates so the "check-in" column ages even when the rows are unchanged.
  const [, setTick] = useState<number>(0)

  useEffect(() => {
    const tick = setInterval(() => setTick((t) => t + 1), TICK_MS)
    return () => clearInterval(tick)
  }, [])

  // Mounted only under /tournament/:tid/live-games, so a tid is always present.
  if (!tid) return null

  return (
    <div>
      <Box sx={{ mt: 2 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Typography variant="h5">Games in Progress</Typography>
          <LiveIndicator status={status} pollIntervalMs={POLL_MS} />
        </Box>
        {rows.length > 0 && (
          <Typography variant="subtitle1" color="text.secondary">
            {rows[0].tournament}
          </Typography>
        )}
      </Box>
      <br />
      {/* A failed refresh keeps showing the last rows; only a refusal, or having nothing, shows the error. */}
      {error && (status === 'error' || data === null) ? (
        <Paper sx={{ width: '100%', p: 3 }}>
          <Typography color="error">{error}</Typography>
        </Paper>
      ) : (
        <GamesInProgressTable rows={rows} loading={loading} />
      )}
    </div>
  )
}
