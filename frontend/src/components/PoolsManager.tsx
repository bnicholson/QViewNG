import { useCallback, useEffect, useMemo, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import Alert from '@mui/material/Alert'
import Divider from '@mui/material/Divider'
import Chip from '@mui/material/Chip'
import Tabs from '@mui/material/Tabs'
import Tab from '@mui/material/Tab'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import FormControl from '@mui/material/FormControl'
import Select, { type SelectChangeEvent } from '@mui/material/Select'
import MenuItem from '@mui/material/MenuItem'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'

import { useAuth } from '../hooks/useAuth'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { PoolBracketGroupAPI, type PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import { TeamAPI, type TeamTS, type TeamRowTS } from '../features/TeamAPI'
import { GameAPI, type GameRowTS } from '../features/GameAPI'
import { PoolDetail } from './PoolDetail'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'
import { AutoSchedulePoolsDialog } from './AutoSchedulePoolsDialog'

const PAGE = 0
const SIZE = 500

interface Props {
  tid: string
  did: string
  canEdit: boolean
  /** Called after any change (pool/group create/edit, team placement) so a host can refresh its data. */
  onChanged?: () => void
}

/**
 * The full "Pools" workflow for one division: pick a pool group, place teams into that group's pools
 * (each team in exactly one pool per group), and view each pool's Rooms × Rounds matrix. Self-contained
 * (loads and mutates its own data) so it can be reused wherever pool management is needed — the Edit V1
 * Pools tab and the Edit V2 "Edit Pool Groups" dialog both render this same component.
 */
export const PoolsManager = ({ tid, did, canEdit, onChanged }: Props) => {
  const navigate = useNavigate()
  const { accessToken } = useAuth()
  const [error, setError] = useState<string | null>(null)

  const [pools, setPools] = useState<PoolBracketTS[]>([])
  const [groups, setGroups] = useState<PoolBracketGroupTS[]>([])
  const [teams, setTeams] = useState<TeamTS[]>([])
  const [teamsByPool, setTeamsByPool] = useState<Record<string, TeamRowTS[]>>({})
  const [gamesByPool, setGamesByPool] = useState<Record<string, GameRowTS[]>>({})
  const [selectedGroupId, setSelectedGroupId] = useState('')
  const [selectedPoolId, setSelectedPoolId] = useState('')

  const [poolDialog, setPoolDialog] = useState<{ open: boolean; pool: PoolBracketTS | null }>({ open: false, pool: null })
  const [autoScheduleOpen, setAutoScheduleOpen] = useState(false)

  // Load per-pool team placements + games for the given pools.
  const loadPlacementAndGames = useCallback((forPools: PoolBracketTS[]) => {
    Promise.all(forPools.map(p => TeamAPI.getRowsByPoolBracket(p.pool_bracket_id, PAGE, SIZE).then(r => [p.pool_bracket_id, r.items] as const)))
      .then(entries => setTeamsByPool(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load team placements.'))
    Promise.all(forPools.map(p => GameAPI.getRowsByPoolBracket(p.pool_bracket_id, PAGE, SIZE).then(r => [p.pool_bracket_id, r.items] as const)))
      .then(entries => setGamesByPool(Object.fromEntries(entries)))
      .catch(() => setError('Failed to load games.'))
  }, [])

  const loadData = useCallback((divisionId: string) => {
    PoolBracketAPI.getByDivision(divisionId)
      .then(bs => {
        const onlyPools = bs.filter(b => b.type === 'pool')
        setPools(onlyPools)
        loadPlacementAndGames(onlyPools)
      })
      .catch(() => setError('Failed to load pools.'))
    PoolBracketGroupAPI.getByDivision(divisionId).then(setGroups).catch(() => setError('Failed to load pool groups.'))
    TeamAPI.getByDivision(divisionId, PAGE, SIZE).then(setTeams).catch(() => setError('Failed to load teams.'))
  }, [loadPlacementAndGames])

  useEffect(() => {
    setPools([]); setGroups([]); setTeams([]); setTeamsByPool({}); setGamesByPool({})
    setSelectedGroupId(''); setSelectedPoolId('')
    if (did) loadData(did)
  }, [did, loadData])

  // Auto-select the first group as soon as groups load, so the workflow shows data immediately.
  useEffect(() => {
    setSelectedGroupId(prev => groups.some(g => g.poolbracketgroupid === prev) ? prev : (groups[0]?.poolbracketgroupid ?? ''))
  }, [groups])

  const groupPools = useMemo(() => pools.filter(p => p.poolbracketgroupid === selectedGroupId), [pools, selectedGroupId])

  // Auto-select the first pool of the active group.
  useEffect(() => {
    setSelectedPoolId(prev => groupPools.some(p => p.pool_bracket_id === prev) ? prev : (groupPools[0]?.pool_bracket_id ?? ''))
  }, [groupPools])

  const placedTeamIds = useMemo(
    () => new Set(groupPools.flatMap(p => (teamsByPool[p.pool_bracket_id] ?? []).map(t => t.teamid))),
    [groupPools, teamsByPool]
  )
  const unplacedTeams = useMemo(() => teams.filter(t => !placedTeamIds.has(t.teamid)), [teams, placedTeamIds])
  const selectedPool = pools.find(p => p.pool_bracket_id === selectedPoolId) ?? null

  const refresh = () => { loadPlacementAndGames(pools); onChanged?.() }

  const handleAddTeam = async (teamid: string) => {
    if (!selectedPoolId) return
    try { await TeamAPI.addToPoolBracket(selectedPoolId, teamid, accessToken); refresh() }
    catch (e) { setError(e instanceof Error ? e.message : 'Failed to add team.') }
  }
  const handleRemoveTeam = async (poolId: string, teamid: string) => {
    try { await TeamAPI.removeFromPoolBracket(poolId, teamid, accessToken); refresh() }
    catch (e) { setError(e instanceof Error ? e.message : 'Failed to remove team.') }
  }
  const handleAddGroup = async () => {
    try {
      const group = await PoolBracketGroupAPI.create({ divisionid: did, name: `Group ${groups.length + 1}` }, accessToken)
      PoolBracketGroupAPI.getByDivision(did).then(gs => { setGroups(gs); setSelectedGroupId(group.poolbracketgroupid) }).catch(() => setError('Failed to load pool groups.'))
      onChanged?.()
    } catch (e) { setError(e instanceof Error ? e.message : 'Failed to create pool group.') }
  }

  return (
    <Box sx={{ textAlign: 'left' }}>
      {error && <Alert severity="error" sx={{ mb: 1 }} onClose={() => setError(null)}>{error}</Alert>}

      <Alert severity="info" sx={{ mb: 1.5 }}>
        Pools organize a Divisions Teams into groups for use in Round Robin-style scheduling. If you have many Teams in a Division and you want to schedule via Round Robin, use multiple Pools to reduce the number of Rounds overall that you require.
      </Alert>

      {/* Pool group selector — team placement is scoped to the chosen group. */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1.5, flexWrap: 'wrap' }}>
        <Typography variant="body2" sx={{ flexShrink: 0 }}>Pool Group:</Typography>
        {groups.length === 0 ? (
          <Typography variant="body2" color="text.secondary">No pool groups yet — add one to place teams.</Typography>
        ) : (
          <FormControl size="small" sx={{ minWidth: 160 }}>
            <Select
              value={groups.some(g => g.poolbracketgroupid === selectedGroupId) ? selectedGroupId : ''}
              onChange={(e: SelectChangeEvent) => setSelectedGroupId(e.target.value)}
              displayEmpty
            >
              {groups.map(g => <MenuItem key={g.poolbracketgroupid} value={g.poolbracketgroupid}>{g.name}</MenuItem>)}
            </Select>
          </FormControl>
        )}
        <Button size="small" startIcon={<AddIcon />} onClick={handleAddGroup} disabled={!canEdit}>Add Group</Button>
        <Typography variant="caption" color="text.secondary" sx={{ flexBasis: '100%' }}>
          Each team is placed in exactly one pool per group. A later group can re-pool the same teams differently.
        </Typography>
      </Box>

      {/* Teams awaiting placement in the selected group. */}
      <Box>
        <Typography variant="caption" color="text.secondary">
          Teams needing placement{selectedPool ? ` — click to add to the selected pool, "${selectedPool.name}"` : ' — create a pool first'}:
        </Typography>
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', mt: 0.5 }}>
          {unplacedTeams.length === 0
            ? <Typography variant="body2" color="text.secondary">All teams placed.</Typography>
            : unplacedTeams.map(t => (
                <Chip key={t.teamid} label={t.name} size="small"
                  onClick={canEdit && selectedPoolId ? () => handleAddTeam(t.teamid) : undefined}
                  disabled={!canEdit || !selectedPoolId} />
              ))
          }
        </Box>
      </Box>

      <Divider sx={{ my: 1.5 }} />

      <Box sx={{ display: 'flex', justifyContent: 'flex-start', mb: 1.5 }}>
        <Button variant="outlined" onClick={() => setAutoScheduleOpen(true)} disabled={!canEdit}>Auto-Schedule Pools</Button>
      </Box>
      <Divider sx={{ mb: 1.5 }} />

      {groupPools.length === 0 ? (
        <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
          <Typography variant="body2" color="text.secondary">This group has no pools yet:</Typography>
          <Button startIcon={<AddIcon />} onClick={() => setPoolDialog({ open: true, pool: null })} disabled={!canEdit || !selectedGroupId}>Create Pool</Button>
        </Box>
      ) : (
        <>
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Typography variant="body1" sx={{ mr: 1, flexShrink: 0 }}>Pools:</Typography>
            <Tabs
              value={groupPools.some(p => p.pool_bracket_id === selectedPoolId) ? selectedPoolId : false}
              onChange={(_e, value: string) => setSelectedPoolId(value)}
              variant="scrollable" scrollButtons="auto" sx={{ minHeight: 0 }}
            >
              {groupPools.map(p => <Tab key={p.pool_bracket_id} value={p.pool_bracket_id} label={p.name} />)}
            </Tabs>
            <Tooltip title="Edit Pool">
              <span>
                <IconButton size="small" onClick={() => setPoolDialog({ open: true, pool: selectedPool })} disabled={!canEdit || !selectedPool} aria-label="Edit Pool">
                  <EditIcon />
                </IconButton>
              </span>
            </Tooltip>
            <Tooltip title="Create Pool">
              <span>
                <IconButton size="small" onClick={() => setPoolDialog({ open: true, pool: null })} disabled={!canEdit || !selectedGroupId} aria-label="Create Pool">
                  <AddIcon />
                </IconButton>
              </span>
            </Tooltip>
          </Box>

          {selectedPool && (
            <PoolDetail
              teams={teamsByPool[selectedPool.pool_bracket_id] ?? []}
              games={gamesByPool[selectedPool.pool_bracket_id] ?? []}
              canEdit={canEdit}
              onRemoveTeam={(teamid) => handleRemoveTeam(selectedPool.pool_bracket_id, teamid)}
              onNavigateGame={(gid) => navigate(`/game/${gid}/overview`)}
            />
          )}
        </>
      )}

      <AutoSchedulePoolsDialog
        tid={tid}
        totalTeams={teams.length}
        isOpen={autoScheduleOpen}
        onCancel={() => setAutoScheduleOpen(false)}
      />

      <PoolBracketEditorDialog
        tid={tid}
        did={did}
        type="pool"
        entityLabel="Pool"
        bracket={poolDialog.pool}
        poolBracketGroupId={selectedGroupId || undefined}
        isOpen={poolDialog.open}
        onCancel={() => setPoolDialog({ open: false, pool: null })}
        onSave={(pool) => {
          setPoolDialog({ open: false, pool: null })
          PoolBracketAPI.getByDivision(did)
            .then(bs => { const op = bs.filter(b => b.type === 'pool'); setPools(op); setSelectedPoolId(pool.pool_bracket_id); loadPlacementAndGames(op) })
            .catch(() => setError('Failed to reload pools.'))
          onChanged?.()
        }}
      />
    </Box>
  )
}
