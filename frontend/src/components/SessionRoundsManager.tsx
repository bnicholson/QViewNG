import { useState, useEffect, useCallback, useMemo, type Dispatch, type SetStateAction } from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import IconButton from '@mui/material/IconButton'
import Tooltip from '@mui/material/Tooltip'
import Chip from '@mui/material/Chip'
import Checkbox from '@mui/material/Checkbox'
import FormControlLabel from '@mui/material/FormControlLabel'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import Alert from '@mui/material/Alert'
import Snackbar from '@mui/material/Snackbar'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import DragIndicatorIcon from '@mui/icons-material/DragIndicator'
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward'
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward'
import ScheduleIcon from '@mui/icons-material/Schedule'
import VisibilityIcon from '@mui/icons-material/Visibility'
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff'
import dayjs from 'dayjs'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { RoundGroupAPI, type RoundGroupTS } from '../features/RoundGroupAPI'
import { type GameRowTS, type GameTS } from '../features/GameAPI'
import { GameAPI } from '../features/GameAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { PoolBracketGroupAPI, type PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import { TeamAPI } from '../features/TeamAPI'
import { PoolGroupsEditorDialog } from './PoolGroupsEditorDialog'
import { AutoSchedulePoolsDialog } from './AutoSchedulePoolsDialog'
import { RoundEditorDialog } from './RoundEditorDialog'
import { RoundGroupEditorDialog } from './RoundGroupEditorDialog'
import { GameEditorDialog } from './GameEditorDialog'
import { PoolBracketEditorDialog } from './PoolBracketEditorDialog'
import { SessionStartTimeDialog } from './SessionStartTimeDialog'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { useAuth } from '../hooks/useAuth'

const PAGE = 0
const SIZE = 500

// Each card "type" gets its own colour so the concentric nesting reads at a glance. Rounds use amber
// (not green) so they don't clash with the teal/green font of the linked games shown inside them.
const SESSION_CARD = { bg: '#eef2ff', border: '#c7d2fe', label: '#3730a3' }   // indigo
const ROUND_CARD = { bg: '#fff7ed', border: '#fed7aa', label: '#c2410c' }     // orange
const GROUP_CARD = { bg: '#faf5ff', border: '#e9d5ff', label: '#7e22ce' }     // purple (Pool/Bracket Groups view)

/** One game as shown under a round in the TimePool variant, with the pool it belongs to. */
export interface RoundGameEntry {
  g: GameRowTS
  poolName: string
  /** The game's pool bracket id (GameRowTS doesn't carry it) — needed to edit the game. */
  poolBracketId: string
}

function formatStart(iso: string | null): string {
  return iso ? dayjs(iso).format('MMM D, YYYY · h:mm A') : 'Unscheduled'
}

/** "Left v Center v Right" from a game row's team names. */
function matchupText(g: GameRowTS): string {
  return [g.left_team_name, g.center_team_name, g.right_team_name].filter(Boolean).join(' v ')
}

// Sort rounds by scheduled start (unscheduled last), then name.
function byStartThenName(a: RoundTS, b: RoundTS): number {
  if (a.scheduled_start_time && b.scheduled_start_time) {
    if (a.scheduled_start_time !== b.scheduled_start_time) return a.scheduled_start_time < b.scheduled_start_time ? -1 : 1
  } else if (a.scheduled_start_time) return -1
  else if (b.scheduled_start_time) return 1
  return a.name.localeCompare(b.name, undefined, { numeric: true })
}

interface Props {
  tid: string
  /** The division whose sessions/rounds are managed. */
  did: string
  /** The division's sessions (roundgroups). */
  roundgroups: RoundGroupTS[]
  canEdit: boolean
  /** Called after a session is created/renamed so the parent can reload the sessions list. */
  onRoundGroupsChanged: () => void
  /**
   * 'timing' (default) is the plain Sessions/Rounds manager with drag-and-drop.
   * 'timepool' also shows each round's games and moves rounds between adjacent sessions with
   * up/down arrows at the session edges instead of drag-and-drop.
   */
  variant?: 'timing' | 'timepool'
  /** TimePool only: games grouped by round id (with their pool name), shown under each round. */
  gamesByRound?: Map<string, RoundGameEntry[]>
  /** TimePool only: navigate to a game's profile when its matchup is clicked. */
  onNavigateGame?: (gid: string) => void
  /** TimePool only: called after a game is added/edited/deleted so the parent can reload games. */
  onGamesChanged?: () => void
  /**
   * TimePool only. Advanced mode (default) exposes Sessions, Pool/Bracket Groups and Pools. In
   * Normal mode the user manages Rounds and Games only: the rounds render as one flat chronological
   * list, the View toggle and the Pool/Pool-Group filter rows are gone, and the session / pool
   * groups / pools behind the schedule are created in the background and never named in the UI.
   */
  advanced?: boolean
  /** TimePool only: the division's pools/brackets, shown as editable chips above the sessions. */
  poolBrackets?: PoolBracketTS[]
  /** TimePool only: called after a pool/bracket is edited so the parent can reload it. */
  onPoolsChanged?: () => void
}

