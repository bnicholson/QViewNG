import { useState, useEffect, useCallback } from 'react'
import React from 'react'
import Alert from '@mui/material/Alert'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import CircularProgress from '@mui/material/CircularProgress'
import Dialog from '@mui/material/Dialog'
import Divider from '@mui/material/Divider'
import FormControl from '@mui/material/FormControl'
import FormControlLabel from '@mui/material/FormControlLabel'
import FormGroup from '@mui/material/FormGroup'
import FormLabel from '@mui/material/FormLabel'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import CloseIcon from '@mui/icons-material/Close'
import SaveIcon from '@mui/icons-material/Save'
import type { TransitionProps } from '@mui/material/transitions'
import {
  EquipmentSetAPI,
  GEAR_TYPE_LABELS,
  detectGearType,
  extractDetailData,
  extractDetailType,
  type EquipmentDboTS,
  type EquipmentDetail,
  type GearSetTS,
  type GearType,
} from '../features/EquipmentSetAPI'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'

const SlideUp = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

const ALL_GEAR_TYPES: GearType[] = [
  'Computer', 'JumpPad', 'InterfaceBox', 'Monitor',
  'MicrophoneRecorder', 'Projector', 'PowerStrip', 'ExtensionCord',
];

// ── Per-type default state helpers ──────────────────────────────────────────

function defaultState(type: GearType, setId: number): any {
  switch (type) {
    case 'Computer': return {
      equipmentsetid: setId, brand: '', operating_system: '', quizmachine_version: '',
      wifi_capabilities: '', login_username: '', login_password: '', misc_note: '',
      clientkey: '', has_vga_out_port: false, has_dvi_out_port: false,
      has_hdmi_out_port: false, has_display_port_out: false, has_usb_port: false,
    };
    case 'JumpPad': return { equipmentsetid: setId, color: '', misc_note: '' };
    case 'InterfaceBox': return { equipmentsetid: setId, type_: '', serial_number: '', misc_note: '' };
    case 'Monitor': return {
      equipmentsetid: setId, size: '', brand: '', misc_note: '',
      has_vga_out_port: false, has_dvi_out_port: false,
      has_hdmi_out_port: false, has_display_port_out: false,
    };
    case 'MicrophoneRecorder': return { equipmentsetid: setId, type_: 'External', misc_note: '' };
    case 'Projector': return {
      equipmentsetid: setId, brand: '', misc_note: '',
      has_vga_out_port: false, has_dvi_out_port: false,
      has_hdmi_out_port: false, has_display_port_out: false,
    };
    case 'PowerStrip': return { equipmentsetid: setId, make: '', model: '', color: '', num_of_plugs: 1, misc_note: '' };
    case 'ExtensionCord': return { equipmentsetid: setId, color: '', length: '', misc_note: '' };
  }
}

function stateFromDetail(detail: EquipmentDetail): any {
  const type = extractDetailType(detail);
  const data = extractDetailData(detail);
  // Normalize: flatten to the form-compatible shape, with empty-string fallbacks
  switch (type) {
    case 'Computer': return {
      equipmentsetid: data.equipmentsetid,
      brand: data.brand ?? '', operating_system: data.operating_system ?? '',
      quizmachine_version: data.quizmachine_version ?? '',
      wifi_capabilities: data.wifi_capabilities ?? '',
      login_username: data.login_username ?? '', login_password: data.login_password ?? '',
      misc_note: data.misc_note ?? '', clientkey: data.clientkey ?? '',
      has_vga_out_port: data.has_vga_out_port ?? false,
      has_dvi_out_port: data.has_dvi_out_port ?? false,
      has_hdmi_out_port: data.has_hdmi_out_port ?? false,
      has_display_port_out: data.has_display_port_out ?? false,
      has_usb_port: data.has_usb_port ?? false,
    };
    case 'JumpPad': return { equipmentsetid: data.equipmentsetid, color: data.color ?? '', misc_note: data.misc_note ?? '' };
    case 'InterfaceBox': return {
      equipmentsetid: data.equipmentsetid, type_: data.type_ ?? '',
      serial_number: data.serial_number ?? '', misc_note: data.misc_note ?? '',
    };
    case 'Monitor': return {
      equipmentsetid: data.equipmentsetid, size: data.size ?? '', brand: data.brand ?? '',
      misc_note: data.misc_note ?? '',
      has_vga_out_port: data.has_vga_out_port ?? false,
      has_dvi_out_port: data.has_dvi_out_port ?? false,
      has_hdmi_out_port: data.has_hdmi_out_port ?? false,
      has_display_port_out: data.has_display_port_out ?? false,
    };
    case 'MicrophoneRecorder': return {
      equipmentsetid: data.equipmentsetid, type_: data.type_ ?? 'External', misc_note: data.misc_note ?? '',
    };
    case 'Projector': return {
      equipmentsetid: data.equipmentsetid, brand: data.brand ?? '', misc_note: data.misc_note ?? '',
      has_vga_out_port: data.has_vga_out_port ?? false,
      has_dvi_out_port: data.has_dvi_out_port ?? false,
      has_hdmi_out_port: data.has_hdmi_out_port ?? false,
      has_display_port_out: data.has_display_port_out ?? false,
    };
    case 'PowerStrip': return {
      equipmentsetid: data.equipmentsetid, make: data.make ?? '', model: data.model ?? '',
      color: data.color ?? '', num_of_plugs: data.num_of_plugs ?? 1, misc_note: data.misc_note ?? '',
    };
    case 'ExtensionCord': return {
      equipmentsetid: data.equipmentsetid, color: data.color ?? '',
      length: data.length ?? '', misc_note: data.misc_note ?? '',
    };
  }
}

