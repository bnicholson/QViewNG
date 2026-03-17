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
import TextareaAutosize from '@mui/material/TextareaAutosize'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { RoomAPI, type NewRoomPayload, type RoomTS } from '../features/RoomAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface RoomFormState {
  name: string;
  building: string;
  comments: string;
  clientkey: string;
}

const emptyState: RoomFormState = {
  name: "",
  building: "",
  comments: "",
  clientkey: "",
};

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (room: RoomTS) => void;
}

export const RoomEditorDialog = (props: Props) => {
  const { tid, isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<RoomFormState>(emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
  }, [isOpen]);

  const openCancelDialog = () => {
    const isDirty = form.name !== "" || form.building !== "" || form.comments !== "" || form.clientkey !== "";
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
      setErrorMsg("Room name is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.building.trim()) {
      setErrorMsg("Building is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.clientkey.trim()) {
      setErrorMsg("Client key is required.");
      setAlertOpened(true);
      return;
    }

    const payload: NewRoomPayload = {
      tid,
      name: form.name,
      building: form.building,
      comments: form.comments,
      clientkey: form.clientkey,
    };

    let result: RoomTS;
    try {
      result = await RoomAPI.create(payload);
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
    title: "Save new room?",
  });

  return (
    <Dialog
      fullScreen
      open={isOpen}
      onClose={openCancelDialog}
      slots={{ transition: Transition }}
    >
      <AppBar sx={{ position: 'relative' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={openCancelDialog} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            Create Room
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
              <IconButton
                aria-label="close"
                color="inherit"
                size="small"
                onClick={() => setAlertOpened(false)}
              >
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
                <InputLabel>Room Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Room Name"
                  value={form.name}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                />
              </Grid>
              <Grid item xs={6}>
                <InputLabel>Building (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Building"
                  value={form.building}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, building: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <InputLabel>Client Key (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Client Key"
                  value={form.clientkey}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, clientkey: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container>
              <Grid item xs={12}>
                <InputLabel>Comments</InputLabel>
                <TextareaAutosize
                  minRows={4}
                  placeholder="Any comments about this room"
                  style={{ width: 600 }}
                  value={form.comments}
                  onChange={(e) => setForm(s => ({ ...s, comments: e.target.value }))}
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