export default function SessionRoundsManager({ tid, did, roundgroups, canEdit, onRoundGroupsChanged, variant = 'timing', gamesByRound, onNavigateGame, onGamesChanged, advanced = true, poolBrackets, onPoolsChanged }: Props) {
  const { accessToken } = useAuth()
  const isTimePool = variant === 'timepool'
  // Normal scheduling mode: Rounds and Games only, with sessions/pool groups/pools kept out of sight.
  const normalMode = isTimePool && !advanced
  // TimePool: group the rounds/games either by their session (default) or by pool/bracket group.
  const [groupView, setGroupView] = useState<'sessions' | 'poolgroups'>('sessions')
  const [rounds, setRounds] = useState<RoundTS[]>([])
  const [notice, setNotice] = useState<string | null>(null)
  const [draggingId, setDraggingId] = useState<string | null>(null)
  const [dragOverSession, setDragOverSession] = useState<string | null>(null)

  // Round create/edit dialog: `session` = the session to create in; `round` = the round being edited.
  const [roundDialog, setRoundDialog] = useState<{ open: boolean; sessionId: string; round: RoundTS | null }>({ open: false, sessionId: '', round: null })
  // Session create/edit dialog.
  const [sessionDialog, setSessionDialog] = useState<{ open: boolean; roundgroup: RoundGroupTS | null }>({ open: false, roundgroup: null })
  // Game add/edit dialog (TimePool): `game` = the game being edited (null = add to `roundId`).
  const [gameDialog, setGameDialog] = useState<{ open: boolean; roundGroupId: string; roundId: string; game: GameRowTS | null; poolBracketId?: string }>({ open: false, roundGroupId: '', roundId: '', game: null })
  const closeGameDialog = () => setGameDialog({ open: false, roundGroupId: '', roundId: '', game: null })
  // Pool/bracket edit dialog (TimePool): opened from the pencil on the chips above the sessions.
  const [poolDialog, setPoolDialog] = useState<{ open: boolean; bracket: PoolBracketTS | null }>({ open: false, bracket: null })
  // "Edit Session Start Time" dialog — re-times all of a session's rounds from a chosen start.
  const [sessionTimeDialog, setSessionTimeDialog] = useState<{ open: boolean; session: RoundGroupTS | null }>({ open: false, session: null })
  // Normal mode's equivalent: re-times every round in the division from a chosen start.
  const [allRoundsTimeDialogOpen, setAllRoundsTimeDialogOpen] = useState(false)
  // Normal mode: the pool a hand-made game is attached to, resolved (or created) just before the
  // game dialog opens. Passed to the dialog, which hides the Pool/Bracket field entirely.
  const [normalPoolId, setNormalPoolId] = useState('')
  // Confirmation prompt for the cascading round/session deletes.
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState)
  // Games are controlled directly by pool/room/team visibility. Pool GROUPS control their pools (a
  // 2nd-degree effect on games): toggling a group off hides all of its pools, which hides their games.
  const [hiddenPoolIds, setHiddenPoolIds] = useState<Set<string>>(new Set())
  const [hiddenRoomIds, setHiddenRoomIds] = useState<Set<string>>(new Set())
  const [hiddenTeamIds, setHiddenTeamIds] = useState<Set<string>>(new Set())
  // When off, sessions/rounds that end up with no visible games (from filtering, or genuinely empty)
  // are hidden entirely. On (default) keeps them visible.
  const [showEmpty, setShowEmpty] = useState(true)
  // Toggle an id in one of the visual-filter sets (clicking a Pool/Room/Team chip hides it).
  const makeToggle = (setter: Dispatch<SetStateAction<Set<string>>>) => (id: string) =>
    setter(prev => {
      const next = new Set(prev)
      if (next.has(id)) next.delete(id); else next.add(id)
      return next
    })
  const togglePoolFilter = makeToggle(setHiddenPoolIds)
  const toggleRoomFilter = makeToggle(setHiddenRoomIds)
  const toggleTeamFilter = makeToggle(setHiddenTeamIds)
  // Set several pools hidden/shown at once — how a pool-group toggle cascades to its member pools.
  const setPoolsHidden = (ids: string[], hidden: boolean) => setHiddenPoolIds(prev => {
    const next = new Set(prev)
    ids.forEach(id => { if (hidden) next.add(id); else next.delete(id) })
    return next
  })
  // A game/round/session filter is active whenever any pool/room/team is hidden (i.e. "Reset / Show All"
  // is enabled). Creating while filtered is blocked so the user can't unknowingly duplicate a hidden game.
  const filtersActive = hiddenPoolIds.size > 0 || hiddenRoomIds.size > 0 || hiddenTeamIds.size > 0
  const createBlockedTooltip = filtersActive
    ? 'Filters are hiding some games. Click "Reset / Show All" first so you can see every game before creating.'
    : ''

  // TimePool: the division's pool groups (loaded locally) drive the "Pool Groups" filter row and the
  // "Edit Pool Groups" dialog. Each group maps to its member pools (from `poolBrackets`).
  const [poolGroups, setPoolGroups] = useState<PoolBracketGroupTS[]>([])
  const [poolGroupsDialogOpen, setPoolGroupsDialogOpen] = useState(false)
  const [autoScheduleOpen, setAutoScheduleOpen] = useState(false)
  const [divisionTeamCount, setDivisionTeamCount] = useState(0)
  const loadPoolGroups = useCallback(() => {
    if (!isTimePool) return
    PoolBracketGroupAPI.getByDivision(did).then(setPoolGroups).catch(() => setNotice('Failed to load pool groups.'))
  }, [isTimePool, did])
  useEffect(() => { loadPoolGroups() }, [loadPoolGroups])
  // The division's team count feeds the Auto-Schedule dialog's round math.
  useEffect(() => {
    if (!isTimePool) return
    TeamAPI.getByDivision(did, PAGE, SIZE).then(ts => setDivisionTeamCount(ts.length)).catch(() => {})
  }, [isTimePool, did])
  // groupId → its pool ids, and the set of all pools that belong to some group (for the row's eye).
  const poolIdsOfGroup = (groupId: string) => (poolBrackets ?? []).filter(pb => pb.poolbracketgroupid === groupId).map(pb => pb.pool_bracket_id)
  const allGroupedPoolIds = useMemo(
    () => (poolBrackets ?? []).filter(pb => pb.poolbracketgroupid != null).map(pb => pb.pool_bracket_id),
    [poolBrackets]
  )

  const loadRounds = useCallback(() => {
    RoundAPI.getByDivision(did, PAGE, SIZE)
      .then(setRounds)
      .catch(() => setNotice('Failed to load rounds.'))
  }, [did])

  useEffect(() => { loadRounds() }, [loadRounds])

  const roundsBySession = useMemo(() => {
    const m = new Map<string, RoundTS[]>()
    rounds.forEach(r => { const list = m.get(r.roundgroup_id); if (list) list.push(r); else m.set(r.roundgroup_id, [r]) })
    m.forEach(list => list.sort(byStartThenName))
    return m
  }, [rounds])

  // Deleting a round cascades on the backend (its games go too, and an emptied pool bracket group +
  // its pools are removed), so refresh rounds, pool groups, pools and games afterward.
  const handleDelete = async (round: RoundTS) => {
    try {
      await RoundAPI.delete(round.roundid, accessToken)
      loadRounds()
      loadPoolGroups()
      onPoolsChanged?.()
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete round.')
    }
  }

  // Deleting a session cascades on the backend (all its rounds + their games, and an emptied pool
  // bracket group + its pools), so refresh everything afterward.
  const handleDeleteSession = async (session: RoundGroupTS) => {
    try {
      await RoundGroupAPI.delete(session.roundgroup_id, accessToken)
      onRoundGroupsChanged()
      loadRounds()
      loadPoolGroups()
      onPoolsChanged?.()
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete session.')
    }
  }

  // Open a warning before the (irreversible, cascading) round delete.
  const confirmDeleteRound = (round: RoundTS) => {
    const gameCount = gamesByRound?.get(round.roundid)?.length ?? 0
    const gamesPart = gameCount > 0
      ? `all ${gameCount} game${gameCount === 1 ? '' : 's'} in it will be deleted`
      : 'it has no games'
    setConfirmDialog({
      isOpen: true,
      title: `Delete round "${round.name}"?`,
      message: normalMode
        ? `Deleting this round means ${gamesPart}. This can't be undone.`
        : `Deleting this round means ${gamesPart}. If this empties a Pool/Bracket Group (no games left), that group and its pools are deleted too. This can't be undone.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleDelete(round) },
    })
  }

  // Open a warning before the (irreversible, cascading) session delete.
  const confirmDeleteSession = (session: RoundGroupTS) => {
    const sessionRounds = roundsBySession.get(session.roundgroup_id) ?? []
    const roundCount = sessionRounds.length
    const gameCount = sessionRounds.reduce((sum, r) => sum + (gamesByRound?.get(r.roundid)?.length ?? 0), 0)
    setConfirmDialog({
      isOpen: true,
      title: `Delete session "${session.name}"?`,
      message: `Deleting this session will also delete all ${roundCount} of its round${roundCount === 1 ? '' : 's'} and their ${gameCount} game${gameCount === 1 ? '' : 's'}. If this empties a Pool/Bracket Group (no games left), that group and its pools are deleted too. This can't be undone.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleDeleteSession(session) },
    })
  }

  // Re-time a run of rounds: the first (chronological) one gets `startTimeIso`, and each later round
  // is spaced `durationMinutes` after the one before it, keeping the run evenly scheduled.
  const applyStartTimes = async (ordered: RoundTS[], startTimeIso: string, durationMinutes: number) => {
    try {
      for (let i = 0; i < ordered.length; i++) {
        const t = dayjs(startTimeIso).add(i * durationMinutes, 'minute').toISOString()
        await RoundAPI.update(ordered[i].roundid, { scheduled_start_time: t }, accessToken)
      }
      loadRounds()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to update the start times.')
    }
  }

  // Advanced mode: re-time one session's rounds (already sorted chronologically).
  const applySessionStartTime = (session: RoundGroupTS, startTimeIso: string, durationMinutes: number) =>
    applyStartTimes(roundsBySession.get(session.roundgroup_id) ?? [], startTimeIso, durationMinutes)

  // Move a round to another session (persists immediately). Used by both drag-and-drop (timing) and
  // the up/down arrows (timepool). The round keeps its start time, so the overall sequence is preserved.
  const moveRound = async (roundId: string, targetSessionId: string) => {
    const round = rounds.find(r => r.roundid === roundId)
    if (!round || round.roundgroup_id === targetSessionId) return
    try {
      await RoundAPI.update(roundId, { roundgroup_id: targetSessionId }, accessToken)
      loadRounds()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to move round.')
    }
  }

  const handleDeleteGame = async (gid: string) => {
    try {
      await GameAPI.delete(gid, accessToken)
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete game.')
    }
  }

  // Drop a round onto a session → move it there.
  const handleDropOnSession = async (targetSessionId: string, roundId: string) => {
    setDragOverSession(null)
    setDraggingId(null)
    await moveRound(roundId, targetSessionId)
  }

  // Order sessions by their earliest round's start time. Sessions with no (scheduled) rounds sort
  // to the end; ties fall back to name.
  const sortedSessions = useMemo(() => {
    const earliestStart = (roundgroupId: string): string | null => {
      // roundsBySession is already sorted (scheduled first, ascending), so the first scheduled round is the earliest.
      const list = roundsBySession.get(roundgroupId) ?? []
      return list.find(r => r.scheduled_start_time)?.scheduled_start_time ?? null
    }
    return [...roundgroups].sort((a, b) => {
      const ea = earliestStart(a.roundgroup_id)
      const eb = earliestStart(b.roundgroup_id)
      if (ea && eb) { if (ea !== eb) return ea < eb ? -1 : 1 }
      else if (ea) return -1
      else if (eb) return 1
      return a.name.localeCompare(b.name, undefined, { numeric: true })
    })
  }, [roundgroups, roundsBySession])

  // ── Normal mode: the hidden infrastructure ───────────────────────────────────
  // The user only ever creates Rounds and Games. A round still needs a session and a game still
  // needs a pool, so both are resolved from whatever the division already has and otherwise created
  // on the spot — silently, and never shown or named in the UI.

  /** The session new rounds are added to: the one holding the latest round, else the first; else a new one. */
  const resolveDefaultSession = async (): Promise<string> => {
    const latest = [...rounds].sort(byStartThenName).at(-1)
    if (latest) return latest.roundgroup_id
    if (sortedSessions.length > 0) return sortedSessions[0].roundgroup_id
    const created = await RoundGroupAPI.create({ did, name: 'Session 1' }, accessToken)
    onRoundGroupsChanged()
    return created.roundgroup_id
  }

  /** The pool a new game is attached to: the one this round's other games use, else the division's
   *  first pool; else a fresh pool (and the group it has to live in). */
  const resolveDefaultPool = async (round: RoundTS): Promise<string> => {
    const fromRound = gamesByRound?.get(round.roundid)?.[0]?.poolBracketId
    if (fromRound) return fromRound
    const existing = (poolBrackets ?? []).find(pb => pb.type === 'pool') ?? (poolBrackets ?? [])[0]
    if (existing) return existing.pool_bracket_id
    const group = await PoolBracketGroupAPI.create({ divisionid: did, name: 'Pool Group 1' }, accessToken)
    const pool = await PoolBracketAPI.create({
      divisionid: did, name: 'Pool A', type: 'pool', poolbracketgroupid: group.poolbracketgroupid,
    }, accessToken)
    loadPoolGroups()
    onPoolsChanged?.()
    return pool.pool_bracket_id
  }

  /** Normal mode's top-level "Add Round" — resolves the session first, then opens the round dialog. */
  const openAddRound = async () => {
    try {
      setRoundDialog({ open: true, sessionId: await resolveDefaultSession(), round: null })
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to prepare a new round.')
    }
  }

  /** Open the game dialog for a round. In Normal mode the pool is resolved (or created) first. */
  const openGameDialog = async (round: RoundTS, sessionId: string, game: GameRowTS | null, poolBracketId?: string) => {
    if (!normalMode) {
      setGameDialog({ open: true, roundGroupId: sessionId, roundId: round.roundid, game, poolBracketId })
      return
    }
    try {
      const pool = poolBracketId ?? await resolveDefaultPool(round)
      setNormalPoolId(pool)
      setGameDialog({ open: true, roundGroupId: sessionId, roundId: round.roundid, game, poolBracketId: pool })
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to prepare the game.')
    }
  }

  /**
   * Normal mode: a game's teams also join the pool behind it, since the user has no way to place
   * them there themselves and pool standings read from those placements. Already-placed teams make
   * the call fail harmlessly, so failures are ignored.
   */
  const placeGameTeamsInPool = async (game: GameTS, poolBracketId: string) => {
    const teamIds = [game.leftteamid, game.centerteamid, game.rightteamid].filter((t): t is string => !!t)
    await Promise.all(teamIds.map(t =>
      TeamAPI.addToPoolBracket(poolBracketId, t, accessToken).catch(() => { /* already placed */ })
    ))
  }

  // Normal mode shows every round in the division as one flat, chronological list.
  const allRoundsSorted = useMemo(() => [...rounds].sort(byStartThenName), [rounds])

  // Distinct rooms and teams that appear in the division's games — drive the Rooms/Teams filter chips.
  const roomsInGames = useMemo(() => {
    const m = new Map<string, string>()
    gamesByRound?.forEach(list => list.forEach(({ g }) => m.set(g.roomid, g.room_name)))
    return [...m.entries()].map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }))
  }, [gamesByRound])
  const teamsInGames = useMemo(() => {
    const m = new Map<string, string>()
    gamesByRound?.forEach(list => list.forEach(({ g }) => {
      m.set(g.leftteamid, g.left_team_name)
      if (g.centerteamid) m.set(g.centerteamid, g.center_team_name ?? '')
      m.set(g.rightteamid, g.right_team_name)
    }))
    return [...m.entries()].map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }))
  }, [gamesByRound])
  // Whether the division has any games at all — the filters are hidden until it does.
  const hasAnyGames = useMemo(() => {
    if (!gamesByRound) return false
    for (const list of gamesByRound.values()) if (list.length > 0) return true
    return false
  }, [gamesByRound])

  // A game is hidden if its pool or its room is toggled off, or — for teams — if EVERY team in the
  // game is toggled off. (A matchup stays visible as long as at least one of its teams is visible, so
  // showing a single team surfaces all of that team's games.)
  const gameHiddenByFilter = (e: RoundGameEntry): boolean => {
    if (hiddenPoolIds.has(e.poolBracketId) || hiddenRoomIds.has(e.g.roomid)) return true
    const teamIds = [e.g.leftteamid, e.g.rightteamid, ...(e.g.centerteamid != null ? [e.g.centerteamid] : [])]
    return teamIds.length > 0 && teamIds.every(id => hiddenTeamIds.has(id))
  }

  // Games for a round (visual filters applied), sorted by room name so the list reads in the same
  // room order as the Pools matrix.
  const gamesForRound = (roundId: string): RoundGameEntry[] =>
    [...(gamesByRound?.get(roundId) ?? [])]
      .filter(e => !gameHiddenByFilter(e))
      .sort((a, b) => a.g.room_name.localeCompare(b.g.room_name, undefined, { numeric: true }))

  // ── Pool/Bracket Group view (poolbracketgroup -> poolbracket -> game -> round) ──────────────
  // A game's pool bracket tells us its group; a round "belongs to" a group when it holds a game
  // whose pool is in that group.
  const groupIdOfPool = useMemo(() => {
    const m = new Map<string, string>()
    ;(poolBrackets ?? []).forEach(pb => { if (pb.poolbracketgroupid) m.set(pb.pool_bracket_id, pb.poolbracketgroupid) })
    return m
  }, [poolBrackets])
  // groupId → the rounds it contains (derived from the unfiltered games), sorted like sessions.
  const roundsByGroup = useMemo(() => {
    const roundById = new Map(rounds.map(r => [r.roundid, r]))
    const acc = new Map<string, Map<string, RoundTS>>()
    gamesByRound?.forEach((list, roundId) => {
      const round = roundById.get(roundId)
      if (!round) return
      list.forEach(e => {
        const gid = groupIdOfPool.get(e.poolBracketId)
        if (!gid) return
        let rm = acc.get(gid); if (!rm) { rm = new Map(); acc.set(gid, rm) }
        rm.set(roundId, round)
      })
    })
    const out = new Map<string, RoundTS[]>()
    acc.forEach((rm, gid) => out.set(gid, [...rm.values()].sort(byStartThenName)))
    return out
  }, [gamesByRound, rounds, groupIdOfPool])
  // Games for one (group, round) — the round's visible games narrowed to this group's pools.
  const gamesForGroupRound = (groupId: string, roundId: string): RoundGameEntry[] =>
    gamesForRound(roundId).filter(e => groupIdOfPool.get(e.poolBracketId) === groupId)
  // Session name for a round's roundgroup, shown on the round card in the group view.
  const sessionNameById = useMemo(() => {
    const m = new Map<string, string>()
    roundgroups.forEach(rg => m.set(rg.roundgroup_id, rg.name))
    return m
  }, [roundgroups])
  // Order pool groups by their earliest round's start (unscheduled last), then name — like sessions.
  const sortedGroups = useMemo(() => {
    const earliestStart = (gid: string): string | null =>
      (roundsByGroup.get(gid) ?? []).find(r => r.scheduled_start_time)?.scheduled_start_time ?? null
    return [...poolGroups].sort((a, b) => {
      const ea = earliestStart(a.poolbracketgroupid)
      const eb = earliestStart(b.poolbracketgroupid)
      if (ea && eb) { if (ea !== eb) return ea < eb ? -1 : 1 }
      else if (ea) return -1
      else if (eb) return 1
      return a.name.localeCompare(b.name, undefined, { numeric: true })
    })
  }, [poolGroups, roundsByGroup])

  // A leading "show/hide all" eye chip for a filter row. Open eye when anything is hidden (click reveals
  // all); closed eye when everything is visible (click hides all).
  const allToggleChip = (noun: string, allIds: string[], hiddenIds: Set<string>, setHidden: Dispatch<SetStateAction<Set<string>>>) => {
    const anyHidden = allIds.some(id => hiddenIds.has(id))
    return (
      <Tooltip title={anyHidden ? `Show all ${noun}` : `Hide all ${noun}`}>
        <Chip
          variant="outlined"
          onClick={() => setHidden(anyHidden ? new Set() : new Set(allIds))}
          sx={{ height: 'auto', borderRadius: '8px', cursor: 'pointer', '& .MuiChip-label': { display: 'flex', alignItems: 'center', px: 0.75, py: 0.5 } }}
          label={anyHidden ? <VisibilityIcon sx={{ fontSize: 16 }} /> : <VisibilityOffIcon sx={{ fontSize: 16 }} />}
        />
      </Tooltip>
    )
  }

  // One round card. Shared by the Sessions and Pool/Bracket Group views so both read identically;
  // `showPoolLine` (the game chip's "Pool:" row) is off in the group view, and `sessionLabel` adds a
  // "Session: {name}" tag after the round's datetime there. `addGameSessionId` is the roundgroup an
  // added game is locked to; `draggable` is only the plain timing variant's drag-to-move-session.
  const renderRoundCard = (
    round: RoundTS,
    roundGames: RoundGameEntry[],
    opts: { showPoolLine: boolean; sessionLabel?: string; addGameSessionId: string; draggable: boolean },
  ) => {
    const { showPoolLine, sessionLabel, addGameSessionId, draggable } = opts
    // A round with any game (ignoring the pool-visibility filter) can't be deleted; it also gates the
    // "No games in this round." message (only shown when the round is truly empty).
    const roundHasGames = (gamesByRound?.get(round.roundid)?.length ?? 0) > 0
    const roundHasAnyGames = isTimePool && roundHasGames
    return (
      <Box
        key={round.roundid}
        draggable={draggable}
        onDragStart={draggable ? (e) => { e.dataTransfer.setData('text/plain', round.roundid); e.dataTransfer.effectAllowed = 'move'; setDraggingId(round.roundid) } : undefined}
        onDragEnd={draggable ? () => { setDraggingId(null); setDragOverSession(null) } : undefined}
        sx={{
          display: 'flex', flexDirection: 'column', gap: 0.5,
          bgcolor: ROUND_CARD.bg,
          border: `1px solid ${ROUND_CARD.border}`,
          borderRadius: 1.5, px: 1, py: 0.75,
          opacity: draggingId === round.roundid ? 0.5 : 1,
          cursor: draggable ? 'grab' : 'default',
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {draggable && <DragIndicatorIcon fontSize="small" sx={{ color: '#9ca3af' }} />}
          <Box sx={{ flex: 1, minWidth: 0, display: 'flex', alignItems: 'baseline', gap: 1, flexWrap: 'wrap' }}>
            <Typography variant="body2" sx={{ fontWeight: 600, color: ROUND_CARD.label }}>Round: {round.name}</Typography>
            <Typography variant="caption" color="text.secondary">{formatStart(round.scheduled_start_time)}</Typography>
            {sessionLabel && <Typography variant="caption" color="text.secondary">Session: {sessionLabel}</Typography>}
          </Box>
          {canEdit && (
            <>
              {isTimePool && (
                <Tooltip title={createBlockedTooltip}>
                  <span>
                    <Button size="small" startIcon={<AddIcon />} disabled={filtersActive} onClick={() => openGameDialog(round, addGameSessionId, null)}>
                      Add Game
                    </Button>
                  </span>
                </Tooltip>
              )}
              <Button size="small" startIcon={<EditIcon />} onClick={() => setRoundDialog({ open: true, sessionId: round.roundgroup_id, round })}>
                Edit Round
              </Button>
              <Button size="small" color="error" startIcon={<DeleteIcon />} onClick={() => confirmDeleteRound(round)}>
                Delete Round
              </Button>
            </>
          )}
        </Box>

        {/* TimePool: the round's games, keeping the Pools matrix's matchups visible. A round whose
            games are all hidden by the filter renders nothing; "No games in this round." only shows
            when the round is truly empty. */}
        {isTimePool && (
          !roundHasAnyGames ? (
            <Typography variant="caption" color="text.secondary" sx={{ pl: 0.5 }}>No games in this round.</Typography>
          ) : roundGames.length === 0 ? null : (
            // Games as wrapping chips so they fill the row's whitespace and cut down on vertical scrolling.
            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, pl: 0.5 }}>
              {roundGames.map(({ g, poolName, poolBracketId }) => (
                // No icons on the chip: clicking it opens the game editor, where the user can edit,
                // delete, or view the game's profile.
                <Chip
                  key={g.gid}
                  variant="outlined"
                  onClick={() => openGameDialog(round, addGameSessionId, g, poolBracketId)}
                  sx={{
                    height: 'auto',
                    borderRadius: '8px',
                    bgcolor: 'rgba(0, 128, 128, 0.08)',
                    borderColor: 'primary.main',
                    color: 'primary.main',
                    cursor: 'pointer',
                    '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                  }}
                  label={
                    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.25 }}>
                      {/* Row 1: room */}
                      <Typography component="span" variant="caption" sx={{ fontWeight: 600, color: 'primary.main', whiteSpace: 'nowrap', lineHeight: 1 }}>
                        Room: {g.room_name}
                      </Typography>
                      {/* Row 2: teams */}
                      <Typography component="span" variant="caption" sx={{ color: 'primary.main', lineHeight: 1.2 }}>
                        {matchupText(g)}
                      </Typography>
                      {/* Row 3: pool (Sessions view only — the group view is already grouped by pool) */}
                      {showPoolLine && poolName && (
                        <Typography component="span" variant="caption" sx={{ color: 'text.secondary', lineHeight: 1 }}>
                          Pool: {poolName}
                        </Typography>
                      )}
                    </Box>
                  }
                />
              ))}
            </Box>
          )
        )}
      </Box>
    )
  }

  return (
    <Box sx={{ textAlign: 'left' }}>
      {normalMode ? (
        <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
          Build your schedule here: create the Rounds that make up your event's time, then add the Games played in each Round. Or use "Create Schedule" to generate a whole round-robin at once.
        </Alert>
      ) : isTimePool ? (
        <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
          This tab combines your schedule's time (Sessions and Rounds) with the games scheduled in each round. Use the up/down arrows at the top and bottom edges of a session to move its boundary round into the neighbouring session.
        </Alert>
      ) : (
        <>
          <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
            This tab lets you manage time by modifying your Rounds and Sessions. Use Sessions to organize your Rounds into sequential groups. (There are usually longer-than-normal breaks of no quizzing in between each Session.)
          </Alert>
          <Box sx={{ mb: 1 }}>
            <Typography variant="body2" color="text.secondary">
              Drag a round from one session onto another to move it. Rounds with a game can't be deleted.
            </Typography>
          </Box>
        </>
      )}

      {/* Auto-Schedule — sits just above the filters. In Normal mode the per-session "Add Round" and
          "Edit Session Start Time" buttons have nowhere to live (there are no session cards), so
          their schedule-wide equivalents sit here beside it. */}
      {isTimePool && canEdit && (
        <Box sx={{ mb: 1.5, display: 'flex', alignItems: 'center', gap: 1, flexWrap: 'wrap' }}>
          <Button variant="contained" disableElevation onClick={() => setAutoScheduleOpen(true)}>Create Schedule</Button>
          {normalMode && (
            <>
              <Tooltip title={createBlockedTooltip}>
                <span>
                  <Button size="small" startIcon={<AddIcon />} disabled={filtersActive} onClick={openAddRound}>
                    Add Round
                  </Button>
                </span>
              </Tooltip>
              <Tooltip title={rounds.length === 0 ? 'Add a round first to set a start time.' : ''}>
                <span>
                  <Button size="small" startIcon={<ScheduleIcon />} disabled={rounds.length === 0} onClick={() => setAllRoundsTimeDialogOpen(true)}>
                    Edit Start Times
                  </Button>
                </span>
              </Tooltip>
            </>
          )}
        </Box>
      )}

      {/* View — group the schedule below by Session (default) or by Pool/Bracket Group. Normal mode
          has only one way to read the schedule (a flat list of rounds), so there's nothing to pick. */}
      {isTimePool && !normalMode && (
        <Box sx={{ mb: 1.5, display: 'flex', alignItems: 'center', gap: 1, flexWrap: 'wrap' }}>
          <Typography variant="body2" sx={{ fontWeight: 600 }}>View:</Typography>
          <ToggleButtonGroup
            size="small"
            exclusive
            value={groupView}
            onChange={(_, v) => { if (v) setGroupView(v) }}
          >
            <ToggleButton value="sessions">Sessions</ToggleButton>
            <ToggleButton value="poolgroups">Pool/Bracket Groups</ToggleButton>
          </ToggleButtonGroup>
        </Box>
      )}

      {/* Filters card — client-side show/hide of the games below by Pool Group, Pool, Room, or Team.
          Hidden until the division actually has games (which implies it has sessions/rounds too). */}
      {isTimePool && hasAnyGames && (
        <Card variant="outlined" sx={{ mb: 1.5 }}>
          <CardContent sx={{ pb: '12px !important' }}>
            <Typography variant="subtitle2" sx={{ fontWeight: 700, mb: 1 }}>Filter:</Typography>
            {/* Reset — clears the Pool/Room/Team hide filters so every game shows again. */}
            <Box sx={{ mb: 1.5 }}>
              <Button
                size="small"
                variant="contained"
                color="primary"
                disableElevation
                disabled={hiddenPoolIds.size === 0 && hiddenRoomIds.size === 0 && hiddenTeamIds.size === 0 && showEmpty}
                onClick={() => { setHiddenPoolIds(new Set()); setHiddenRoomIds(new Set()); setHiddenTeamIds(new Set()); setShowEmpty(true) }}
              >
                Reset / Show All
              </Button>
            </Box>

      {/* Pool groups — filter chips (like the rows below) plus an Edit chip that opens the editor. */}
      {isTimePool && !normalMode && (poolGroups.length > 0 || canEdit) && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Pool Groups:</Typography>
          {/* {canEdit && (
            <Tooltip title="Edit pool groups">
              <Chip
                variant="outlined"
                icon={<EditIcon sx={{ fontSize: 15 }} />}
                label="Edit"
                onClick={() => setPoolGroupsDialogOpen(true)}
                sx={{ height: 'auto', borderRadius: '8px', cursor: 'pointer', '& .MuiChip-label': { px: 0.75, py: 0.5 } }}
              />
            </Tooltip>
          )} */}
          {(() => {
            // Show/hide-all eye for the whole row: operates on every grouped pool (via hiddenPoolIds).
            const anyHidden = allGroupedPoolIds.some(id => hiddenPoolIds.has(id))
            return (
              <Tooltip title={anyHidden ? 'Show all pool groups' : 'Hide all pool groups'}>
                <Chip
                  variant="outlined"
                  onClick={() => setPoolsHidden(allGroupedPoolIds, !anyHidden)}
                  sx={{ height: 'auto', borderRadius: '8px', cursor: 'pointer', '& .MuiChip-label': { display: 'flex', alignItems: 'center', px: 0.75, py: 0.5 } }}
                  label={anyHidden ? <VisibilityIcon sx={{ fontSize: 16 }} /> : <VisibilityOffIcon sx={{ fontSize: 16 }} />}
                />
              </Tooltip>
            )
          })()}
          {poolGroups.map(group => {
            // A group is "off" when all of its pools are hidden; clicking flips its pools' visibility.
            const poolIds = poolIdsOfGroup(group.poolbracketgroupid)
            const hidden = poolIds.length > 0 && poolIds.every(id => hiddenPoolIds.has(id))
            return (
              <Chip
                key={group.poolbracketgroupid}
                variant="outlined"
                onClick={() => setPoolsHidden(poolIds, !hidden)}
                sx={{
                  height: 'auto', borderRadius: '8px', cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                    {group.name}
                  </Typography>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Pools/brackets for the division — editable chips above the first session. */}
      {isTimePool && !normalMode && poolBrackets && poolBrackets.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Pools:</Typography>
          {allToggleChip('pools', poolBrackets.map(pb => pb.pool_bracket_id), hiddenPoolIds, setHiddenPoolIds)}
          {poolBrackets.map(pb => {
            // Clicking the chip body toggles whether this pool's games show below; the pencil edits it.
            const hidden = hiddenPoolIds.has(pb.pool_bracket_id)
            return (
              <Chip
                key={pb.pool_bracket_id}
                variant="outlined"
                onClick={() => togglePoolFilter(pb.pool_bracket_id)}
                sx={{
                  height: 'auto',
                  borderRadius: '8px',
                  cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                    <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                      {pb.name}
                    </Typography>
                    {canEdit && (
                      <Tooltip title={`Edit ${pb.type === 'bracket' ? 'bracket' : 'pool'}`}>
                        <IconButton size="small" sx={{ p: 0.25 }} onClick={(e) => { e.stopPropagation(); setPoolDialog({ open: true, bracket: pb }) }}>
                          <EditIcon sx={{ fontSize: 15 }} />
                        </IconButton>
                      </Tooltip>
                    )}
                  </Box>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Rooms filter — click a room chip to show/hide its games below. */}
      {isTimePool && roomsInGames.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Rooms:</Typography>
          {allToggleChip('rooms', roomsInGames.map(r => r.id), hiddenRoomIds, setHiddenRoomIds)}
          {roomsInGames.map(room => {
            const hidden = hiddenRoomIds.has(room.id)
            return (
              <Chip
                key={room.id}
                variant="outlined"
                onClick={() => toggleRoomFilter(room.id)}
                sx={{
                  height: 'auto', borderRadius: '8px', cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                    {room.name}
                  </Typography>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Teams filter — click a team chip to show/hide the games that team plays in. */}
      {isTimePool && teamsInGames.length > 0 && (
        <Box sx={{ mb: 1.5, display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 0.5 }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mr: 0.5 }}>Teams:</Typography>
          {allToggleChip('teams', teamsInGames.map(t => t.id), hiddenTeamIds, setHiddenTeamIds)}
          {teamsInGames.map(team => {
            const hidden = hiddenTeamIds.has(team.id)
            return (
              <Chip
                key={team.id}
                variant="outlined"
                onClick={() => toggleTeamFilter(team.id)}
                sx={{
                  height: 'auto', borderRadius: '8px', cursor: 'pointer',
                  opacity: hidden ? 0.45 : 1,
                  bgcolor: hidden ? 'transparent' : 'rgba(0, 128, 128, 0.08)',
                  '& .MuiChip-label': { display: 'block', px: 1, py: 0.5 },
                }}
                label={
                  <Typography component="span" variant="caption" sx={{ fontWeight: 600, whiteSpace: 'nowrap', lineHeight: 1, textDecoration: hidden ? 'line-through' : 'none' }}>
                    {team.name}
                  </Typography>
                }
              />
            )
          })}
        </Box>
      )}

      {/* Bottom row: whether to keep empty (or filtered-to-empty) sessions/rounds visible. */}
      <FormControlLabel
        sx={{ mt: 0.5 }}
        control={<Checkbox size="small" checked={showEmpty} onChange={(e) => setShowEmpty(e.target.checked)} />}
        label={<Typography variant="body2">{normalMode ? 'Show Rounds When Empty' : 'Show Sessions/Rounds When Empty'}</Typography>}
      />
          </CardContent>
        </Card>
      )}

      {/* Normal mode — every round in the division as one flat, chronological list. No session
          wrappers, no "Pool:" line on the game chips, and no drag-to-another-session. */}
      {normalMode && (() => {
        const displayRounds = showEmpty ? allRoundsSorted : allRoundsSorted.filter(r => gamesForRound(r.roundid).length > 0)
        if (allRoundsSorted.length === 0) {
          return <Typography variant="body2" color="text.secondary">No rounds yet. Use "Create Schedule" to build a whole schedule, or "Add Round" to start one round at a time.</Typography>
        }
        if (displayRounds.length === 0) {
          return <Typography variant="body2" color="text.secondary">No rounds have a visible game. Clear the filters, or tick "Show Rounds When Empty".</Typography>
        }
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            {displayRounds.map(round => renderRoundCard(
              round,
              gamesForRound(round.roundid),
              { showPoolLine: false, addGameSessionId: round.roundgroup_id, draggable: false },
            ))}
          </Box>
        )
      })()}

      {!normalMode && groupView === 'sessions' && (sortedSessions.length === 0 ? (
        <Typography variant="body2" color="text.secondary">This division has no sessions yet.</Typography>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          {sortedSessions.map((session, sessionIdx) => {
            const sessionRounds = roundsBySession.get(session.roundgroup_id) ?? []
            // With "Show ... When Empty" off, drop rounds that have no visible games, and skip a
            // session entirely when none of its rounds have any visible games.
            const hideEmpty = isTimePool && !showEmpty
            const displayRounds = hideEmpty ? sessionRounds.filter(r => gamesForRound(r.roundid).length > 0) : sessionRounds
            if (hideEmpty && displayRounds.length === 0) return null
            const isDropTarget = !isTimePool && dragOverSession === session.roundgroup_id && draggingId != null
            const prevSession = sortedSessions[sessionIdx - 1]
            const nextSession = sortedSessions[sessionIdx + 1]
            // Arrows only move a session's boundary round into an adjacent session.
            const canMoveUp = isTimePool && canEdit && prevSession != null && sessionRounds.length > 0
            const canMoveDown = isTimePool && canEdit && nextSession != null && sessionRounds.length > 0
            return (
              <Box
                key={session.roundgroup_id}
                onDragOver={!isTimePool ? (e) => { e.preventDefault(); setDragOverSession(session.roundgroup_id) } : undefined}
                onDragLeave={!isTimePool ? () => setDragOverSession(prev => prev === session.roundgroup_id ? null : prev) : undefined}
                onDrop={!isTimePool ? (e) => { e.preventDefault(); handleDropOnSession(session.roundgroup_id, e.dataTransfer.getData('text/plain')) } : undefined}
                sx={{
                  bgcolor: SESSION_CARD.bg,
                  border: `2px solid ${isDropTarget ? SESSION_CARD.label : SESSION_CARD.border}`,
                  borderRadius: 2, p: 1.5,
                  transition: 'border-color 120ms',
                }}
              >
                {/* Session label + actions */}
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
                  <Typography variant="subtitle2" sx={{ flex: 1, fontWeight: 700, color: SESSION_CARD.label }}>
                    Session: {session.name}
                  </Typography>
                  {canEdit && (
                    <>
                      {isTimePool && (
                        <Tooltip title={sessionRounds.length === 0 ? 'Add a round first to set a start time.' : ''}>
                          <span>
                            <Button size="small" startIcon={<ScheduleIcon />} disabled={sessionRounds.length === 0} onClick={() => setSessionTimeDialog({ open: true, session })}>
                              Edit Session Start Time
                            </Button>
                          </span>
                        </Tooltip>
                      )}
                      <Tooltip title={createBlockedTooltip}>
                        <span>
                          <Button size="small" startIcon={<AddIcon />} disabled={filtersActive} onClick={() => setRoundDialog({ open: true, sessionId: session.roundgroup_id, round: null })}>
                            Add Round
                          </Button>
                        </span>
                      </Tooltip>
                      <Button size="small" startIcon={<EditIcon />} onClick={() => setSessionDialog({ open: true, roundgroup: session })}>
                        Edit Session
                      </Button>
                      <Button size="small" color="error" startIcon={<DeleteIcon />} onClick={() => confirmDeleteSession(session)}>
                        Delete Session
                      </Button>
                    </>
                  )}
                </Box>

                {/* Up arrow — move this session's first (top) round up into the previous session. */}
                {canMoveUp && (
                  <Box sx={{ display: 'flex', justifyContent: 'center', mb: 0.5 }}>
                    <Tooltip title={`Move “${sessionRounds[0].name}” up to ${prevSession.name}`}>
                      <IconButton size="small" onClick={() => moveRound(sessionRounds[0].roundid, prevSession.roundgroup_id)}>
                        <ArrowUpwardIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  </Box>
                )}

                {/* Round cards (concentric, own colour) */}
                {displayRounds.length === 0 ? (
                  <Typography variant="caption" color="text.secondary">No rounds yet.</Typography>
                ) : (
                  <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                    {displayRounds.map(round => renderRoundCard(
                      round,
                      isTimePool ? gamesForRound(round.roundid) : [],
                      { showPoolLine: true, addGameSessionId: session.roundgroup_id, draggable: canEdit && !isTimePool },
                    ))}
                  </Box>
                )}

                {/* Down arrow — move this session's last (bottom) round down into the next session. */}
                {canMoveDown && (
                  <Box sx={{ display: 'flex', justifyContent: 'center', mt: 0.5 }}>
                    <Tooltip title={`Move “${sessionRounds[sessionRounds.length - 1].name}” down to ${nextSession.name}`}>
                      <IconButton size="small" onClick={() => moveRound(sessionRounds[sessionRounds.length - 1].roundid, nextSession.roundgroup_id)}>
                        <ArrowDownwardIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  </Box>
                )}
              </Box>
            )
          })}
        </Box>
      ))}

      {/* Pool/Bracket Group view — the same round cards grouped by pool/bracket group instead of
          session (poolbracketgroup -> poolbracket -> game -> round). Group cards are purple. */}
      {!normalMode && groupView === 'poolgroups' && (sortedGroups.length === 0 ? (
        <Typography variant="body2" color="text.secondary">This division has no pool/bracket groups yet.</Typography>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          {sortedGroups.map(group => {
            const groupRounds = roundsByGroup.get(group.poolbracketgroupid) ?? []
            // With "Show ... When Empty" off, drop rounds with no visible games (and skip empty groups).
            const hideEmpty = !showEmpty
            const displayRounds = hideEmpty
              ? groupRounds.filter(r => gamesForGroupRound(group.poolbracketgroupid, r.roundid).length > 0)
              : groupRounds
            if (hideEmpty && displayRounds.length === 0) return null
            return (
              <Box
                key={group.poolbracketgroupid}
                sx={{
                  bgcolor: GROUP_CARD.bg,
                  border: `2px solid ${GROUP_CARD.border}`,
                  borderRadius: 2, p: 1.5,
                }}
              >
                {/* Group label */}
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
                  <Typography variant="subtitle2" sx={{ flex: 1, fontWeight: 700, color: GROUP_CARD.label }}>
                    Pool/Bracket Group: {group.name}
                  </Typography>
                </Box>

                {displayRounds.length === 0 ? (
                  <Typography variant="caption" color="text.secondary">No rounds yet.</Typography>
                ) : (
                  <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                    {displayRounds.map(round => renderRoundCard(
                      round,
                      gamesForGroupRound(group.poolbracketgroupid, round.roundid),
                      { showPoolLine: false, sessionLabel: sessionNameById.get(round.roundgroup_id) ?? '', addGameSessionId: round.roundgroup_id, draggable: false },
                    ))}
                  </Box>
                )}
              </Box>
            )
          })}
        </Box>
      ))}

      {/* Add a new session — a "+" button directly below the last session, left-aligned (Sessions view). */}
      {/* {canEdit && groupView === 'sessions' && (
        <Box sx={{ mt: 1.5 }}>
          <Tooltip title={filtersActive ? createBlockedTooltip : 'Add session'}>
            <span>
              <Button size="small" variant="outlined" sx={{ minWidth: 40, px: 1 }}
                disabled={filtersActive}
                onClick={() => setSessionDialog({ open: true, roundgroup: null })}>
                + Add a Session
              </Button>
            </span>
          </Tooltip>
        </Box>
      )} */}

      <RoundEditorDialog
        tid={tid}
        lockedDivisionId={did}
        lockedRoundGroupId={roundDialog.sessionId || undefined}
        hideRoundGroupField={normalMode}
        round={roundDialog.round}
        isOpen={roundDialog.open}
        onCancel={() => setRoundDialog({ open: false, sessionId: '', round: null })}
        onSave={() => { setRoundDialog({ open: false, sessionId: '', round: null }); loadRounds() }}
      />

      <RoundGroupEditorDialog
        tid={tid}
        lockedDivisionId={did}
        roundgroup={sessionDialog.roundgroup}
        isOpen={sessionDialog.open}
        onCancel={() => setSessionDialog({ open: false, roundgroup: null })}
        onSave={() => { setSessionDialog({ open: false, roundgroup: null }); onRoundGroupsChanged() }}
      />

      {isTimePool && (
        <GameEditorDialog
          tid={tid}
          lockedDivisionId={did}
          // Adding to a round locks the session/round; editing leaves them changeable.
          lockedRoundGroupId={gameDialog.game ? undefined : (gameDialog.roundGroupId || undefined)}
          lockedRoundId={gameDialog.game ? undefined : (gameDialog.roundId || undefined)}
          game={gameDialog.game}
          gamePoolBracketId={gameDialog.poolBracketId}
          hideSessionAndPoolFields={normalMode}
          defaultPoolBracketId={normalMode ? (normalPoolId || undefined) : undefined}
          isOpen={gameDialog.open}
          onCancel={closeGameDialog}
          onSave={async (game) => {
            const pool = gameDialog.poolBracketId ?? normalPoolId
            closeGameDialog()
            if (normalMode && pool) await placeGameTeamsInPool(game, pool)
            onGamesChanged?.()
          }}
          // Editing an existing game: allow viewing its profile and deleting it from the dialog.
          onView={gameDialog.game && onNavigateGame ? () => onNavigateGame(gameDialog.game!.gid) : undefined}
          onDelete={gameDialog.game ? () => { const gid = gameDialog.game!.gid; closeGameDialog(); handleDeleteGame(gid) } : undefined}
        />
      )}

      {isTimePool && (
        <AutoSchedulePoolsDialog
          tid={tid}
          did={did}
          advanced={advanced}
          totalTeams={divisionTeamCount}
          isOpen={autoScheduleOpen}
          onCancel={() => setAutoScheduleOpen(false)}
          onCreated={() => { loadRounds(); loadPoolGroups(); onRoundGroupsChanged(); onPoolsChanged?.(); onGamesChanged?.() }}
        />
      )}

      {normalMode && (
        <SessionStartTimeDialog
          isOpen={allRoundsTimeDialogOpen}
          tid={tid}
          hideSessionWording
          sessionName=""
          defaultStart={allRoundsSorted.find(r => r.scheduled_start_time)?.scheduled_start_time ?? null}
          roundCount={allRoundsSorted.length}
          onCancel={() => setAllRoundsTimeDialogOpen(false)}
          onApply={(startIso, dur) => {
            setAllRoundsTimeDialogOpen(false)
            applyStartTimes(allRoundsSorted, startIso, dur)
          }}
        />
      )}

      {isTimePool && !normalMode && (
        <PoolGroupsEditorDialog
          isOpen={poolGroupsDialogOpen}
          tid={tid}
          did={did}
          canEdit={canEdit}
          onClose={() => setPoolGroupsDialogOpen(false)}
          onChanged={() => { loadPoolGroups(); onPoolsChanged?.(); onGamesChanged?.() }}
        />
      )}

      {isTimePool && !normalMode && (
        <SessionStartTimeDialog
          isOpen={sessionTimeDialog.open}
          tid={tid}
          sessionName={sessionTimeDialog.session?.name ?? ''}
          defaultStart={sessionTimeDialog.session
            ? (roundsBySession.get(sessionTimeDialog.session.roundgroup_id)?.find(r => r.scheduled_start_time)?.scheduled_start_time ?? null)
            : null}
          roundCount={sessionTimeDialog.session ? (roundsBySession.get(sessionTimeDialog.session.roundgroup_id)?.length ?? 0) : 0}
          onCancel={() => setSessionTimeDialog({ open: false, session: null })}
          onApply={(startIso, dur) => {
            const s = sessionTimeDialog.session
            setSessionTimeDialog({ open: false, session: null })
            if (s) applySessionStartTime(s, startIso, dur)
          }}
        />
      )}

      {isTimePool && !normalMode && (
        <PoolBracketEditorDialog
          tid={tid}
          did={did}
          type={poolDialog.bracket?.type ?? 'pool'}
          entityLabel={poolDialog.bracket?.type === 'bracket' ? 'Bracket' : 'Pool'}
          bracket={poolDialog.bracket}
          isOpen={poolDialog.open}
          onCancel={() => setPoolDialog({ open: false, bracket: null })}
          onSave={() => { setPoolDialog({ open: false, bracket: null }); onPoolsChanged?.(); onGamesChanged?.() }}
        />
      )}

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        title={confirmDialog.title}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
      />

      <Snackbar
        open={!!notice}
        autoHideDuration={6000}
        onClose={() => setNotice(null)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
      >
        <Alert severity="warning" onClose={() => setNotice(null)} sx={{ maxWidth: 520 }}>
          {notice}
        </Alert>
      </Snackbar>
    </Box>
  )
}