function stateToPayload(type: GearType, state: any): any {
  const common = {
    equipmentsetid: state.equipmentsetid,
    misc_note: state.misc_note?.trim() || null,
  };
  switch (type) {
    case 'Computer': return {
      ...common, brand: state.brand, operating_system: state.operating_system,
      quizmachine_version: state.quizmachine_version, wifi_capabilities: state.wifi_capabilities,
      login_username: state.login_username, login_password: state.login_password,
      has_vga_out_port: state.has_vga_out_port, has_dvi_out_port: state.has_dvi_out_port,
      has_hdmi_out_port: state.has_hdmi_out_port, has_display_port_out: state.has_display_port_out,
      has_usb_port: state.has_usb_port, clientkey: state.clientkey,
    };
    case 'JumpPad': return { ...common, color: state.color };
    case 'InterfaceBox': return {
      ...common, type_: state.type_, serial_number: state.serial_number?.trim() || null,
    };
    case 'Monitor': return {
      ...common, size: state.size, brand: state.brand,
      has_vga_out_port: state.has_vga_out_port, has_dvi_out_port: state.has_dvi_out_port,
      has_hdmi_out_port: state.has_hdmi_out_port, has_display_port_out: state.has_display_port_out,
    };
    case 'MicrophoneRecorder': return { ...common, type_: state.type_ };
    case 'Projector': return {
      ...common, brand: state.brand,
      has_vga_out_port: state.has_vga_out_port, has_dvi_out_port: state.has_dvi_out_port,
      has_hdmi_out_port: state.has_hdmi_out_port, has_display_port_out: state.has_display_port_out,
    };
    case 'PowerStrip': return {
      ...common, make: state.make, model: state.model, color: state.color,
      num_of_plugs: Number(state.num_of_plugs),
    };
    case 'ExtensionCord': return { ...common, color: state.color, length: state.length };
  }
}

// ── Port/bool checkbox grid ──────────────────────────────────────────────────

function PortCheckboxes({
  state, onChange, includeUsb = false,
}: {
  state: any;
  onChange: (key: string, val: boolean) => void;
  includeUsb?: boolean;
}) {
  const ports = [
    { key: 'has_vga_out_port', label: 'VGA Out' },
    { key: 'has_dvi_out_port', label: 'DVI Out' },
    { key: 'has_hdmi_out_port', label: 'HDMI Out' },
    { key: 'has_display_port_out', label: 'DisplayPort Out' },
    ...(includeUsb ? [{ key: 'has_usb_port', label: 'USB Port' }] : []),
  ];
  return (
    <FormControl component="fieldset" sx={{ mb: 2 }}>
      <FormLabel component="legend" sx={{ mb: 0.5 }}>Ports / Connections</FormLabel>
      <FormGroup row>
        {ports.map(p => (
          <FormControlLabel
            key={p.key}
            control={
              <Checkbox
                checked={!!state[p.key]}
                onChange={e => onChange(p.key, e.target.checked)}
                size="small"
              />
            }
            label={p.label}
          />
        ))}
      </FormGroup>
    </FormControl>
  );
}

// ── Per-type form fields ─────────────────────────────────────────────────────

