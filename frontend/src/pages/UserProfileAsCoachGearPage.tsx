import { useCallback, useEffect, useState } from 'react'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Chip from '@mui/material/Chip'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import Menu from '@mui/material/Menu'
import MenuItem from '@mui/material/MenuItem'
import Paper from '@mui/material/Paper'
import Tab from '@mui/material/Tab'
import Table from '@mui/material/Table'
import TableBody from '@mui/material/TableBody'
import TableCell from '@mui/material/TableCell'
import TableContainer from '@mui/material/TableContainer'
import TableHead from '@mui/material/TableHead'
import TableRow from '@mui/material/TableRow'
import Tabs from '@mui/material/Tabs'
import Tooltip from '@mui/material/Tooltip'
import Typography from '@mui/material/Typography'
import AddIcon from '@mui/icons-material/Add'
import DeleteIcon from '@mui/icons-material/Delete'
import DriveFileMoveIcon from '@mui/icons-material/DriveFileMove'
import EditIcon from '@mui/icons-material/Edit'
import InventoryIcon from '@mui/icons-material/Inventory'
import {
  EquipmentSetAPI,
  GEAR_TYPE_LABELS,
  detectGearType,
  type EquipmentDboTS,
  type EquipmentDetail,
  type GearSetTS,
  type GearType,
} from '../features/EquipmentSetAPI'
import { GearItemEditorDialog } from '../components/GearItemEditorDialog'
import { GearSetEditorDialog } from '../components/GearSetEditorDialog'
import { ConfirmDialog, confirmDialogDefaultState } from '../components/ConfirmDialog'

// ── Gear-type chip colours ────────────────────────────────────────────────────

const GEAR_TYPE_COLORS: Record<GearType, 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info' | 'default'> = {
  Computer: 'primary',
  JumpPad: 'success',
  InterfaceBox: 'secondary',
  Monitor: 'info',
  MicrophoneRecorder: 'warning',
  Projector: 'default',
  PowerStrip: 'error',
  ExtensionCord: 'default',
};

// ── Move-to-GearSet button ────────────────────────────────────────────────────

function MoveButton({
  dbo,
  gearSets,
  onMoved,
}: {
  dbo: EquipmentDboTS;
  gearSets: GearSetTS[];
  onMoved: () => void;
}) {
  const [anchor, setAnchor] = useState<null | HTMLElement>(null);
  const [moving, setMoving] = useState(false);
  const otherSets = gearSets.filter(s => s.id !== dbo.equipmentsetid);

  const handleMove = async (targetSetId: number) => {
    setAnchor(null);
    const type = detectGearType(dbo);
    if (!type) return;
    setMoving(true);
    try {
      // For types with required non-optional fields (InterfaceBox, MicrophoneRecorder),
      // fetch the full detail to include those fields in the changeset.
      if (type === 'InterfaceBox' || type === 'MicrophoneRecorder') {
        const detail = await EquipmentSetAPI.getEquipmentDetail(dbo.id);
        const data = Object.values(detail)[0] as any;
        await EquipmentSetAPI.updateGearItem(type, dbo.id, {
          type_: data.type_,
          equipmentsetid: targetSetId,
        });
      } else {
        await EquipmentSetAPI.updateGearItem(type, dbo.id, { equipmentsetid: targetSetId });
      }
      onMoved();
    } catch (err) {
      console.error('Failed to move gear:', err);
    } finally {
      setMoving(false);
    }
  };

  if (otherSets.length === 0) return null;

  return (
    <>
      <Tooltip title="Move to another GearSet">
        <span>
          <IconButton
            size="small"
            onClick={e => setAnchor(e.currentTarget)}
            disabled={moving}
          >
            {moving ? <CircularProgress size={16} /> : <DriveFileMoveIcon fontSize="small" />}
          </IconButton>
        </span>
      </Tooltip>
      <Menu anchorEl={anchor} open={Boolean(anchor)} onClose={() => setAnchor(null)}>
        <Typography variant="caption" sx={{ px: 2, py: 0.5, display: 'block', color: 'text.secondary' }}>
          Move to GearSet:
        </Typography>
        {otherSets.map(s => (
          <MenuItem key={s.id} onClick={() => handleMove(s.id)}>
            {s.name}
          </MenuItem>
        ))}
      </Menu>
    </>
  );
}

