import { useState, useEffect } from 'react'
import Box from '@mui/material/Box'
import CircularProgress from '@mui/material/CircularProgress'
import MenuItem from '@mui/material/MenuItem'
import Paper from '@mui/material/Paper'
import Select from '@mui/material/Select'
import Tab from '@mui/material/Tab'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import Tabs from '@mui/material/Tabs'
import Typography from '@mui/material/Typography'
import type { EquipmentRegistrationTS } from '../features/EquipmentRegistrationAPI'
import { EquipmentRegistrationAPI } from '../features/EquipmentRegistrationAPI'
import type {
  EquipmentDetail,
  ComputerTS,
  MonitorTS,
  ProjectorTS,
} from '../features/EquipmentSetAPI'
import {
  EquipmentSetAPI,
  extractDetailType,
  getGearDescription,
  GEAR_TYPE_LABELS,
} from '../features/EquipmentSetAPI'
import type { RoomTS } from '../features/RoomAPI'
import { RoomAPI } from '../features/RoomAPI'

// ── Status options (mirrors backend EquipmentRegistrationStatus enum) ─────────

const STATUSES = [
  'Not Yet Received from Owner',
  'Received from Owner',
  'Prepared for Assignment',
  'Assigned to Room',
  'On Standby',
  'Deployed to Room',
  'Returned from Room',
  'Needs Repair',
  'Returned to Owner',
]

// ── Tab definitions ───────────────────────────────────────────────────────────

const TAB_LABELS = [
  'All',
  'Computer',
  'Jump Pad',
  'Interface Box',
  'Monitor',
  'Mic / Recorder',
  'Projector',
  'Power Strip',
  'Extension Cord',
]

const TAB_TYPES = [
  null,
  'Computer',
  'JumpPad',
  'InterfaceBox',
  'Monitor',
  'MicrophoneRecorder',
  'Projector',
  'PowerStrip',
  'ExtensionCord',
]

// ── Helpers ───────────────────────────────────────────────────────────────────

interface GearRow {
  reg: EquipmentRegistrationTS
  detail: EquipmentDetail
}

function formatPorts(d: ComputerTS | MonitorTS | ProjectorTS): string {
  const ports: string[] = []
  if (d.has_vga_out_port) ports.push('VGA')
  if (d.has_dvi_out_port) ports.push('DVI')
  if (d.has_hdmi_out_port) ports.push('HDMI')
  if (d.has_display_port_out) ports.push('DP')
  return ports.join(', ') || '—'
}

// ── Shared Status / Room cells ────────────────────────────────────────────────

function StatusCell({
  row,
  saving,
  onChange,
}: {
  row: GearRow
  saving: boolean
  onChange: (regId: number, status: string) => void
}) {
  return (
    <Select
      size="small"
      value={row.reg.status}
      disabled={saving}
      onChange={e => onChange(row.reg.id, e.target.value)}
      sx={{ minWidth: 230 }}
    >
      {STATUSES.map(s => (
        <MenuItem key={s} value={s}>{s}</MenuItem>
      ))}
    </Select>
  )
}

function RoomCell({
  row,
  rooms,
  saving,
  onChange,
}: {
  row: GearRow
  rooms: RoomTS[]
  saving: boolean
  onChange: (regId: number, roomId: string) => void
}) {
  return (
    <Select
      size="small"
      displayEmpty
      value={row.reg.roomid ?? ''}
      disabled={saving}
      onChange={e => { if (e.target.value) onChange(row.reg.id, e.target.value) }}
      sx={{ minWidth: 140 }}
    >
      <MenuItem value="" disabled>Not Assigned</MenuItem>
      {rooms.map(r => (
        <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>
      ))}
    </Select>
  )
}

// ── Generic gear table ────────────────────────────────────────────────────────

type ColDef = { header: string; cell: (row: GearRow) => React.ReactNode }

