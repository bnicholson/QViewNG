import React, { useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import Dialog from '@mui/material/Dialog'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface FormState {
  name: string;
  description: string;
}

const emptyState: FormState = { name: '', description: '' };

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (group: TournamentGroupTS) => void;
  /** When provided the dialog operates in edit mode. */
  initialGroup?: TournamentGroupTS;
}

export const TournamentGroupEditorDialog = ({ tid, isOpen, onCancel, onSave, initialGroup }: Props) => {
  const isEdit = Boolean(initialGroup);
  const [form, setForm] = useState<FormState>(emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(initialGroup ? { name: initialGroup.name, description: initialGroup.description ?? '' } : emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg('');
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
  }, [isOpen, initialGroup]);

  const isDirty = isEdit
    ? form.name !== (initialGroup?.name ?? '') || form.description !== (initialGroup?.description ?? '')
    : form.name !== '' || form.description !== '';

  const openCancelDialog = () => {
    if (!isDirty) { onCancel(); return; }
    setConfirmDialog({
      isOpen: true,
      title: 'Are you sure you want to cancel?',
      message: 'Any changes you\'ve made will be lost.',
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { onCancel(); resetState(); },
    });
  };

  const handleSave = async () => {
    if (!form.name.trim()) {
      setErrorMsg('Group name is required.');
      setAlertOpened(true);
      return;
    }

    let result: TournamentGroupTS;
    try {
      if (isEdit) {
        result = await TournamentGroupAPI.update(initialGroup!.tgid, {
          name: form.name.trim(),
          description: form.description.trim() || null,
        });
      } else {
        result = await TournamentGroupAPI.create(tid, {
          name: form.name.trim(),
          description: form.description.trim() || null,
        });
      }
    } catch (err: any) {
      setErrorMsg('Failed to save: ' + err.message);
      setAlertOpened(true);
      return;
    }

    onSave(result);
    resetState();
  };

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    title: isEdit ? 'Save changes?' : 'Create tournament group?',
    message: 'Cancel if you want to make more changes.',
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
  });

  return (
    <Dialog fullScreen open={isOpen} onClose={openCancelDialog} slots={{ transition: Transition }}>
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={openCancelDialog} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            {isEdit ? 'Edit Tournament Group' : 'Create Tournament Group'}
          </Typography>
          <Button autoFocus color="inherit" onClick={openSaveDialog}>
            Save
          </Button>
        </Toolbar>
      </AppBar>

      <Box component="form">
        <Collapse in={alertOpened}>
          <Alert
            severity="error"
            action={
              <IconButton aria-label="close" color="inherit" size="small" onClick={() => setAlertOpened(false)}>
                <CloseIcon fontSize="inherit" />
              </IconButton>
            }
            sx={{ mb: 2 }}
          >
            <AlertTitle>Error</AlertTitle>
            {errorMsg}
          </Alert>
        </Collapse>

        <List>
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <InputLabel>Group Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Group Name"
                  value={form.name}
                  fullWidth
                  onChange={e => setForm(s => ({ ...s, name: e.target.value }))}
                />
              </Grid>
              <Grid item xs={6}>
                <InputLabel>Description</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Optional description"
                  value={form.description}
                  fullWidth
                  onChange={e => setForm(s => ({ ...s, description: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>
        </List>
      </Box>

      <ConfirmDialog
        isOpen={confirmDialog.isOpen}
        message={confirmDialog.message}
        onCancel={confirmDialog.onCancel}
        onConfirm={confirmDialog.onConfirm}
        title={confirmDialog.title}
      />
    </Dialog>
  );
};