// ── Gear item info string ─────────────────────────────────────────────────────

function getGearInfo(detail: EquipmentDetail, dbo: EquipmentDboTS): string {
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
  return props.map(([label, value]) => `${label}: ${value}`).join(',  ')
}

// ── Gear table ────────────────────────────────────────────────────────────────

function GearTable({
  rows,
  gearSets,
  showSetColumn,
  onEdit,
  onDelete,
  onRefresh,
}: {
  rows: EquipmentDboTS[];
  gearSets: GearSetTS[];
  showSetColumn: boolean;
  onEdit: (dbo: EquipmentDboTS) => void;
  onDelete: (dbo: EquipmentDboTS) => void;
  onRefresh: () => void;
}) {
  const setById = Object.fromEntries(gearSets.map(s => [s.id, s]));

  const [detailsByEquipId, setDetailsByEquipId] = useState<Record<number, EquipmentDetail>>({});

  useEffect(() => {
    if (rows.length === 0) return;
    Promise.all(rows.map(r => EquipmentSetAPI.getEquipmentDetail(r.id).catch(() => null)))
      .then(results => {
        const map: Record<number, EquipmentDetail> = {};
        rows.forEach((r, i) => { if (results[i]) map[r.id] = results[i]!; });
        setDetailsByEquipId(map);
      });
  }, [rows]);

  if (rows.length === 0) {
    return (
      <Typography variant="body2" color="text.secondary" sx={{ py: 3, textAlign: 'center' }}>
        No gear items yet.
      </Typography>
    );
  }

  return (
    <TableContainer component={Paper} variant="outlined" sx={{ borderRadius: 2 }}>
      <Table size="small">
        <TableHead>
          <TableRow sx={{ '& th': { fontWeight: 600, backgroundColor: 'action.hover' } }}>
            <TableCell>Type</TableCell>
            <TableCell>Info</TableCell>
            {showSetColumn && <TableCell>Gear Set</TableCell>}
            <TableCell>Added</TableCell>
            <TableCell align="right">Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {rows.map(row => {
            const type = detectGearType(row);
            return (
              <TableRow key={row.id} hover>
                <TableCell>
                  {type ? (
                    <Chip
                      label={GEAR_TYPE_LABELS[type]}
                      color={GEAR_TYPE_COLORS[type]}
                      size="small"
                      variant="outlined"
                    />
                  ) : (
                    <Chip label="Unknown" size="small" />
                  )}
                </TableCell>
                <TableCell>
                  <Typography variant="body2">
                    {detailsByEquipId[row.id]
                      ? getGearInfo(detailsByEquipId[row.id], row)
                      : `ID: ${row.id}`}
                  </Typography>
                </TableCell>
                {showSetColumn && (
                  <TableCell>
                    <Typography variant="body2">{setById[row.equipmentsetid]?.name ?? '—'}</Typography>
                  </TableCell>
                )}
                <TableCell>
                  <Typography variant="body2" color="text.secondary">
                    {new Date(row.created_at).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })}
                  </Typography>
                </TableCell>
                <TableCell align="right">
                  <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 0.5 }}>
                    {showSetColumn && (
                      <MoveButton dbo={row} gearSets={gearSets} onMoved={onRefresh} />
                    )}
                    <Tooltip title="Edit">
                      <IconButton size="small" onClick={() => onEdit(row)}>
                        <EditIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Delete">
                      <IconButton size="small" color="error" onClick={() => onDelete(row)}>
                        <DeleteIcon fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  </Box>
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

// ── All Gear panel ────────────────────────────────────────────────────────────

function AllGearPanel({
  gearSets,
  loading,
  onEdit,
  onDelete,
  onRefresh,
}: {
  gearSets: GearSetTS[];
  loading: boolean;
  onEdit: (dbo: EquipmentDboTS) => void;
  onDelete: (dbo: EquipmentDboTS) => void;
  onRefresh: () => void;
}) {
  const [allRows, setAllRows] = useState<EquipmentDboTS[]>([]);
  const [innerLoading, setInnerLoading] = useState(false);

  const load = useCallback(async () => {
    if (gearSets.length === 0) { setAllRows([]); return; }
    setInnerLoading(true);
    try {
      const perSet = await Promise.all(gearSets.map(s => EquipmentSetAPI.getEquipmentInSet(s.id)));
      setAllRows(perSet.flat());
    } catch {
      console.error('Failed to load all gear');
    } finally {
      setInnerLoading(false);
    }
  }, [gearSets]);

  useEffect(() => { load(); }, [load]);

  const handleRefresh = useCallback(() => { load(); onRefresh(); }, [load, onRefresh]);

  if (loading || innerLoading) {
    return <Box sx={{ display: 'flex', justifyContent: 'center', py: 6 }}><CircularProgress /></Box>;
  }

  return (
    <Box>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2, textAlign: 'left' }}>
        gear sets help organize your equipment for tournaments. Each piece of gear belongs to exactly
        one gear set. This tab shows all gear across your {gearSets.length} gear set{gearSets.length !== 1 ? 's' : ''}.
        Use the <DriveFileMoveIcon sx={{ fontSize: 14, verticalAlign: 'middle' }} /> button on any row to move gear between sets.
      </Typography>
      <GearTable
        rows={allRows}
        gearSets={gearSets}
        showSetColumn
        onEdit={onEdit}
        onDelete={onDelete}
        onRefresh={handleRefresh}
      />
    </Box>
  );
}

