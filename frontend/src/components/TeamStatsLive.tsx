import { useEffect, useState } from 'react'
import Box from '@mui/material/Box'
import FormControl from '@mui/material/FormControl'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Paper from '@mui/material/Paper'
import Select from '@mui/material/Select'
import Typography from '@mui/material/Typography'
import { useTheme } from '@mui/material/styles'
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined'
import { DataTableTemplate, type ColumnDef } from './DataTableTemplate'
import { LiveIndicator } from './LiveIndicator'
import RankRuleDialog from './RankRuleDialog'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import type { TeamStatTS } from '../features/StatsGroupAPI'
import { useLiveFeed } from '../hooks/useLiveFeed'

// The standings are pushed live by the server whenever a game event lands in the division (see
// useLiveFeed). This is how often they are re-read instead while the live connection is down.
const POLL_MS = 30_000

async function fetchTeamStats(did: string): Promise<TeamStatTS[]> {
  const res = await fetch(`/api/divisions/${did}/teamstats`)
  if (!res.ok) throw new Error(`Failed to load team stats (${res.status})`)
  return res.json()
}

/**
 * Live team standings for one division of a tournament: the division's own stats group, counting
 * finished games only, with the tie-breaker rule that set each team's place.
 *
 * The selected division is owned by the caller (LIVE Stats keeps it in the URL, so a spectator
 * screen can be pointed straight at one division).
 */
export default function TeamStatsLive({
  tid,
  divisionId,
  onDivisionChange,
}: {
  tid: string
  divisionId: string | null
  onDivisionChange: (did: string) => void
}) {
  const theme = useTheme()
  const [divisions, setDivisions] = useState<DivisionTS[] | null>(null)
  const [divisionsError, setDivisionsError] = useState<string | null>(null)
  const [rankRuleTeam, setRankRuleTeam] = useState<TeamStatTS | null>(null)

  useEffect(() => {
    let cancelled = false
    DivisionAPI.getByTournament(tid, 0, 500)
      .then((list) => {
        if (cancelled) return
        setDivisions([...list].sort((a, b) => a.dname.localeCompare(b.dname, undefined, { numeric: true })))
      })
      .catch(() => { if (!cancelled) setDivisionsError('Failed to load the divisions.') })
    return () => { cancelled = true }
  }, [tid])

  // With no division chosen (or one that is not in this tournament), show the first.
  const selected = divisions?.find((d) => d.did === divisionId) ?? divisions?.[0] ?? null
  useEffect(() => {
    if (selected && selected.did !== divisionId) onDivisionChange(selected.did)
  }, [selected, divisionId, onDivisionChange])

  const { data, status, error } = useLiveFeed<TeamStatTS[]>({
    url: selected ? `/api/divisions/${selected.did}/teamstats/stream` : null,
    poll: () => fetchTeamStats(selected?.did ?? ''),
    pollIntervalMs: POLL_MS,
  })
  const rows = data ?? []
  const loading = divisions === null || (data === null && !error)

  const columns: ColumnDef<TeamStatTS>[] = [
    { header: 'Place', align: 'center', render: (r) => r.place },
    { header: 'Team Name', render: (r) => <span style={{ whiteSpace: 'nowrap' }}>{r.name}</span> },
    { header: 'Rounds', align: 'center', render: (r) => r.games },
    { header: 'Wins', align: 'center', render: (r) => r.wins },
    { header: 'Losses', align: 'center', render: (r) => r.losses },
    { header: 'Olympic Pts', align: 'center', render: (r) => r.olympic_points },
    { header: 'Mod Olympic', align: 'center', render: (r) => r.mod_olympic_points },
    { header: 'Avg Score', align: 'center', render: (r) => r.avg_points.toFixed(1) },
    {
      header: 'Rank Rules',
      align: 'center',
      render: (r) => (r.rank_rule_above || r.rank_rule_below) ? (
        <IconButton
          size="small"
          aria-label={`Why ${r.name} is ranked here`}
          title={`Why ${r.name} is ranked here`}
          onClick={() => setRankRuleTeam(r)}
        >
          <InfoOutlinedIcon fontSize="small" />
        </IconButton>
      ) : null,
    },
  ]

  return (
    <div>
      <Box sx={{ mb: 1.5 }}>
        {/* Equal side columns keep the title centered on the page; the indicator sits in the right one. */}
        <Box sx={{ display: 'grid', gridTemplateColumns: '1fr auto 1fr', alignItems: 'center' }}>
          <Typography variant="h5" sx={{ gridColumn: 2, textAlign: 'center' }}>
            Team Stats
          </Typography>
          <Box sx={{ justifySelf: 'end' }}>
            {selected && <LiveIndicator status={status} pollIntervalMs={POLL_MS} />}
          </Box>
        </Box>
      </Box>

      {divisionsError ? (
        <Paper sx={{ width: '100%', p: 3 }}>
          <Typography color="error">{divisionsError}</Typography>
        </Paper>
      ) : divisions !== null && divisions.length === 0 ? (
        <Typography variant="body2" color="text.secondary">This tournament has no divisions yet.</Typography>
      ) : (
        <>
          <FormControl size="small" sx={{ minWidth: 240, mb: 2 }}>
            <InputLabel id="team-stats-division-label">Division</InputLabel>
            <Select
              labelId="team-stats-division-label"
              label="Division"
              value={selected?.did ?? ''}
              onChange={(e) => onDivisionChange(String(e.target.value))}
            >
              {(divisions ?? []).map((d) => (
                <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>
              ))}
            </Select>
          </FormControl>

          {/* A failed refresh keeps showing the last rows; only a refusal, or having nothing, shows the error. */}
          {error && (status === 'error' || data === null) ? (
            <Paper sx={{ width: '100%', p: 3 }}>
              <Typography color="error">{error}</Typography>
            </Paper>
          ) : (
            <>
              <DataTableTemplate<TeamStatTS>
                entityLabel="Team"
                hideHeader
                headerStyle={{ background: theme.palette.primary.main, color: theme.palette.primary.contrastText }}
                showCreateButton={false}
                showDeleteButton={false}
                onDelete={async () => {}}
                loading={loading}
                columns={columns}
                rows={rows}
                totalCount={rows.length}
                getId={(r) => r.name}
                page={0}
                pageSize={rows.length || 1}
                onPageChange={() => {}}
                onPageSizeChange={() => {}}
              />
              {!loading && rows.length === 0 && (
                <Typography variant="body2" color="text.secondary" sx={{ mt: 1.5 }}>
                  No games in this division have finished yet. Teams appear here as their games end.
                </Typography>
              )}
            </>
          )}
        </>
      )}
      <RankRuleDialog team={rankRuleTeam} onClose={() => setRankRuleTeam(null)} />
    </div>
  )
}