function GearTypeForm({
  type, state, onChange,
}: {
  type: GearType;
  state: any;
  onChange: (updates: Partial<any>) => void;
}) {
  const field = (key: string, label: string, opts?: { type?: string; required?: boolean; multiline?: boolean }) => (
    <TextField
      key={key}
      label={label}
      fullWidth
      required={opts?.required}
      type={opts?.type ?? 'text'}
      multiline={opts?.multiline}
      minRows={opts?.multiline ? 2 : undefined}
      value={state[key] ?? ''}
      onChange={e => onChange({ [key]: e.target.value })}
      sx={{ mb: 2 }}
    />
  );

  switch (type) {
    case 'Computer':
      return (
        <>
          {field('brand', 'Brand', { required: true })}
          {field('operating_system', 'Operating System', { required: true })}
          {field('quizmachine_version', 'QuizMachine Version', { required: true })}
          {field('wifi_capabilities', 'Wi-Fi Capabilities', { required: true })}
          {field('login_username', 'Login Username', { required: true })}
          {field('login_password', 'Login Password', { required: true })}
          {field('clientkey', 'Client Key')}
          <PortCheckboxes state={state} onChange={(k, v) => onChange({ [k]: v })} includeUsb />
        </>
      );

    case 'JumpPad':
      return <>{field('color', 'Color', { required: true })}</>;

    case 'InterfaceBox':
      return (
        <>
          {field('type_', 'Type (e.g. "Wired", "Wireless")', { required: true })}
          {field('serial_number', 'Serial Number')}
        </>
      );

    case 'Monitor':
      return (
        <>
          {field('size', 'Screen Size (e.g. "24")', { required: true })}
          {field('brand', 'Brand', { required: true })}
          <PortCheckboxes state={state} onChange={(k, v) => onChange({ [k]: v })} />
        </>
      );

    case 'MicrophoneRecorder':
      return (
        <FormControl fullWidth required sx={{ mb: 2 }}>
          <InputLabel>Type</InputLabel>
          <Select
            value={state.type_ ?? 'External'}
            label="Type"
            onChange={e => onChange({ type_: e.target.value })}
          >
            <MenuItem value="External">External</MenuItem>
            <MenuItem value="Built-In">Built-In</MenuItem>
          </Select>
        </FormControl>
      );

    case 'Projector':
      return (
        <>
          {field('brand', 'Brand', { required: true })}
          <PortCheckboxes state={state} onChange={(k, v) => onChange({ [k]: v })} />
        </>
      );

    case 'PowerStrip':
      return (
        <>
          {field('make', 'Make / Manufacturer', { required: true })}
          {field('model', 'Model', { required: true })}
          {field('color', 'Color', { required: true })}
          <TextField
            label="Number of Plugs"
            fullWidth
            required
            type="number"
            inputProps={{ min: 1 }}
            value={state.num_of_plugs ?? 1}
            onChange={e => onChange({ num_of_plugs: parseInt(e.target.value, 10) || 1 })}
            sx={{ mb: 2 }}
          />
        </>
      );

    case 'ExtensionCord':
      return (
        <>
          {field('color', 'Color', { required: true })}
          {field('length', 'Length (e.g. "25 ft")', { required: true })}
        </>
      );
  }
}

// ── Main dialog ──────────────────────────────────────────────────────────────

interface Props {
  isOpen: boolean;
  onCancel: () => void;
  onSave: () => void;
  gearSets: GearSetTS[];
  /** When provided: editing mode — dialog loads full details for this DBO */
  editingDbo?: EquipmentDboTS;
  /** When provided (create mode): which set to default to */
  defaultSetId?: number;
}