// ── Single GearSet panel ──────────────────────────────────────────────────────

function GearSetPanel({
  gearSet,
  allGearSets,
  onEditSet,
  onDeleteSet,
  onAddGear,
  onEditGear,
  onDeleteGear,
  onRefresh,
  refreshKey,
}: {
  gearSet: GearSetTS;
  allGearSets: GearSetTS[];
  onEditSet: () => void;
  onDeleteSet: () => void;
  onAddGear: () => void;
  onEditGear: (dbo: EquipmentDboTS) => void;
  onDeleteGear: (dbo: EquipmentDboTS) => void;
  onRefresh: () => void;
  refreshKey: number;
}) {
  const [rows, setRows] = useState<EquipmentDboTS[]>([]);
  const [loading, setLoading] = useState(false);
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      setRows(await EquipmentSetAPI.getEquipmentInSet(gearSet.id));
    } catch {
      console.error('Failed to load gear for set', gearSet.id);
    } finally {
      setLoading(false);
    }
  }, [gearSet.id]);

  useEffect(() => { load(); }, [load, refreshKey]);

  const handleRefresh = useCallback(() => { load(); onRefresh(); }, [load, onRefresh]);

  return (
    <Box>
      {/* GearSet header */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', mb: 2, flexWrap: 'wrap', gap: 1 }}>
        <Box>
          {gearSet.description && (
            <Typography variant="body2" color="text.secondary">{gearSet.description}</Typography>
          )}
          <Box sx={{ display: 'flex', gap: 1, mt: 0.5, flexWrap: 'wrap' }}>
            {!gearSet.is_active && <Chip label="Inactive" size="small" color="warning" variant="outlined" />}
            <Typography variant="caption" color="text.disabled">
              Created {new Date(gearSet.created_at).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })}
            </Typography>
          </Box>
        </Box>
        <Box sx={{ display: 'flex', gap: 1 }}>
          <Button size="small" variant="outlined" startIcon={<EditIcon />} onClick={onEditSet} sx={{ textTransform: 'none' }}>
            Edit Gear Set
          </Button>
          <Button
            size="small"
            variant="outlined"
            color="error"
            startIcon={<DeleteIcon />}
            onClick={() => setConfirmDialog({
              isOpen: true,
              title: 'Delete this Gear Set?',
              message: `"${gearSet.name}" and all its gear items will be permanently removed.`,
              onCancel: () => setConfirmDialog(confirmDialogDefaultState),
              onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); onDeleteSet(); },
            })}
            sx={{ textTransform: 'none' }}
          >
            Delete Gear Set
          </Button>
        </Box>
      </Box>

      <Divider sx={{ mb: 2 }} />

      {/* Gear items section */}
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 1.5 }}>
        <Typography variant="subtitle1" fontWeight={600}>Gear Items</Typography>
        <Button
          size="small"
          variant="contained"
          startIcon={<AddIcon />}
          onClick={onAddGear}
          sx={{ ml: 'auto', textTransform: 'none' }}
        >
          Add Gear
        </Button>
      </Box>

      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', py: 4 }}><CircularProgress /></Box>
      ) : (
        <GearTable
          rows={rows}
          gearSets={allGearSets}
          showSetColumn={false}
          onEdit={onEditGear}
          onDelete={onDeleteGear}
          onRefresh={handleRefresh}
        />
      )}

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        title={confirmDialog.title}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
      />
    </Box>
  );
}

