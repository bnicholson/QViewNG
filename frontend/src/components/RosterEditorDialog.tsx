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
import { RosterAPI, type RosterTS } from '../features/RosterAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface RosterFormState {
  name: string;
  description: string;
}

const emptyState: RosterFormState = {
  name: "",
  description: "",
};

interface Props {
  coachId: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (roster: RosterTS) => void;
  /** When provided, the dialog edits an existing roster instead of creating one. */
  editingRoster?: RosterTS | null;
}

export const RosterEditorDialog = (props: Props) => {
  const { coachId, isOpen, onCancel, onSave, editingRoster } = props;
  const [form, setForm] = useState<RosterFormState>(emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const isEditing = !!editingRoster;

  const resetState = () => {
    setForm(emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    if (editingRoster) {
      setForm({
        name: editingRoster.name,
        description: editingRoster.description ?? "",
      });
    } else {
      resetState();
    }
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  }, [isOpen, editingRoster]);

  const openCancelDialog = () => {
    const isDirty = isEditing
      ? form.name !== editingRoster!.name || form.description !== (editingRoster!.description ?? "")
      : form.name !== "" || form.description !== "";
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
      setErrorMsg("Roster name is required.");
      setAlertOpened(true);
      return;
    }

    let result: RosterTS;
    try {
      if (isEditing) {
        result = await RosterAPI.update(editingRoster!.rosterid, {
          name: form.name,
          description: form.description || null,
        });
      } else {
        result = await RosterAPI.create(coachId, {
          name: form.name,
          description: form.description || null,
          created_by_userid: coachId,
        });
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
    title: isEditing ? "Save changes to roster?" : "Save new roster?",
  });

  return (
    <Dialog
      fullScreen
      open={isOpen}
      onClose={openCancelDialog}
      slots={{ transition: Transition }}
    >
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={openCancelDialog} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            {isEditing ? 'Edit Roster' : 'Create Roster'}
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
                <InputLabel>Roster Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Roster Name"
                  value={form.name}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container>
              <Grid item xs={12}>
                <InputLabel>Description</InputLabel>
                <TextareaAutosize
                  minRows={4}
                  placeholder="Optional description of this roster"
                  style={{ width: 600 }}
                  value={form.description}
                  onChange={(e) => setForm(s => ({ ...s, description: e.target.value }))}
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