export function GearItemEditorDialog({ isOpen, onCancel, onSave, gearSets, editingDbo, defaultSetId }: Props) {
  const isEditing = !!editingDbo;

  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [confirmClose, setConfirmClose] = useState(confirmDialogDefaultState);

  // In create mode the user picks the type; in edit mode it's fixed
  const [selectedType, setSelectedType] = useState<GearType>('Computer');
  const [form, setForm] = useState<any>({});
  const [loadedDetail, setLoadedDetail] = useState<EquipmentDetail | null>(null);

  const loadEditDetail = useCallback(async (dbo: EquipmentDboTS) => {
    setLoading(true);
    setError(null);
    try {
      const detail = await EquipmentSetAPI.getEquipmentDetail(dbo.id);
      setLoadedDetail(detail);
      setSelectedType(extractDetailType(detail));
      setForm(stateFromDetail(detail));
    } catch (err: any) {
      setError(err.message ?? 'Failed to load gear details.');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!isOpen) return;
    setError(null);
    setLoadedDetail(null);
    if (editingDbo) {
      loadEditDetail(editingDbo);
    } else {
      const initialType: GearType = 'Computer';
      setSelectedType(initialType);
      setForm(defaultState(initialType, defaultSetId ?? gearSets[0]?.id ?? 0));
    }
  }, [isOpen, editingDbo, defaultSetId, gearSets, loadEditDetail]);

  const handleTypeChange = (type: GearType) => {
    setSelectedType(type);
    setForm(defaultState(type, form.equipmentsetid ?? defaultSetId ?? gearSets[0]?.id ?? 0));
  };

  const updateForm = (updates: Partial<any>) => setForm((f: any) => ({ ...f, ...updates }));

  const handleClose = () => {
    setConfirmClose({
      isOpen: true,
      title: 'Discard changes?',
      message: 'Any unsaved changes will be lost.',
      onCancel: () => setConfirmClose(confirmDialogDefaultState),
      onConfirm: () => { setConfirmClose(confirmDialogDefaultState); onCancel(); },
    });
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    const payload = stateToPayload(selectedType, form);
    try {
      if (isEditing && editingDbo) {
        await EquipmentSetAPI.updateGearItem(selectedType, editingDbo.id, payload);
      } else {
        await EquipmentSetAPI.createGearItem(selectedType, payload);
      }
      onSave();
    } catch (err: any) {
      setError(err.message ?? 'An unexpected error occurred.');
    } finally {
      setSaving(false);
    }
  };

  const editingType = editingDbo ? detectGearType(editingDbo) : null;
  const title = isEditing
    ? `Edit ${editingType ? GEAR_TYPE_LABELS[editingType] : 'Gear Item'}`
    : 'Add Gear Item';

  return (
    <>
      <Dialog fullScreen open={isOpen} onClose={handleClose} TransitionComponent={SlideUp}>
        <AppBar sx={{ position: 'relative' }}>
          <Toolbar>
            <IconButton edge="start" color="inherit" onClick={handleClose} aria-label="close">
              <CloseIcon />
            </IconButton>
            <Typography sx={{ ml: 2, flex: 1 }} variant="h6">{title}</Typography>
            <Button
              color="inherit"
              startIcon={<SaveIcon />}
              onClick={handleSave}
              disabled={saving || loading}
            >
              {saving ? 'Saving...' : 'Save'}
            </Button>
          </Toolbar>
        </AppBar>

        <Box sx={{ p: 3, maxWidth: 640 }}>
          {loading && (
            <Box sx={{ display: 'flex', justifyContent: 'center', py: 6 }}>
              <CircularProgress />
            </Box>
          )}

          {!loading && (
            <>
              {error && (
                <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 2 }}>
                  {error}
                </Alert>
              )}

              {/* GearSet selector */}
              <FormControl fullWidth sx={{ mb: 2 }}>
                <InputLabel>GearSet</InputLabel>
                <Select
                  value={form.equipmentsetid ?? ''}
                  label="GearSet"
                  onChange={e => updateForm({ equipmentsetid: Number(e.target.value) })}
                >
                  {gearSets.map(s => (
                    <MenuItem key={s.id} value={s.id}>{s.name}</MenuItem>
                  ))}
                </Select>
              </FormControl>

              {/* Gear type selector — only shown when creating */}
              {!isEditing && (
                <FormControl fullWidth sx={{ mb: 2 }}>
                  <InputLabel>Gear Type</InputLabel>
                  <Select
                    value={selectedType}
                    label="Gear Type"
                    onChange={e => handleTypeChange(e.target.value as GearType)}
                  >
                    {ALL_GEAR_TYPES.map(t => (
                      <MenuItem key={t} value={t}>{GEAR_TYPE_LABELS[t]}</MenuItem>
                    ))}
                  </Select>
                </FormControl>
              )}

              <Divider sx={{ mb: 2 }} />

              {/* Type-specific fields */}
              <GearTypeForm type={selectedType} state={form} onChange={updateForm} />

              {/* Common note field */}
              <TextField
                label="Note"
                fullWidth
                multiline
                minRows={2}
                value={form.misc_note ?? ''}
                onChange={e => updateForm({ misc_note: e.target.value })}
                helperText="Optional — visible in all gear lists"
              />
            </>
          )}
        </Box>
      </Dialog>

      <ConfirmDialog
        isOpen={confirmClose.isOpen}
        title={confirmClose.title}
        message={confirmClose.message}
        onCancel={confirmClose.onCancel}
        onConfirm={confirmClose.onConfirm}
      />
    </>
  );
}
