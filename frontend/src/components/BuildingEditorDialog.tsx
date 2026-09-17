import React, { useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import Box from '@mui/material/Box'
import { SaveButton } from './SaveButton'
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
import { RoomGroupAPI, type RoomGroupTS } from '../features/RoomGroupAPI'
import { useAuth } from '../hooks/useAuth'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface FormState {
  name: string;
  notes: string;
}

const emptyState: FormState = { name: "", notes: "" };

interface Props {
  /** Tournament the building belongs to; required when creating a new building. */
  tid: string;
  isOpen: boolean;
  /** When set, the dialog edits this existing building instead of creating a new one. */
  building?: RoomGroupTS | null;
  onCancel: VoidFunction;
  onSave: (building: RoomGroupTS) => void;
}

export const BuildingEditorDialog = (props: Props) => {
  const { tid, isOpen, building, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const isEdit = !!building;
  const [form, setForm] = useState<FormState>(emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(building ? { name: building.name, notes: building.notes } : emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpen, building]);

  const openCancelDialog = () => {
    const initial = building ? { name: building.name, notes: building.notes } : emptyState;
    const isDirty = form.name !== initial.name || form.notes !== initial.notes;
    if (!isDirty) {
      onCancel();
    } else {
      setConfirmDialog({
        isOpen: true,
        message: "Any changes you've made will be lost.",
        onCancel: () => setConfirmDialog(confirmDialogDefaultState),
        onConfirm: () => { onCancel(); resetState(); },
        title: "Are you sure you want to cancel?",
      });
    }
  };

  const handleSave = async () => {
    if (!form.name.trim()) {
      setErrorMsg("Building name is required.");
      setAlertOpened(true);
      return;
    }

    let result: RoomGroupTS;
    try {
      if (building) {
        result = await RoomGroupAPI.update(building.roomgroupid, { name: form.name.trim(), notes: form.notes }, accessToken);
      } else {
        result = await RoomGroupAPI.create({ tournamentid: tid, name: form.name.trim(), notes: form.notes }, accessToken);
      }
    } catch (err: any) {
      setErrorMsg("Failed to save: " + err.message);
      setAlertOpened(true);
      return;
    }

    onSave(result);
    resetState();
  };

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    message: "Cancel if you want to make more changes.",
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
    title: isEdit ? "Save changes to this building?" : "Save new building?",
  });

  return (
    <Dialog fullScreen open={isOpen} onClose={openCancelDialog} slots={{ transition: Transition }}>
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={openCancelDialog} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            {isEdit ? "Edit Building" : "Create Building"}
          </Typography>
          <SaveButton onClick={openSaveDialog} />
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
              <Grid size={{ xs: 6 }}>
                <InputLabel>Building Name (*required)</InputLabel>
                <TextField
                  value={form.name}
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                  placeholder="e.g. Fellowship Hall"
                  fullWidth
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 12 }}>
                <InputLabel>Notes</InputLabel>
                <TextField
                  value={form.notes}
                  onChange={(e) => setForm(s => ({ ...s, notes: e.target.value }))}
                  placeholder="Anything worth noting about this building"
                  fullWidth
                  multiline
                  minRows={2}
                  size="small"
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