function GearTable({
  rows,
  rooms,
  savingIds,
  extraCols,
  onStatusChange,
  onRoomChange,
}: {
  rows: GearRow[]
  rooms: RoomTS[]
  savingIds: Set<number>
  extraCols: ColDef[]
  onStatusChange: (regId: number, status: string) => void
  onRoomChange: (regId: number, roomId: string) => void
}) {
  return (
    <TableContainer component={Paper} variant="outlined" sx={{ mt: 2 }}>
      <Table size="small">
        <TableHead>
          <TableRow>
            <TableCell><strong>ID</strong></TableCell>
            {extraCols.map(c => <TableCell key={c.header}><strong>{c.header}</strong></TableCell>)}
            <TableCell><strong>Status</strong></TableCell>
            <TableCell><strong>Room</strong></TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {rows.length === 0 ? (
            <TableRow>
              <TableCell colSpan={extraCols.length + 3} align="center">
                <Typography variant="body2" color="text.secondary" sx={{ py: 2 }}>
                  No gear registered for this tournament.
                </Typography>
              </TableCell>
            </TableRow>
          ) : rows.map(row => (
            <TableRow key={row.reg.id}>
              <TableCell>{row.reg.equipmentid}</TableCell>
              {extraCols.map(c => (
                <TableCell key={c.header}>{c.cell(row)}</TableCell>
              ))}
              <TableCell>
                <StatusCell row={row} saving={savingIds.has(row.reg.id)} onChange={onStatusChange} />
              </TableCell>
              <TableCell>
                <RoomCell row={row} rooms={rooms} saving={savingIds.has(row.reg.id)} onChange={onRoomChange} />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  )
}

// ── Column definitions per gear type ─────────────────────────────────────────

const allCols: ColDef[] = [
  { header: 'Type',        cell: row => GEAR_TYPE_LABELS[extractDetailType(row.detail)] },
  { header: 'Description', cell: row => getGearDescription(row.detail) },
]

const computerCols: ColDef[] = [
  { header: 'Brand',        cell: row => ('Computer' in row.detail ? row.detail.Computer.brand : '—') },
  { header: 'OS',           cell: row => ('Computer' in row.detail ? row.detail.Computer.operating_system : '—') },
  { header: 'QM Version',   cell: row => ('Computer' in row.detail ? row.detail.Computer.quizmachine_version : '—') },
  { header: 'WiFi',         cell: row => ('Computer' in row.detail ? row.detail.Computer.wifi_capabilities : '—') },
  { header: 'Login User',   cell: row => ('Computer' in row.detail ? row.detail.Computer.login_username : '—') },
  { header: 'Login Pwd',    cell: row => ('Computer' in row.detail ? row.detail.Computer.login_password : '—') },
  { header: 'Output Ports', cell: row => ('Computer' in row.detail ? formatPorts(row.detail.Computer) : '—') },
  { header: 'USB',          cell: row => ('Computer' in row.detail ? (row.detail.Computer.has_usb_port ? 'Yes' : 'No') : '—') },
]

const jumpPadCols: ColDef[] = [
  { header: 'Color', cell: row => ('JumpPad' in row.detail ? row.detail.JumpPad.color : '—') },
]

const interfaceBoxCols: ColDef[] = [
  { header: 'Type',          cell: row => ('InterfaceBox' in row.detail ? row.detail.InterfaceBox.type_ : '—') },
  { header: 'Serial Number', cell: row => ('InterfaceBox' in row.detail ? (row.detail.InterfaceBox.serial_number ?? '—') : '—') },
]

const monitorCols: ColDef[] = [
  { header: 'Size',         cell: row => ('Monitor' in row.detail ? `${row.detail.Monitor.size}"` : '—') },
  { header: 'Brand',        cell: row => ('Monitor' in row.detail ? row.detail.Monitor.brand : '—') },
  { header: 'Output Ports', cell: row => ('Monitor' in row.detail ? formatPorts(row.detail.Monitor) : '—') },
]

const micRecorderCols: ColDef[] = [
  { header: 'Type', cell: row => ('MicrophoneRecorder' in row.detail ? row.detail.MicrophoneRecorder.type_ : '—') },
]

const projectorCols: ColDef[] = [
  { header: 'Brand',        cell: row => ('Projector' in row.detail ? row.detail.Projector.brand : '—') },
  { header: 'Output Ports', cell: row => ('Projector' in row.detail ? formatPorts(row.detail.Projector) : '—') },
]

const powerStripCols: ColDef[] = [
  { header: 'Make',  cell: row => ('PowerStrip' in row.detail ? row.detail.PowerStrip.make : '—') },
  { header: 'Model', cell: row => ('PowerStrip' in row.detail ? row.detail.PowerStrip.model : '—') },
  { header: 'Color', cell: row => ('PowerStrip' in row.detail ? row.detail.PowerStrip.color : '—') },
  { header: 'Plugs', cell: row => ('PowerStrip' in row.detail ? String(row.detail.PowerStrip.num_of_plugs) : '—') },
]

const extensionCordCols: ColDef[] = [
  { header: 'Color',  cell: row => ('ExtensionCord' in row.detail ? row.detail.ExtensionCord.color : '—') },
  { header: 'Length', cell: row => ('ExtensionCord' in row.detail ? row.detail.ExtensionCord.length : '—') },
]

const EXTRA_COLS_BY_TAB: ColDef[][] = [
  allCols,
  computerCols,
  jumpPadCols,
  interfaceBoxCols,
  monitorCols,
  micRecorderCols,
  projectorCols,
  powerStripCols,
  extensionCordCols,
]

// ── Main page component ───────────────────────────────────────────────────────

export const TournamentGearPage = ({ tid }: { tid: string }) => {
  const [rows, setRows] = useState<GearRow[]>([])
  const [rooms, setRooms] = useState<RoomTS[]>([])
  const [loading, setLoading] = useState(true)
  const [tab, setTab] = useState(0)
  const [savingIds, setSavingIds] = useState<Set<number>>(new Set())

  useEffect(() => {
    let cancelled = false
    setLoading(true)

    Promise.all([
      EquipmentRegistrationAPI.getByTournament(tid, 0, 500),
      RoomAPI.getByTournament(tid, 0, 500),
    ])
      .then(async ([regs, roomList]) => {
        if (cancelled) return
        setRooms(roomList)

        const uniqueEquipIds = [...new Set(regs.map(r => r.equipmentid))]
        const detailEntries = await Promise.all(
          uniqueEquipIds.map(id =>
            EquipmentSetAPI.getEquipmentDetail(id)
              .then(d => [id, d] as const)
              .catch(() => null)
          )
        )
        if (cancelled) return

        const detailMap = new Map<number, EquipmentDetail>()
        for (const entry of detailEntries) {
          if (entry) detailMap.set(entry[0], entry[1])
        }

        const gearRows: GearRow[] = regs
          .filter(r => detailMap.has(r.equipmentid))
          .map(r => ({ reg: r, detail: detailMap.get(r.equipmentid)! }))
        setRows(gearRows)
        setLoading(false)
      })
      .catch(() => { if (!cancelled) setLoading(false) })

    return () => { cancelled = true }
  }, [tid])

  const handleStatusChange = async (regId: number, status: string) => {
    setSavingIds(prev => new Set(prev).add(regId))
    try {
      const updated = await EquipmentRegistrationAPI.update(regId, { status })
      setRows(prev => prev.map(r =>
        r.reg.id === regId ? { ...r, reg: { ...r.reg, status: updated.status } } : r
      ))
    } catch (e) {
      console.error(e)
    } finally {
      setSavingIds(prev => { const s = new Set(prev); s.delete(regId); return s })
    }
  }

  const handleRoomChange = async (regId: number, roomId: string) => {
    setSavingIds(prev => new Set(prev).add(regId))
    try {
      const updated = await EquipmentRegistrationAPI.update(regId, { roomid: roomId })
      setRows(prev => prev.map(r =>
        r.reg.id === regId ? { ...r, reg: { ...r.reg, roomid: updated.roomid } } : r
      ))
    } catch (e) {
      console.error(e)
    } finally {
      setSavingIds(prev => { const s = new Set(prev); s.delete(regId); return s })
    }
  }

  if (loading) {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 3 }}>
        <CircularProgress size={20} />
        <Typography color="text.secondary">Loading gear…</Typography>
      </Box>
    )
  }

  const tabType = TAB_TYPES[tab]
  const visibleRows = tabType === null
    ? rows
    : rows.filter(r => extractDetailType(r.detail) === tabType)

  return (
    <Box>
      <Tabs
        value={tab}
        onChange={(_, v) => setTab(v)}
        variant="scrollable"
        scrollButtons="auto"
        sx={{ borderBottom: 1, borderColor: 'divider' }}
      >
        {TAB_LABELS.map(label => (
          <Tab key={label} label={label} />
        ))}
      </Tabs>

      <GearTable
        rows={visibleRows}
        rooms={rooms}
        savingIds={savingIds}
        extraCols={EXTRA_COLS_BY_TAB[tab]}
        onStatusChange={handleStatusChange}
        onRoomChange={handleRoomChange}
      />
    </Box>
  )
}
