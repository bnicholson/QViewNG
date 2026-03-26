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
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DivisionAPI, type DivisionTS, type NewDivisionPayload } from '../features/DivisionAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface DivisionFormState {
  dname: string;
  breadcrumb: string;
  is_public: boolean;
  shortinfo: string;
}

const emptyState: DivisionFormState = {
  dname: "",
  breadcrumb: "",
  is_public: true,
  shortinfo: "",
};

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (division: DivisionTS) => void;
}

export const DivisionEditorDialog = (props: Props) => {
  const { tid, isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<DivisionFormState>(emptyState);
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
    const isDirty = form.dname !== "" || form.breadcrumb !== "" || form.shortinfo !== "";
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
    if (!form.dname.trim()) {
      setErrorMsg("Division name is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.breadcrumb.trim()) {
      setErrorMsg("Breadcrumb is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.shortinfo.trim()) {
      setErrorMsg("Short info is required.");
      setAlertOpened(true);
      return;
    }

    const payload: NewDivisionPayload = {
      tid,
      dname: form.dname,
      breadcrumb: form.breadcrumb,
      is_public: form.is_public,
      shortinfo: form.shortinfo,
    };

    let result: DivisionTS;
    try {
      result = await DivisionAPI.create(payload);
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
    title: "Save new division?",
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
            Create Division
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
                <InputLabel>Division Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Division Name"
                  value={form.dname}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, dname: e.target.value }))}
                />
              </Grid>
              <Grid item xs={6}>
                <InputLabel>Breadcrumb (short URL name)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Breadcrumb"
                  value={form.breadcrumb}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, breadcrumb: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <InputLabel>Visibility</InputLabel>
                <Select
                  value={form.is_public ? "true" : "false"}
                  onChange={(e) => setForm(s => ({ ...s, is_public: e.target.value === "true" }))}
                >
                  <MenuItem value="true">Public</MenuItem>
                  <MenuItem value="false">Private</MenuItem>
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container>
              <Grid item xs={12}>
                <InputLabel>Short Info (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Short description of this division"
                  value={form.shortinfo}
                  style={{ width: 600 }}
                  onChange={(e) => setForm(s => ({ ...s, shortinfo: e.target.value }))}
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
