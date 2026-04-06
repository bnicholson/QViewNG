import { useState, useEffect, useCallback } from 'react'
import { useNavigate } from 'react-router-dom'
import Accordion from '@mui/material/Accordion'
import AccordionDetails from '@mui/material/AccordionDetails'
import AccordionSummary from '@mui/material/AccordionSummary'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import CheckCircleIcon from '@mui/icons-material/CheckCircle'
import ExpandMoreIcon from '@mui/icons-material/ExpandMore'
import {
  EquipmentSetAPI,
  type GearSetTS,
  type EquipmentDboTS,
  type EquipmentDetail,
  detectGearType,
  GEAR_TYPE_LABELS,
} from '../features/EquipmentSetAPI'
import {
  EquipmentRegistrationAPI,
  type EquipmentRegistrationTS,
} from '../features/EquipmentRegistrationAPI'
import { useAuth } from '../hooks/useAuth'

type ChipColor = 'default' | 'primary' | 'secondary' | 'error' | 'info' | 'success' | 'warning'

const GEAR_TYPE_COLORS: Record<string, ChipColor> = {
  Computer: 'primary',
  JumpPad: 'success',
  InterfaceBox: 'warning',
  Monitor: 'info',
  MicrophoneRecorder: 'secondary',
  Projector: 'error',
  PowerStrip: 'default',
  ExtensionCord: 'default',
}

function getGearProps(detail: EquipmentDetail, dbo: EquipmentDboTS): Array<[string, string]> {
  const props: Array<[string, string]> = [['ID', String(dbo.id)]]

  if ('Computer' in detail) {
    const c = detail.Computer
    if (c.brand) props.push(['Brand', c.brand])
    if (c.operating_system) props.push(['OS', c.operating_system])
    if (c.quizmachine_version) props.push(['QuizMachine', c.quizmachine_version])
    if (c.wifi_capabilities) props.push(['WiFi', c.wifi_capabilities])
  } else if ('JumpPad' in detail) {
    if (detail.JumpPad.color) props.push(['Color', detail.JumpPad.color])
  } else if ('InterfaceBox' in detail) {
    const ib = detail.InterfaceBox
    props.push(['Type', ib.type_])
    if (ib.serial_number) props.push(['S/N', ib.serial_number])
  } else if ('Monitor' in detail) {
    const m = detail.Monitor
    if (m.size) props.push(['Size', `${m.size}"`])
    if (m.brand) props.push(['Brand', m.brand])
  } else if ('MicrophoneRecorder' in detail) {
    if (detail.MicrophoneRecorder.type_) props.push(['Type', detail.MicrophoneRecorder.type_])
  } else if ('Projector' in detail) {
    if (detail.Projector.brand) props.push(['Brand', detail.Projector.brand])
  } else if ('PowerStrip' in detail) {
    const ps = detail.PowerStrip
    if (ps.make) props.push(['Make', ps.make])
    if (ps.model) props.push(['Model', ps.model])
    if (ps.color) props.push(['Color', ps.color])
    props.push(['Plugs', String(ps.num_of_plugs)])
  } else if ('ExtensionCord' in detail) {
    const ec = detail.ExtensionCord
    if (ec.color) props.push(['Color', ec.color])
    if (ec.length) props.push(['Length', ec.length])
  }

  if (dbo.misc_note) props.push(['Note', dbo.misc_note])
  return props
}

interface Props {
  tid: string
}

