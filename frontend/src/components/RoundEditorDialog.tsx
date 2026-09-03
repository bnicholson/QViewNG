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
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { DateTimePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import { type Dayjs } from 'dayjs'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { RoundAPI, type NewRoundPayload, type RoundTS } from '../features/RoundAPI'
import { useAuth } from '../hooks/useAuth'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface RoundFormState {
  did: string;
  name: string;
  scheduled_start_time: Dayjs | null;
}

const emptyState: RoundFormState = {
  did: "",
  name: "",
  scheduled_start_time: null,
};

interface Props {
  tid: string;
  isOpen: boolean;
  /** When set, the Division is fixed to this id and its dropdown is disabled (e.g. from a Division profile). */
  lockedDivisionId?: string;
  onCancel: VoidFunction;
  onSave: (round: RoundTS) => void;
}

export const RoundEditorDialog = (props: Props) => {
  const { tid, isOpen, lockedDivisionId, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const [form, setForm] = useState<RoundFormState>(emptyState);
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(lockedDivisionId ? { ...emptyState, did: lockedDivisionId } : emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    DivisionAPI.getByTournament(tid, 0, 100)
      .then(items => setDivisions(items))
      .catch(() => console.error("Failed to load divisions for round form"));
  }, [isOpen, tid, lockedDivisionId]);

  const openCancelDialog = () => {
    const isDirty = form.did !== "" || form.name !== "" || form.scheduled_start_time !== null;
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
    if (!form.did) {
      setErrorMsg("Division is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.name.trim()) {
      setErrorMsg("Round name is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.scheduled_start_time || !form.scheduled_start_time.isValid()) {
      setErrorMsg("Scheduled start time is required.");
      setAlertOpened(true);
      return;
    }

    const payload: NewRoundPayload = {
      did: form.did,
      name: form.name.trim(),
      scheduled_start_time: form.scheduled_start_time.toISOString(),
    };

    let result: RoundTS;
    try {
      result = await RoundAPI.create(payload, accessToken);
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
    title: "Save new round?",
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
            Create Round
          </Typography>
          <SaveButton onClick={openSaveDialog} />
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
              <Grid size={{ xs: 6 }}>
                <InputLabel>Round Name (*required)</InputLabel>
                <TextField
                  value={form.name}
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                  placeholder="e.g. 1"
                  fullWidth
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Division (*required)</InputLabel>
                <Select
                  value={form.did}
                  onChange={(e) => setForm(s => ({ ...s, did: e.target.value }))}
                  displayEmpty
                  fullWidth
                  disabled={!!lockedDivisionId}
                  renderValue={(val) => {
                    if (!val) return <em>Select a division</em>;
                    return divisions.find(d => d.did === val)?.dname ?? val;
                  }}
                >
                  {divisions.map(d => (
                    <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>
                  ))}
                </Select>
              </Grid>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Scheduled Start Time (*required)</InputLabel>
                <LocalizationProvider dateAdapter={AdapterDayjs}>
                  <DateTimePicker
                    enableAccessibleFieldDOMStructure={false}
                    value={form.scheduled_start_time}
                    onChange={(val) => setForm(s => ({ ...s, scheduled_start_time: val }))}
                    slots={{ textField: TextField }}
                  />
                </LocalizationProvider>
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
