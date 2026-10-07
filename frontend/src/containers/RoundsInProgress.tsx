import React from 'react'
import { useState, useEffect } from 'react'
import { useParams } from 'react-router'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import Paper from '@mui/material/Paper'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import CircularProgress from '@mui/material/CircularProgress'

// How often the tickertape re-reads the backend. The backend serves this from the Valkey
// cache, so a short interval is cheap: a warm read does no gameevents queries at all.
const POLL_MS = 15_000

/** One team's live standing within a round in progress. Mirrors the backend's RoundsInProgressTeam. */
export interface RoundsInProgressTeam {
  name: string
  score: number
}

/** One actively-reporting room. Mirrors the backend's RoundsInProgressRow. */
export interface RoundsInProgressRow {
  roomid: string
  tournament: string
  division: string | null
  room: string | null
  round: string | null
  question: number | null
  check_in: string | null
  teams: RoundsInProgressTeam[]
}

export const RoundsInProgressAPI = {
  get: async (tid: string): Promise<RoundsInProgressRow[]> => {
    const res = await fetch(`/api/tournaments/${tid}/roundsinprogress`)
    if (!res.ok) throw new Error(`Failed to load rounds in progress (${res.status})`)
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

export const RoundsInProgressTable = ({ rows }: { rows: RoundsInProgressRow[] }) => {
  if (rows.length === 0) {
    return (
      <Paper sx={{ width: '100%', p: 3 }}>
        <Typography color="text.secondary">
          No rooms are currently reporting. Rounds appear here while the QuizMachine clients
          are checking in, and drop off about 30 minutes after a room goes quiet.
        </Typography>
      </Paper>
    )
  }

  return (
    <Paper sx={{ width: '100%', overflow: 'hidden' }}>
      <TableContainer>
        <Table stickyHeader aria-label="rounds in progress">
          <TableHead>
            <TableRow>
              {['Division', 'Room', 'Round', 'Question'].map((label) => (
                <TableCell key={label} sx={{ backgroundColor: '#7aba7a', borderBottom: '2px solid black', fontSize: '1.1rem' }}>
                  {label}
                </TableCell>
              ))}
              {TEAM_SLOTS.map((slot) => (
                <React.Fragment key={slot}>
                  <TableCell sx={{ backgroundColor: '#7aba7a', borderBottom: '2px solid black', fontSize: '1.1rem' }}>
                    {`Team #${slot + 1}`}
                  </TableCell>
                  <TableCell align="right" sx={{ backgroundColor: '#7aba7a', borderBottom: '2px solid black', fontSize: '1.1rem' }}>
                    Score
                  </TableCell>
                </React.Fragment>
              ))}
              <TableCell align="right" sx={{ backgroundColor: '#7aba7a', borderBottom: '2px solid black', fontSize: '1.1rem' }}>
                Check-in
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {rows.map((row) => (
              <TableRow hover tabIndex={-1} key={row.roomid}>
                <TableCell>{row.division ?? '—'}</TableCell>
                <TableCell>{row.room ?? '—'}</TableCell>
                <TableCell>{row.round ?? '—'}</TableCell>
                <TableCell align="right">{row.question ?? '—'}</TableCell>
                {TEAM_SLOTS.map((slot) => {
                  const team = row.teams[slot]
                  return (
                    <React.Fragment key={slot}>
                      <TableCell>{team ? team.name : ''}</TableCell>
                      <TableCell align="right">{team ? team.score : ''}</TableCell>
                    </React.Fragment>
                  )
                })}
                <TableCell align="right" sx={{ color: 'text.secondary' }}>
                  {sinceLabel(row.check_in)}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Paper>
  )
}

export const RoundsInProgress = () => {
  const { tid } = useParams()
  const [rows, setRows] = useState<RoundsInProgressRow[]>([])
  const [loading, setLoading] = useState<boolean>(true)
  const [error, setError] = useState<string | null>(null)
  // Re-renders on each poll so the "check-in" column ages even when the rows are unchanged.
  const [, setTick] = useState<number>(0)

  useEffect(() => {
    if (!tid) {
      setLoading(false)
      return
    }
    let cancelled = false

    const load = () => {
      RoundsInProgressAPI.get(tid)
        .then((fetched) => {
          if (cancelled) return
          setRows(fetched)
          setError(null)
          setLoading(false)
        })
        .catch((e: Error) => {
          if (cancelled) return
          setError(e.message)
          setLoading(false)
        })
    }

    load()
    const poll = setInterval(load, POLL_MS)
    const tick = setInterval(() => setTick((t) => t + 1), POLL_MS)
    return () => {
      cancelled = true
      clearInterval(poll)
      clearInterval(tick)
    }
  }, [tid])

  // Reached via the tournament-less /rounds-in-progress path.
  if (!tid) {
    return (
      <Box sx={{ mt: 2 }}>
        <Typography variant="h5">Rounds In Progress</Typography>
        <Typography color="text.secondary" sx={{ mt: 2 }}>
          Open this view from a tournament to see its rounds in progress.
        </Typography>
      </Box>
    )
  }

  return (
    <div>
      <Box sx={{ mt: 2 }}>
        <Typography variant="h5">Rounds In Progress</Typography>
        {rows.length > 0 && (
          <Typography variant="subtitle1" color="text.secondary">
            {rows[0].tournament}
          </Typography>
        )}
      </Box>
      <br />
      {loading && <CircularProgress />}
      {error && !loading && (
        <Paper sx={{ width: '100%', p: 3 }}>
          <Typography color="error">{error}</Typography>
        </Paper>
      )}
      {!loading && !error && <RoundsInProgressTable rows={rows} />}
    </div>
  )
}