export const TournamentGearRegistrationPanel = ({ tid }: Props) => {
  const { session } = useAuth()
  const navigate = useNavigate()
  const userId = session?.userId

  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [gearSets, setGearSets] = useState<GearSetTS[]>([])
  const [equipmentBySet, setEquipmentBySet] = useState<Record<number, EquipmentDboTS[]>>({})
  const [registrations, setRegistrations] = useState<EquipmentRegistrationTS[]>([])
  const [detailsByEquipId, setDetailsByEquipId] = useState<Record<number, EquipmentDetail>>({})
  // Set of equipment DBO IDs currently being acted on
  const [busy, setBusy] = useState<Set<number>>(new Set())

  const load = useCallback(async () => {
    if (!userId) { setLoading(false); return }
    setLoading(true)
    setError(null)
    try {
      const sets = await EquipmentSetAPI.getByOwner(userId)
      setGearSets(sets)

      if (sets.length === 0) {
        setEquipmentBySet({})
        setDetailsByEquipId({})
        setRegistrations([])
        return
      }

      const equipResults = await Promise.all(sets.map(s => EquipmentSetAPI.getEquipmentInSet(s.id)))
      const bySet: Record<number, EquipmentDboTS[]> = {}
      sets.forEach((s, i) => { bySet[s.id] = equipResults[i] })
      setEquipmentBySet(bySet)

      const allItems = equipResults.flat()
      const allEquipIds = allItems.map(e => e.id)

      const [regs, detailResults] = await Promise.all([
        EquipmentRegistrationAPI.getForEquipmentInTournament(allEquipIds, tid),
        Promise.all(allItems.map(e => EquipmentSetAPI.getEquipmentDetail(e.id).catch(() => null))),
      ])
      setRegistrations(regs)

      const detailMap: Record<number, EquipmentDetail> = {}
      allItems.forEach((e, i) => { if (detailResults[i]) detailMap[e.id] = detailResults[i]! })
      setDetailsByEquipId(detailMap)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }, [userId, tid])

  useEffect(() => { load() }, [load])

  // ── Derived lookup maps (recomputed on render, stable for the lifetime of each render) ──

  const regByEquipId = new Map(registrations.map(r => [r.equipmentid, r]))

  // ── Busy-state helpers ──────────────────────────────────────────────────────

  const addBusy = (ids: number[]) =>
    setBusy(prev => { const n = new Set(prev); ids.forEach(i => n.add(i)); return n })

  const removeBusy = (ids: number[]) =>
    setBusy(prev => { const n = new Set(prev); ids.forEach(i => n.delete(i)); return n })

  // ── Registration actions ────────────────────────────────────────────────────

  const doRegister = async (items: EquipmentDboTS[]) => {
    const unregistered = items.filter(e => !regByEquipId.has(e.id))
    if (unregistered.length === 0) return
    const ids = unregistered.map(e => e.id)
    addBusy(ids)
    try {
      const created = await Promise.all(
        unregistered.map(e =>
          EquipmentRegistrationAPI.create({
            equipmentid: e.id,
            tournamentid: tid,
            status: 'Not Yet Received from Owner',
          }),
        ),
      )
      setRegistrations(prev => [...prev, ...created])
    } catch (e: any) {
      setError(e.message)
    } finally {
      removeBusy(ids)
    }
  }

  const doUnregister = async (items: EquipmentDboTS[]) => {
    const toDelete = items
      .map(e => regByEquipId.get(e.id))
      .filter((r): r is EquipmentRegistrationTS => r !== undefined)
    if (toDelete.length === 0) return
    const equipIds = toDelete.map(r => r.equipmentid)
    addBusy(equipIds)
    try {
      await Promise.all(toDelete.map(r => EquipmentRegistrationAPI.delete(r.id)))
      const gone = new Set(toDelete.map(r => r.id))
      setRegistrations(prev => prev.filter(r => !gone.has(r.id)))
    } catch (e: any) {
      setError(e.message)
    } finally {
      removeBusy(equipIds)
    }
  }

  const handleRegisterItem = (item: EquipmentDboTS) => doRegister([item])

  const handleUnregisterItem = (item: EquipmentDboTS) => doUnregister([item])

  const handleRegisterSet = (setId: number) => doRegister(equipmentBySet[setId] ?? [])
  const handleUnregisterSet = (setId: number) => doUnregister(equipmentBySet[setId] ?? [])

  // ── Not logged in ───────────────────────────────────────────────────────────

  if (!userId) {
    return (
      <Alert severity="info">
        Please <strong>sign in</strong> or <strong>create an account</strong> to register gear for
        this tournament.
      </Alert>
    )
  }

  // ── Loading ─────────────────────────────────────────────────────────────────

  if (loading) {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 3 }}>
        <CircularProgress size={20} />
        <Typography color="text.secondary">Loading your gear…</Typography>
      </Box>
    )
  }

  // ── Hard error (before any data loaded) ────────────────────────────────────

  if (error && gearSets.length === 0) {
    return (
      <Alert severity="error" action={
        <Button color="inherit" size="small" onClick={() => { setError(null); load() }}>Retry</Button>
      }>
        {error}
      </Alert>
    )
  }

  // ── No gear ─────────────────────────────────────────────────────────────────

  const allItems = Object.values(equipmentBySet).flat()
  if (gearSets.length === 0 || allItems.length === 0) {
    return (
      <Stack spacing={2} alignItems="flex-start">
        <Alert severity="info" sx={{ width: '100%' }}>
          You don't have any gear in your profile yet. Add gear to your profile before
          registering it for this tournament.
        </Alert>
        <Button variant="outlined" onClick={() => navigate(`/user/${userId}/my-gear`)}>
          Go to My Gear →
        </Button>
      </Stack>
    )
  }

  // ── Registration UI ─────────────────────────────────────────────────────────

  const totalRegistered = registrations.length

  return (
    <Box>
      {/* ── Page header ── */}
      <Box sx={{ display: 'flex', alignItems: 'baseline', gap: 2, mb: 2 }}>
        <Typography variant="h6">My Registered Gear</Typography>
        <Typography variant="body2" color="text.secondary">
          {totalRegistered} of {allItems.length} item{allItems.length !== 1 ? 's' : ''} registered
          for this tournament
        </Typography>
        <Button
          size="small"
          variant="outlined"
          sx={{ ml: 'auto' }}
          onClick={() => navigate(`/user/${userId}/my-gear`)}
        >
          Manage My Gear →
        </Button>
      </Box>

      {/* ── Soft error banner (shown while data is present) ── */}
      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>
          {error}
        </Alert>
      )}

      {/* ── GearSet accordions ── */}
      <Stack spacing={1}>
        {gearSets.map(set => {
          const items = equipmentBySet[set.id] ?? []
          if (items.length === 0) return null

          const registeredCount = items.filter(e => regByEquipId.has(e.id)).length
          const allRegistered = registeredCount === items.length
          const noneRegistered = registeredCount === 0
          const setIsBusy = items.some(e => busy.has(e.id))

          return (
            <Accordion
              key={set.id}
              defaultExpanded
              disableGutters
              elevation={1}
              sx={{ '&:before': { display: 'none' }, borderRadius: '6px !important', overflow: 'hidden' }}
            >
              {/* ── Set header ── */}
              <AccordionSummary expandIcon={<ExpandMoreIcon />} sx={{ minHeight: 52 }}>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, flexGrow: 1, pr: 1, flexWrap: 'wrap' }}>
                  <Typography fontWeight={600}>{set.name}</Typography>

                  <Chip
                    label={`${registeredCount} / ${items.length} registered`}
                    size="small"
                    color={allRegistered ? 'success' : noneRegistered ? 'default' : 'warning'}
                    variant={allRegistered ? 'filled' : 'outlined'}
                  />

                  {setIsBusy && <CircularProgress size={14} sx={{ ml: 0.5 }} />}

                  {/* ── Set-level action buttons ── */}
                  <Box sx={{ ml: 'auto', display: 'flex', gap: 1 }}>
                    {!allRegistered && (
                      <Button
                        component="div"
                        size="small"
                        variant="contained"
                        disabled={setIsBusy}
                        onClick={e => { e.stopPropagation(); handleRegisterSet(set.id) }}
                      >
                        {noneRegistered ? 'Register Set' : 'Register Remaining'}
                      </Button>
                    )}
                    {!noneRegistered && (
                      <Button
                        component="div"
                        size="small"
                        variant="outlined"
                        color="error"
                        disabled={setIsBusy}
                        onClick={e => { e.stopPropagation(); handleUnregisterSet(set.id) }}
                      >
                        Unregister Set
                      </Button>
                    )}
                  </Box>
                </Box>
              </AccordionSummary>

              {/* ── Item list ── */}
              <AccordionDetails sx={{ px: 2, pt: 0, pb: 1 }}>
                {set.description && (
                  <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5, mb: 1.5, textAlign: "left" }}>
                    Description: {set.description}
                  </Typography>
                )}

                <Stack divider={<Divider />}>
                  {items.map(item => {
                    const gearType = detectGearType(item)
                    const isRegistered = regByEquipId.has(item.id)
                    const itemBusy = busy.has(item.id)

                    const detail = detailsByEquipId[item.id]
                    const props = detail ? getGearProps(detail, item) : [['ID', String(item.id)]]

                    return (
                      <Box
                        key={item.id}
                        sx={{
                          display: 'flex',
                          alignItems: 'flex-start',
                          gap: 1.5,
                          py: 1,
                          transition: 'opacity 0.15s',
                          opacity: itemBusy ? 0.5 : 1,
                        }}
                      >
                        {/* Type chip */}
                        {gearType && (
                          <Chip
                            label={GEAR_TYPE_LABELS[gearType]}
                            size="small"
                            color={GEAR_TYPE_COLORS[gearType] ?? 'default'}
                            sx={{ flexShrink: 0, mt: 0.25 }}
                          />
                        )}

                        {/* Properties */}
                        <Typography variant="body2" color="text.secondary" sx={{ flex: 1, textAlign: "left" }}>
                          {props.map(([label, value]) => `${label}: ${value}`).join(',  ')}
                        </Typography>

                        {/* Register / Registered state */}
                        {isRegistered ? (
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.75, flexShrink: 0, ml: 'auto' }}>
                            <CheckCircleIcon fontSize="small" color="success" />
                            <Typography variant="body2" color="success.main" fontWeight={500}>
                              Registered
                            </Typography>
                            <Button
                              size="small"
                              color="error"
                              variant="text"
                              disabled={itemBusy}
                              onClick={() => handleUnregisterItem(item)}
                              sx={{ minWidth: 0, px: 1 }}
                            >
                              {itemBusy ? <CircularProgress size={12} /> : 'Undo'}
                            </Button>
                          </Box>
                        ) : (
                          <Button
                            size="small"
                            variant="outlined"
                            disabled={itemBusy}
                            onClick={() => handleRegisterItem(item)}
                            sx={{ flexShrink: 0 }}
                          >
                            {itemBusy ? <CircularProgress size={12} /> : 'Register'}
                          </Button>
                        )}
                      </Box>
                    )
                  })}
                </Stack>
              </AccordionDetails>
            </Accordion>
          )
        })}
      </Stack>
    </Box>
  )
}
