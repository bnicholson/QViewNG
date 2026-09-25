import React, { useMemo } from 'react'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import Paper from '@mui/material/Paper'
import dayjs from 'dayjs'
import { type GameRowTS } from '../features/GameAPI'
import { type TeamRowTS } from '../features/TeamAPI'

// Numeric-aware string compare so "Room 10" sorts after "Room 9", not right after "Room 1".
export const naturalCompare = (a: string, b: string) =>
  a.localeCompare(b, undefined, { numeric: true, sensitivity: 'base' })

interface PoolDetailProps {
  teams: TeamRowTS[]
  games: GameRowTS[]
  canEdit: boolean
  onRemoveTeam: (teamid: string) => void
  onNavigateGame: (gid: string) => void
}

/**
 * The detail for the selected pool/bracket (its name is shown by its tab): the teams in it (chips)
 * and a Rooms × Rounds matrix of the matchups played in it, rendered with each team's name. Renders
 * inline (no card of its own) since it lives inside the schedule card, below the tabs.
 */
export const PoolDetail = ({ teams, games, canEdit, onRemoveTeam, onNavigateGame }: PoolDetailProps) => {
  // Each team gets a 1-based number (its order in the pool), shown on its chip and in the matrix so
  // the compact matchup cells stay unambiguous.
  const numberByTeam = useMemo(() => {
    const m = new Map<string, number>()
    teams.forEach((t, i) => m.set(t.teamid, i + 1))
    return m
  }, [teams])

  // The scheduled start of the pool/bracket's first round — the earliest game start time (the
  // stored timestamps are ISO strings, so a lexical min is the chronological min).
  const firstRoundStart = useMemo(() => {
    const times = games.map(g => g.scheduled_start_time).filter((t): t is string => !!t)
    return times.length === 0 ? null : times.reduce((a, b) => (a < b ? a : b))
  }, [games])

  // Distinct rooms (X axis) and rounds (Y axis) taken from the games in this pool/bracket.
  const rooms = useMemo(() => {
    const m = new Map<string, string>()
    games.forEach(g => m.set(g.roomid, g.room_name))
    return [...m.entries()].sort((a, b) => naturalCompare(a[1], b[1]))
  }, [games])

  const rounds = useMemo(() => {
    const m = new Map<string, number | null>()
    games.forEach(g => m.set(g.roundid, g.round_number))
    return [...m.entries()].sort((a, b) => (a[1] ?? Number.MAX_SAFE_INTEGER) - (b[1] ?? Number.MAX_SAFE_INTEGER))
  }, [games])

  const gamesByCell = useMemo(() => {
    const m = new Map<string, GameRowTS[]>()
    games.forEach(g => {
      const key = `${g.roundid}|${g.roomid}`
      const list = m.get(key)
      if (list) list.push(g); else m.set(key, [g])
    })
    return m
  }, [games])

  // Teams that play in each round — anything in the pool not here is on a bye that round.
  const playedTeamsByRound = useMemo(() => {
    const m = new Map<string, Set<string>>()
    games.forEach(g => {
      let s = m.get(g.roundid)
      if (!s) { s = new Set(); m.set(g.roundid, s) }
      s.add(g.leftteamid)
      if (g.centerteamid) s.add(g.centerteamid)
      s.add(g.rightteamid)
    })
    return m
  }, [games])

  const byeTeamsForRound = (roundid: string) => {
    const played = playedTeamsByRound.get(roundid)
    return teams.filter(t => !played?.has(t.teamid))
  }

  // Only show the Bye column when at least one round actually has a team on a bye.
  const hasAnyBye = useMemo(
    () => rounds.some(([roundid]) => {
      const played = playedTeamsByRound.get(roundid)
      return teams.some(t => !played?.has(t.teamid))
    }),
    [rounds, teams, playedTeamsByRound]
  )

  // A single team's "N: name" (plain text — the whole matchup cell is the link, see below).
  const teamToken = (teamid: string, name: string) => {
    const n = numberByTeam.get(teamid)
    return <span key={teamid}>{n != null ? `${n}: ${name}` : name}</span>
  }

  // The whole cell's matchup text is one link to that game's profile.
  const matchup = (g: GameRowTS) => {
    const tokens: React.ReactNode[] = [teamToken(g.leftteamid, g.left_team_name)]
    if (g.centerteamid) tokens.push(teamToken(g.centerteamid, g.center_team_name ?? ''))
    tokens.push(teamToken(g.rightteamid, g.right_team_name))
    const withSeparators = tokens.reduce<React.ReactNode[]>((acc, tok, i) =>
      i === 0 ? [tok] : [...acc, <span key={`v${i}`}> v </span>, tok], [])
    return (
      <Box
        component="span"
        onClick={() => onNavigateGame(g.gid)}
        sx={{ cursor: 'pointer', color: 'primary.main', textDecoration: 'underline' }}
      >
        {withSeparators}
      </Box>
    )
  }

  return (
    <Box sx={{ mt: 1.5, textAlign: 'left' }}>
        {/* First-round start time (just under the tabs) */}
        <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
          Start time: {firstRoundStart ? dayjs(firstRoundStart).format('MMM D, YYYY h:mm A') : '—'}
        </Typography>

        {/* Teams in this pool/bracket (chips) */}
        <Typography variant="caption" color="text.secondary">
          Teams{canEdit ? ' — click a team to remove it from this pool/bracket' : ''}:
        </Typography>
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5, mb: 1 }}>
          {teams.length === 0
            ? <Typography variant="body2" color="text.secondary">No teams yet.</Typography>
            : teams.map((t, i) => (
                <Chip
                  key={t.teamid}
                  label={`${i + 1}: ${t.name}`}
                  size="small"
                  onClick={canEdit ? () => onRemoveTeam(t.teamid) : undefined}
                  onDelete={canEdit ? () => onRemoveTeam(t.teamid) : undefined}
                />
              ))
          }
        </Box>

        {/* Card Row 3 — Rooms × Rounds matchup matrix */}
        {games.length === 0 ? (
          <Typography variant="body2" color="text.secondary">No games scheduled.</Typography>
        ) : (
          <TableContainer component={Paper} variant="outlined" sx={{ overflowX: 'auto' }}>
            <Table size="small">
              <TableHead>
                <TableRow>
                  <TableCell sx={{ fontWeight: 600 }}>Round \ Room</TableCell>
                  {rooms.map(([roomid, roomName]) => (
                    <TableCell key={roomid} align="center" sx={{ fontWeight: 600 }}>Room: {roomName}</TableCell>
                  ))}
                  {hasAnyBye && <TableCell align="center" sx={{ fontWeight: 600 }}>Bye</TableCell>}
                </TableRow>
              </TableHead>
              <TableBody>
                {rounds.map(([roundid, roundNumber]) => (
                  <TableRow key={roundid}>
                    <TableCell sx={{ fontWeight: 600 }}>{roundNumber != null ? `Round: ${roundNumber}` : 'Round'}</TableCell>
                    {rooms.map(([roomid]) => {
                      const cellGames = gamesByCell.get(`${roundid}|${roomid}`) ?? []
                      return (
                        <TableCell key={roomid} align="center">
                          {cellGames.map((g, i) => (
                            <Box key={g.gid} sx={{ whiteSpace: 'nowrap' }}>
                              {i > 0 && <Divider sx={{ my: 0.5 }} />}
                              {matchup(g)}
                            </Box>
                          ))}
                        </TableCell>
                      )
                    })}
                    {hasAnyBye && (() => {
                      const byeTeams = byeTeamsForRound(roundid)
                      return (
                        <TableCell align="center" sx={{ color: 'text.secondary' }}>
                          {byeTeams.length === 0
                            ? '—'
                            : byeTeams.map(t => {
                                const n = numberByTeam.get(t.teamid)
                                return <Box key={t.teamid} sx={{ whiteSpace: 'nowrap' }}>{n != null ? `${n}: ${t.name}` : t.name}</Box>
                              })}
                        </TableCell>
                      )
                    })()}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        )}
    </Box>
  )
}
