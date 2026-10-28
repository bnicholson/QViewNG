import { Fragment, useState, useEffect, useCallback, useMemo, type Dispatch, type SetStateAction } from 'react'
import Box from '@mui/material/Box'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import Button from '@mui/material/Button'
import Typography from '@mui/material/Typography'
import Tooltip from '@mui/material/Tooltip'
import Chip from '@mui/material/Chip'
import Checkbox from '@mui/material/Checkbox'
import FormControlLabel from '@mui/material/FormControlLabel'
import Alert from '@mui/material/Alert'
import Snackbar from '@mui/material/Snackbar'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import ScheduleIcon from '@mui/icons-material/Schedule'
import VisibilityIcon from '@mui/icons-material/Visibility'
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff'
import WarningAmberIcon from '@mui/icons-material/WarningAmber'
import dayjs from 'dayjs'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { type GameRowTS, type GameTS } from '../features/GameAPI'
import { GameAPI } from '../features/GameAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { PoolBracketGroupAPI } from '../features/PoolBracketGroupAPI'
import { TeamAPI } from '../features/TeamAPI'
import { AutoSchedulePoolsDialog } from './AutoSchedulePoolsDialog'
import { RoundEditorDialog } from './RoundEditorDialog'
import { GameEditorDialog } from './GameEditorDialog'
import { RoundStartTimesDialog } from './RoundStartTimesDialog'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { useAuth } from '../hooks/useAuth'
import { PERMISSION } from '../auth/permissions'
import SetQuestionTypesButton from './SetQuestionTypesButton'

const PAGE = 0
const SIZE = 500

// Rounds use orange (not green) so they don't clash with the teal/green font of the linked games
// shown inside them. Breaks between rounds get a muted slate so they read as "no quizzing" time.
const ROUND_CARD = { bg: '#fff7ed', border: '#fed7aa', label: '#c2410c' }     // orange
const BREAK_CARD = { bg: '#f1f5f9', border: '#cbd5e1', label: '#475569' }   // slate
const GAME_CHIP_BG = '#eef2ff'                                                    // indigo (the old Session colour)

/** One game as shown under a round, with the pool it belongs to. */
export interface RoundGameEntry {
  g: GameRowTS
  poolName: string
  /** The game's pool bracket id (GameRowTS doesn't carry it) — needed to edit the game. */
  poolBracketId: string
}

/**
 * What sits between a round and the one before it (in start order), given the tournament's round
 * duration: free time (a break) or too little time (an overlap).
 */
type RoundGap =
  | { kind: 'break'; minutes: number; from: string; to: string }
  | { kind: 'overlap'; minutes: number; prevNames: string[]; name: string }

/** "Oct 27, 2026 · 7:00 AM – 7:30 AM": the round's start and its end one round duration later. The
 *  end gets its own date only when the round runs past midnight. */
function formatTimeRange(iso: string | null, durationMinutes: number): string {
  if (!iso) return 'Unscheduled'
  const start = dayjs(iso)
  const end = start.add(durationMinutes, 'minute')
  const endFormat = end.isSame(start, 'day') ? 'h:mm A' : 'MMM D, YYYY · h:mm A'
  return `${start.format('MMM D, YYYY · h:mm A')} – ${end.format(endFormat)}`
}

/** "45 min", "1 hr", "1 hr 30 min". */
function formatMinutes(total: number): string {
  const h = Math.floor(total / 60)
  const m = total % 60
  if (h === 0) return `${m} min`
  return m === 0 ? `${h} hr` : `${h} hr ${m} min`
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
  /** The division whose rounds are managed. */
  did: string
  canEdit: boolean
  /**
   * The tournament's default round duration, in minutes. Each round is assumed to last this long,
   * which is how breaks (and overlaps) between rounds are found from their start times alone.
   */
  roundDurationMinutes: number
  /** Games grouped by round id (with their pool name), shown under each round. */
  gamesByRound?: Map<string, RoundGameEntry[]>
  /** Navigate to a game's profile when its "View Game" is clicked in the game editor. */
  onNavigateGame?: (gid: string) => void
  /** Called after a game is added/edited/deleted so the parent can reload games. */
  onGamesChanged?: () => void
  /** The division's pools/brackets — new games are attached to one of them behind the scenes. */
  poolBrackets?: PoolBracketTS[]
  /** Called after a pool is created behind the scenes so the parent can reload pools. */
  onPoolsChanged?: () => void
}

