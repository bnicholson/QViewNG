import { useState, useEffect } from 'react'
import Alert from '@mui/material/Alert'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import Dialog from '@mui/material/Dialog'
import FormControlLabel from '@mui/material/FormControlLabel'
import IconButton from '@mui/material/IconButton'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import CloseIcon from '@mui/icons-material/Close'
import SaveIcon from '@mui/icons-material/Save'
import type { TransitionProps } from '@mui/material/transitions'
import React from 'react'
import { EquipmentSetAPI, type GearSetTS } from '../features/EquipmentSetAPI'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'

const SlideUp = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface Props {
  ownerId: string;
  isOpen: boolean;
  onCancel: () => void;
  onSave: (gearSet: GearSetTS) => void;
  editingSet?: GearSetTS;
}

const EMPTY_FORM = { name: '', description: '', is_default: false };

export function GearSetEditorDialog({ ownerId, isOpen, onCancel, onSave, editingSet }: Props) {
  const [form, setForm] = useState(EMPTY_FORM);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [confirmClose, setConfirmClose] = useState(confirmDialogDefaultState);

  const isDirty = editingSet
    ? form.name !== editingSet.name ||
      form.description !== (editingSet.description ?? '') ||
      form.is_default !== (editingSet.is_default ?? false)
    : form.name !== '' || form.description !== '';

  useEffect(() => {
    if (isOpen) {
      setForm(
        editingSet
          ? {
              name: editingSet.name,
              description: editingSet.description ?? '',
              is_default: editingSet.is_default ?? false,
            }
          : EMPTY_FORM,
      );
      setError(null);
    }
  }, [isOpen, editingSet]);

  const handleClose = () => {
    if (isDirty) {
      setConfirmClose({
        isOpen: true,
        title: 'Discard changes?',
        message: 'You have unsaved changes. Are you sure you want to close?',
        onCancel: () => setConfirmClose(confirmDialogDefaultState),
        onConfirm: () => { setConfirmClose(confirmDialogDefaultState); onCancel(); },
      });
    } else {
      onCancel();
    }
  };

  const handleSave = async () => {
    if (!form.name.trim()) { setError('Name is required.'); return; }
    setSaving(true);
    setError(null);
    try {
      const result = editingSet
        ? await EquipmentSetAPI.update(editingSet.id, {
            name: form.name.trim(),
            description: form.description.trim() || null,
            is_default: form.is_default,
          })
        : await EquipmentSetAPI.create({
            equipmentownerid: ownerId,
            is_active: true,
            is_default: form.is_default,
            name: form.name.trim(),
            description: form.description.trim() || null,
          });
      onSave(result);
    } catch (err: any) {
      setError(err.message ?? 'An unexpected error occurred.');
    } finally {
      setSaving(false);
    }
  };

  return (
    <>
      <Dialog fullScreen open={isOpen} onClose={handleClose} TransitionComponent={SlideUp}>
        <AppBar sx={{ position: 'relative' }}>
          <Toolbar>
            <IconButton edge="start" color="inherit" onClick={handleClose} aria-label="close">
              <CloseIcon />
            </IconButton>
            <Typography sx={{ ml: 2, flex: 1 }} variant="h6">
              {editingSet ? `Edit GearSet: ${editingSet.name}` : 'Create New GearSet'}
            </Typography>
            <Button
              color="inherit"
              startIcon={<SaveIcon />}
              onClick={handleSave}
              disabled={saving || !form.name.trim()}
            >
              {saving ? 'Saving...' : 'Save'}
            </Button>
          </Toolbar>
        </AppBar>

        <Box sx={{ p: 3, maxWidth: 560 }}>
          {error && (
            <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 2 }}>
              {error}
            </Alert>
          )}

          <TextField
            label="Name"
            required
            fullWidth
            value={form.name}
            onChange={e => setForm(f => ({ ...f, name: e.target.value }))}
            sx={{ mb: 2 }}
          />

          <TextField
            label="Description"
            fullWidth
            multiline
            minRows={3}
            value={form.description}
            onChange={e => setForm(f => ({ ...f, description: e.target.value }))}
            sx={{ mb: 2 }}
          />

          <FormControlLabel
            control={
              <Checkbox
                checked={form.is_default}
                onChange={e => setForm(f => ({ ...f, is_default: e.target.checked }))}
              />
            }
            label="Mark as default GearSet"
          />
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