// ── Main page ─────────────────────────────────────────────────────────────────

export const UserProfileAsCoachGearPage = (props: { userId: string; isSuperUser: boolean }) => {
  const { userId } = props;

  const [gearSets, setGearSets] = useState<GearSetTS[]>([]);
  const [loading, setLoading] = useState(false);
  const [tabIndex, setTabIndex] = useState(0); // 0 = All Gear, 1..N = set tabs

  // Controls which set tab to refresh (bump to trigger reload)
  const [setRefreshKey, setSetRefreshKey] = useState(0);

  // Dialog state
  const [createSetOpen, setCreateSetOpen] = useState(false);
  const [editSetOpen, setEditSetOpen] = useState(false);
  const [gearItemOpen, setGearItemOpen] = useState(false);
  const [editingDbo, setEditingDbo] = useState<EquipmentDboTS | undefined>();
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const loadGearSets = useCallback(async () => {
    setLoading(true);
    try {
      const sets = await EquipmentSetAPI.getByOwner(userId);
      setGearSets(sets);
    } catch {
      console.error('Failed to load gear sets');
    } finally {
      setLoading(false);
    }
  }, [userId]);

  useEffect(() => { loadGearSets(); }, [loadGearSets]);

  const selectedSet = tabIndex > 0 && tabIndex <= gearSets.length ? gearSets[tabIndex - 1] : null;

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    if (newValue === gearSets.length + 1) {
      setCreateSetOpen(true);
      return;
    }
    setTabIndex(newValue);
  };

  const handleCreateSetSave = async (gs: GearSetTS) => {
    setCreateSetOpen(false);
    await loadGearSets();
    setGearSets(prev => {
      const idx = prev.findIndex(s => s.id === gs.id);
      if (idx >= 0) setTabIndex(idx + 1);
      return prev;
    });
  };

  const handleEditSetSave = (gs: GearSetTS) => {
    setEditSetOpen(false);
    setGearSets(prev => prev.map(s => s.id === gs.id ? gs : s));
  };

  const handleDeleteSet = async () => {
    if (!selectedSet) return;
    try {
      await EquipmentSetAPI.delete(selectedSet.id);
      setTabIndex(0);
      await loadGearSets();
    } catch (err: any) {
      console.error('Failed to delete gear set:', err.message);
    }
  };

  const handleDeleteGear = (dbo: EquipmentDboTS) => {
    const type = detectGearType(dbo);
    if (!type) return;
    setConfirmDialog({
      isOpen: true,
      title: 'Delete this gear item?',
      message: `This ${GEAR_TYPE_LABELS[type]}${dbo.misc_note ? ` ("${dbo.misc_note}")` : ''} will be permanently removed.`,
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: async () => {
        setConfirmDialog(confirmDialogDefaultState);
        try {
          await EquipmentSetAPI.deleteGearItem(type, dbo.id);
          setSetRefreshKey(k => k + 1);
        } catch (err: any) {
          console.error('Failed to delete gear item:', err.message);
        }
      },
    });
  };

  const handleEditGear = (dbo: EquipmentDboTS) => {
    setEditingDbo(dbo);
    setGearItemOpen(true);
  };

  const handleAddGear = () => {
    setEditingDbo(undefined);
    setGearItemOpen(true);
  };

  const handleGearItemSaved = () => {
    setGearItemOpen(false);
    setEditingDbo(undefined);
    setSetRefreshKey(k => k + 1);
  };

  return (
    <Box>
      {/* Tab bar */}
      <Box sx={{ borderBottom: 1, borderColor: 'divider', mb: 2 }}>
        <Tabs
          value={tabIndex}
          onChange={handleTabChange}
          variant="scrollable"
          scrollButtons="auto"
          sx={{ '& .MuiTab-root': { textTransform: 'none', minHeight: 48, fontWeight: 500 } }}
        >
          <Tab
            label="All Gear"
            icon={<InventoryIcon fontSize="small" />}
            iconPosition="start"
          />
          {gearSets.map(s => (
            <Tab
              key={s.id}
              label={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.75 }}>
                  {s.name}
                </Box>
              }
            />
          ))}
          <Tab
            icon={<AddIcon fontSize="small" />}
            aria-label="Create new GearSet"
            sx={{ minWidth: 48 }}
          />
        </Tabs>
      </Box>

      {/* All Gear tab */}
      {tabIndex === 0 && (
        <AllGearPanel
          gearSets={gearSets}
          loading={loading}
          onEdit={handleEditGear}
          onDelete={handleDeleteGear}
          onRefresh={() => setSetRefreshKey(k => k + 1)}
        />
      )}

      {/* Individual GearSet tabs */}
      {selectedSet && (
        <GearSetPanel
          key={selectedSet.id}
          gearSet={selectedSet}
          allGearSets={gearSets}
          onEditSet={() => setEditSetOpen(true)}
          onDeleteSet={handleDeleteSet}
          onAddGear={handleAddGear}
          onEditGear={handleEditGear}
          onDeleteGear={handleDeleteGear}
          onRefresh={() => setSetRefreshKey(k => k + 1)}
          refreshKey={setRefreshKey}
        />
      )}

      {/* Empty state */}
      {gearSets.length === 0 && tabIndex === 0 && !loading && (
        <Box sx={{ textAlign: 'center', py: 8 }}>
          <InventoryIcon sx={{ fontSize: 48, color: 'text.disabled', mb: 1 }} />
          <Typography variant="body1" color="text.secondary" sx={{ mb: 2 }}>
            No gear sets yet. Create your first gear set to start tracking your equipment.
          </Typography>
          <Button variant="contained" startIcon={<AddIcon />} onClick={() => setCreateSetOpen(true)}>
            Create Gear Set
          </Button>
        </Box>
      )}

      {/* Dialogs */}
      <GearSetEditorDialog
        ownerId={userId}
        isOpen={createSetOpen}
        onCancel={() => setCreateSetOpen(false)}
        onSave={handleCreateSetSave}
      />

      {selectedSet && (
        <GearSetEditorDialog
          ownerId={userId}
          isOpen={editSetOpen}
          onCancel={() => setEditSetOpen(false)}
          onSave={handleEditSetSave}
          editingSet={selectedSet}
        />
      )}

      <GearItemEditorDialog
        isOpen={gearItemOpen}
        onCancel={() => { setGearItemOpen(false); setEditingDbo(undefined); }}
        onSave={handleGearItemSaved}
        gearSets={gearSets}
        editingDbo={editingDbo}
        defaultSetId={selectedSet?.id}
      />

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        title={confirmDialog.title}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
      />
    </Box>
  );
};