/**
 * A division's schedule as the user builds it: Rounds (the event's time) and the Games played in
 * each, as one flat chronological list. Pools are kept out of sight — a game still needs one, so it
 * is resolved (or created) in the background and never named here. Breaks and overlaps between
 * rounds aren't stored anywhere; they're worked out from the rounds' start times and the
 * tournament's round duration.
 */
export default function RoundsManager({ tid, did, canEdit, roundDurationMinutes, gamesByRound, onNavigateGame, onGamesChanged, poolBrackets, onPoolsChanged }: Props) {
  const { accessToken, session } = useAuth()
  // Setting every round's question types at once also takes its own permission on top of editing the schedule.
  const canUploadQuestionTypes = canEdit && (session?.can(PERMISSION.ROUND_UPLOAD_QUESTION_TYPES) ?? false)
  const [rounds, setRounds] = useState<RoundTS[]>([])
  const [notice, setNotice] = useState<string | null>(null)

  // Round create/edit dialog: `round` = the round being edited (null = create).
  const [roundDialog, setRoundDialog] = useState<{ open: boolean; round: RoundTS | null }>({ open: false, round: null })
  // Game add/edit dialog: `game` = the game being edited (null = add to `roundId`).
  const [gameDialog, setGameDialog] = useState<{ open: boolean; roundId: string; game: GameRowTS | null; poolBracketId?: string }>({ open: false, roundId: '', game: null })
  const closeGameDialog = () => setGameDialog({ open: false, roundId: '', game: null })
  // Re-times every round in the division from a chosen start.
  const [startTimesDialogOpen, setStartTimesDialogOpen] = useState(false)
  // The pool a hand-made game is attached to, resolved (or created) just before the game dialog
  // opens. Passed to the dialog, which hides the Pool/Bracket field entirely.
  const [gamePoolId, setGamePoolId] = useState('')
  // Confirmation prompt for the cascading round delete.
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState)
  // Games are controlled directly by room/team visibility.
  const [hiddenRoomIds, setHiddenRoomIds] = useState<Set<string>>(new Set())
  const [hiddenTeamIds, setHiddenTeamIds] = useState<Set<string>>(new Set())
  // When off, rounds that end up with no visible games (from filtering, or genuinely empty) are
  // hidden entirely. On (default) keeps them visible.
  const [showEmpty, setShowEmpty] = useState(true)
  // Toggle an id in one of the visual-filter sets (clicking a Room/Team chip hides it).
  const makeToggle = (setter: Dispatch<SetStateAction<Set<string>>>) => (id: string) =>
    setter(prev => {
      const next = new Set(prev)
      if (next.has(id)) next.delete(id); else next.add(id)
      return next
    })
  const toggleRoomFilter = makeToggle(setHiddenRoomIds)
  const toggleTeamFilter = makeToggle(setHiddenTeamIds)
  // A game/round filter is active whenever any room/team is hidden (i.e. "Reset / Show All" is
  // enabled). Creating while filtered is blocked so the user can't unknowingly duplicate a hidden game.
  const filtersActive = hiddenRoomIds.size > 0 || hiddenTeamIds.size > 0
  const createBlockedTooltip = filtersActive
    ? 'Filters are hiding some games. Click "Reset / Show All" first so you can see every game before creating.'
    : ''

  const [autoScheduleOpen, setAutoScheduleOpen] = useState(false)
  const [divisionTeamCount, setDivisionTeamCount] = useState(0)
  // The division's team count feeds the Auto-Schedule dialog's round math.
  useEffect(() => {
    TeamAPI.getByDivision(did, PAGE, SIZE).then(ts => setDivisionTeamCount(ts.length)).catch(() => {})
  }, [did])

  const loadRounds = useCallback(() => {
    RoundAPI.getByDivision(did, PAGE, SIZE)
      .then(setRounds)
      .catch(() => setNotice('Failed to load rounds.'))
  }, [did])

  useEffect(() => { loadRounds() }, [loadRounds])

  // Every round in the division as one flat, chronological list.
  const allRoundsSorted = useMemo(() => [...rounds].sort(byStartThenName), [rounds])

  // The gap before each round, measured from the round before it: positive is a break, negative an
  // overlap. Rounds sharing a start time run in parallel and get nothing between them; unscheduled
  // rounds aren't compared. Keyed by the later round's id.
  const gapBefore = useMemo(() => {
    const m = new Map<string, RoundGap>()
    for (let i = 1; i < allRoundsSorted.length; i++) {
      const prev = allRoundsSorted[i - 1]
      const cur = allRoundsSorted[i]
      if (!prev.scheduled_start_time || !cur.scheduled_start_time) continue
      if (prev.scheduled_start_time === cur.scheduled_start_time) continue
      const prevEnd = dayjs(prev.scheduled_start_time).add(roundDurationMinutes, 'minute')
      const minutes = dayjs(cur.scheduled_start_time).diff(prevEnd, 'minute')
      if (minutes > 0) {
        m.set(cur.roundid, { kind: 'break', minutes, from: prevEnd.toISOString(), to: cur.scheduled_start_time })
      } else if (minutes < 0) {
        // Every round running in parallel with `prev` overlaps too, so name them all.
        const prevNames = allRoundsSorted
          .filter(r => r.scheduled_start_time === prev.scheduled_start_time)
          .map(r => r.name)
        m.set(cur.roundid, { kind: 'overlap', minutes: -minutes, prevNames, name: cur.name })
      }
    }
    return m
  }, [allRoundsSorted, roundDurationMinutes])

  // Each round's immediate predecessor in the full list. A gap is only drawn when that predecessor
  // is also on screen, so a filtered-out round never shows up as a fake break.
  const prevRoundId = useMemo(() => {
    const m = new Map<string, string>()
    allRoundsSorted.forEach((r, i) => { if (i > 0) m.set(r.roundid, allRoundsSorted[i - 1].roundid) })
    return m
  }, [allRoundsSorted])

  // Deleting a round cascades on the backend to its games (and nothing else — pools and their teams
  // stay), so refresh rounds and games afterward.
  const handleDelete = async (round: RoundTS) => {
    try {
      await RoundAPI.delete(round.roundid, accessToken)
      loadRounds()
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete round.')
    }
  }

  // Open a warning before the (irreversible, cascading) round delete.
  const confirmDeleteRound = (round: RoundTS) => {
    const gameCount = gamesByRound?.get(round.roundid)?.length ?? 0
    const gamesPart = gameCount > 0
      ? `Deleting this round also deletes the ${gameCount} game${gameCount === 1 ? '' : 's'} in it.`
      : 'This round has no games.'
    setConfirmDialog({
      isOpen: true,
      title: `Delete round "${round.name}"?`,
      message: `${gamesPart} Pools and the Teams placed in them aren't affected. This can't be undone.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleDelete(round) },
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

  const handleDeleteGame = async (gid: string) => {
    try {
      await GameAPI.delete(gid, accessToken)
      onGamesChanged?.()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to delete game.')
    }
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
    onPoolsChanged?.()
    return pool.pool_bracket_id
  }

  /** Open the game dialog for a round, resolving (or creating) its pool first. */
  const openGameDialog = async (round: RoundTS, game: GameRowTS | null, poolBracketId?: string) => {
    try {
      const pool = poolBracketId ?? await resolveDefaultPool(round)
      setGamePoolId(pool)
      setGameDialog({ open: true, roundId: round.roundid, game, poolBracketId: pool })
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Failed to prepare the game.')
    }
  }

  /**
   * A game's teams also join the pool behind it, since the user has no way to place them there
   * themselves and pool standings read from those placements. Already-placed teams make the call
   * fail harmlessly, so failures are ignored.
   */
  const placeGameTeamsInPool = async (game: GameTS, poolBracketId: string) => {
    const teamIds = [game.leftteamid, game.centerteamid, game.rightteamid].filter((t): t is string => !!t)
    await Promise.all(teamIds.map(t =>
      TeamAPI.addToPoolBracket(poolBracketId, t, accessToken).catch(() => { /* already placed */ })
    ))
  }

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

  // A game is hidden if its room is toggled off, or — for teams — if EVERY team in the game is
  // toggled off. (A matchup stays visible as long as at least one of its teams is visible, so
  // showing a single team surfaces all of that team's games.)
  const gameHiddenByFilter = (e: RoundGameEntry): boolean => {
    if (hiddenRoomIds.has(e.g.roomid)) return true
    const teamIds = [e.g.leftteamid, e.g.rightteamid, ...(e.g.centerteamid != null ? [e.g.centerteamid] : [])]
    return teamIds.length > 0 && teamIds.every(id => hiddenTeamIds.has(id))
  }

  // Games for a round (visual filters applied), sorted by room name.
  const gamesForRound = (roundId: string): RoundGameEntry[] =>
    [...(gamesByRound?.get(roundId) ?? [])]
      .filter(e => !gameHiddenByFilter(e))
      .sort((a, b) => a.g.room_name.localeCompare(b.g.room_name, undefined, { numeric: true }))

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

  // A break (free time between rounds) as its own slate card, or an overlap as a small warning line.
  const renderGap = (gap: RoundGap) => gap.kind === 'break' ? (
    <Box
      sx={{
        display: 'flex', alignItems: 'baseline', gap: 1, flexWrap: 'wrap',
        bgcolor: BREAK_CARD.bg,
        border: `1px dashed ${BREAK_CARD.border}`,
        borderRadius: 1.5, px: 1, py: 0.75,
      }}
    >
      <Typography variant="body2" sx={{ fontWeight: 600, color: BREAK_CARD.label }}>Break: {formatMinutes(gap.minutes)}</Typography>
      <Typography variant="caption" color="text.secondary">
        {dayjs(gap.from).format('h:mm A')} – {dayjs(gap.to).format('h:mm A')}
      </Typography>
    </Box>
  ) : (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, pl: 1 }}>
      <WarningAmberIcon sx={{ fontSize: 16, color: 'warning.main' }} />
      <Typography variant="caption" sx={{ color: 'warning.dark', fontWeight: 600 }}>
        Overlaps: Round “{gap.name}” starts {formatMinutes(gap.minutes)} before {gap.prevNames.length === 1 ? 'Round' : 'Rounds'} {gap.prevNames.map(n => `“${n}”`).join(', ')} {gap.prevNames.length === 1 ? 'ends' : 'end'}.
      </Typography>
    </Box>
  )

  // One round card with its games.
  const renderRoundCard = (round: RoundTS, roundGames: RoundGameEntry[]) => {
    // A round with any game (ignoring the visibility filters) gates the "No games in this round."
    // message (only shown when the round is truly empty).
    const roundHasAnyGames = (gamesByRound?.get(round.roundid)?.length ?? 0) > 0
    return (
      <Box
        sx={{
          display: 'flex', flexDirection: 'column', gap: 0.5,
          bgcolor: ROUND_CARD.bg,
          border: `1px solid ${ROUND_CARD.border}`,
          borderRadius: 1.5, px: 1, py: 0.75,
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Box sx={{ flex: 1, minWidth: 0, display: 'flex', alignItems: 'baseline', gap: 1, flexWrap: 'wrap' }}>
            <Typography variant="body2" sx={{ fontWeight: 600, color: ROUND_CARD.label }}>Round: {round.name}</Typography>
            <Typography variant="caption" color="text.secondary">{formatTimeRange(round.scheduled_start_time, roundDurationMinutes)}</Typography>
          </Box>
          {canEdit && (
            <>
              <Tooltip title={createBlockedTooltip}>
                <span>
                  <Button size="small" startIcon={<AddIcon />} disabled={filtersActive} onClick={() => openGameDialog(round, null)}>
                    Add Game
                  </Button>
                </span>
              </Tooltip>
              <Button size="small" startIcon={<EditIcon />} onClick={() => setRoundDialog({ open: true, round })}>
                Edit Round
              </Button>
              <Button size="small" color="error" startIcon={<DeleteIcon />} onClick={() => confirmDeleteRound(round)}>
                Delete Round
              </Button>
            </>
          )}
        </Box>

        {/* The round's games. A round whose games are all hidden by the filter renders nothing;
            "No games in this round." only shows when the round is truly empty. */}
        {!roundHasAnyGames ? (
          <Typography variant="caption" color="text.secondary" sx={{ pl: 0.5 }}>No games in this round.</Typography>
        ) : roundGames.length === 0 ? null : (
          // Games as wrapping chips so they fill the row's whitespace and cut down on vertical scrolling.
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, pl: 0.5 }}>
            {roundGames.map(({ g, poolBracketId }) => (
              // No icons on the chip: clicking it opens the game editor, where the user can edit,
              // delete, or view the game's profile.
              <Chip
                key={g.gid}
                variant="outlined"
                onClick={() => openGameDialog(round, g, poolBracketId)}
                sx={{
                  height: 'auto',
                  borderRadius: '8px',
                  bgcolor: GAME_CHIP_BG,
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
                  </Box>
                }
              />
            ))}
          </Box>
        )}
      </Box>
    )
  }

  return (
    <Box sx={{ textAlign: 'left' }}>
      <Alert severity="info" sx={{ mb: 1.5, textAlign: 'left' }}>
        Build your schedule here: create the Rounds that make up your event's time, then add the Games played in each Round. Or use "Create Schedule" to generate a whole round-robin at once. Each Round is taken to last the Tournament's round duration ({formatMinutes(roundDurationMinutes)}), so any longer gap between Rounds shows as a break.
      </Alert>

      {/* Filters card — client-side show/hide of the games below by Room or Team. Hidden until the
          division actually has games (which implies it has rounds too). */}
      {hasAnyGames && (
        <Card variant="outlined" sx={{ mb: 1.5 }}>
          <CardContent sx={{ pb: '12px !important' }}>
            <Typography variant="subtitle2" sx={{ fontWeight: 700, mb: 1 }}>Filter:</Typography>
            {/* Reset — clears the Room/Team hide filters so every game shows again. */}
            <Box sx={{ mb: 1.5 }}>
              <Button
                size="small"
                variant="contained"
                color="primary"
                disableElevation
                disabled={hiddenRoomIds.size === 0 && hiddenTeamIds.size === 0 && showEmpty}
                onClick={() => { setHiddenRoomIds(new Set()); setHiddenTeamIds(new Set()); setShowEmpty(true) }}
              >
                Reset / Show All
              </Button>
            </Box>

            {/* Rooms filter — click a room chip to show/hide its games below. */}
            {roomsInGames.length > 0 && (
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
            {teamsInGames.length > 0 && (
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

            {/* Bottom row: whether to keep empty (or filtered-to-empty) rounds visible. */}
            <FormControlLabel
              sx={{ mt: 0.5 }}
              control={<Checkbox size="small" checked={showEmpty} onChange={(e) => setShowEmpty(e.target.checked)} />}
              label={<Typography variant="body2">Show Rounds When Empty</Typography>}
            />
          </CardContent>
        </Card>
      )}

      {/* Auto-Schedule plus the schedule-wide round actions — directly below the filters. */}
      {canEdit && (
        <Box sx={{ mb: 1.5, display: 'flex', alignItems: 'center', gap: 1, flexWrap: 'wrap' }}>
          <Button variant="contained" disableElevation onClick={() => setAutoScheduleOpen(true)}>Create Schedule</Button>
          <Tooltip title={createBlockedTooltip}>
            <span>
              <Button size="small" startIcon={<AddIcon />} disabled={filtersActive} onClick={() => setRoundDialog({ open: true, round: null })}>
                Add Round
              </Button>
            </span>
          </Tooltip>
          <Tooltip title={rounds.length === 0 ? 'Add a round first to set a start time.' : ''}>
            <span>
              <Button size="small" startIcon={<ScheduleIcon />} disabled={rounds.length === 0} onClick={() => setStartTimesDialogOpen(true)}>
                Edit Start Times
              </Button>
            </span>
          </Tooltip>
          {canUploadQuestionTypes && <SetQuestionTypesButton did={did} onSaved={loadRounds} />}
        </Box>
      )}

      {/* Every round in the division as one flat, chronological list, with any break or overlap
          drawn between a round and the one before it. */}
      {(() => {
        const displayRounds = showEmpty ? allRoundsSorted : allRoundsSorted.filter(r => gamesForRound(r.roundid).length > 0)
        if (allRoundsSorted.length === 0) {
          return <Typography variant="body2" color="text.secondary">No rounds yet. Use "Create Schedule" to build a whole schedule, or "Add Round" to start one round at a time.</Typography>
        }
        if (displayRounds.length === 0) {
          return <Typography variant="body2" color="text.secondary">No rounds have a visible game. Clear the filters, or tick "Show Rounds When Empty".</Typography>
        }
        return (
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            {displayRounds.map((round, i) => {
              const prevShown = i > 0 && displayRounds[i - 1].roundid === prevRoundId.get(round.roundid)
              const gap = prevShown ? gapBefore.get(round.roundid) : undefined
              return (
                <Fragment key={round.roundid}>
                  {gap && renderGap(gap)}
                  {renderRoundCard(round, gamesForRound(round.roundid))}
                </Fragment>
              )
            })}
          </Box>
        )
      })()}

      <RoundEditorDialog
        tid={tid}
        lockedDivisionId={did}
        round={roundDialog.round}
        isOpen={roundDialog.open}
        onCancel={() => setRoundDialog({ open: false, round: null })}
        onSave={() => { setRoundDialog({ open: false, round: null }); loadRounds() }}
      />

      <GameEditorDialog
        tid={tid}
        lockedDivisionId={did}
        // Adding to a round locks the round; editing leaves it changeable.
        lockedRoundId={gameDialog.game ? undefined : (gameDialog.roundId || undefined)}
        game={gameDialog.game}
        gamePoolBracketId={gameDialog.poolBracketId}
        hidePoolField
        defaultPoolBracketId={gamePoolId || undefined}
        isOpen={gameDialog.open}
        onCancel={closeGameDialog}
        onSave={async (game) => {
          const pool = gameDialog.poolBracketId ?? gamePoolId
          closeGameDialog()
          if (pool) await placeGameTeamsInPool(game, pool)
          onGamesChanged?.()
        }}
        // Editing an existing game: allow viewing its profile and deleting it from the dialog.
        onView={gameDialog.game && onNavigateGame ? () => onNavigateGame(gameDialog.game!.gid) : undefined}
        onDelete={gameDialog.game ? () => { const gid = gameDialog.game!.gid; closeGameDialog(); handleDeleteGame(gid) } : undefined}
      />

      <AutoSchedulePoolsDialog
        tid={tid}
        did={did}
        totalTeams={divisionTeamCount}
        isOpen={autoScheduleOpen}
        onCancel={() => setAutoScheduleOpen(false)}
        onCreated={() => { loadRounds(); onPoolsChanged?.(); onGamesChanged?.() }}
      />

      <RoundStartTimesDialog
        isOpen={startTimesDialogOpen}
        tid={tid}
        rounds={allRoundsSorted}
        onCancel={() => setStartTimesDialogOpen(false)}
        onApply={(selected, startIso, dur) => {
          setStartTimesDialogOpen(false)
          applyStartTimes(selected, startIso, dur)
        }}
      />

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
