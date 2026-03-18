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
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { TeamAPI, type NewTeamPayload, type TeamTS } from '../features/TeamAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface TeamFormState {
  name: string;
  did: string;
  coachid: string;
}

const emptyState: TeamFormState = {
  name: '',
  did: '',
  coachid: '',
};

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (team: TeamTS) => void;
}

export const TeamEditorDialog = (props: Props) => {
  const { tid, isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<TeamFormState>(emptyState);
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg('');
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    Promise.all([
      DivisionAPI.get(0, 100),
      UserAPI.get(0, 200),
    ])
      .then(([divResult, userResult]) => {
        setDivisions(divResult.items);
        setUsers(userResult.items);
      })
      .catch(() => console.error('Failed to load form data for team editor'));
  }, [isOpen]);

  const openCancelDialog = () => {
    const isDirty = form.name !== '' || form.did !== '' || form.coachid !== '';
    if (!isDirty) {
      onCancel();
    } else {
      setConfirmDialog({
        isOpen: true,
        message: "Any changes you've made will be lost.",
        onCancel: () => setConfirmDialog(confirmDialogDefaultState),
        onConfirm: () => { onCancel(); resetState(); },
        title: 'Are you sure you want to cancel?',
      });
    }
  };

  const handleSave = async () => {
    if (!form.name.trim()) {
      setErrorMsg('Team name is required.');
      setAlertOpened(true);
      return;
    }
    if (!form.did) {
      setErrorMsg('Division is required.');
      setAlertOpened(true);
      return;
    }
    if (!form.coachid) {
      setErrorMsg('Coach is required.');
      setAlertOpened(true);
      return;
    }

    const payload: NewTeamPayload = {
      name: form.name,
      did: form.did,
      coachid: form.coachid,
    };

    let result: TeamTS;
    try {
      result = await TeamAPI.create(payload);
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
    message: 'Cancel if you want to make more changes.',
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
    title: 'Save new team?',
  });

  const userLabel = (u: UserTS) =>
    [u.fname, u.mname, u.lname].filter(Boolean).join(' ') + ` (${u.username})`;

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
            Create Team
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
              <Grid item xs={12} sm={6}>
                <InputLabel>Team Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Team Name"
                  value={form.name}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={6}>
                <InputLabel>Division (*required)</InputLabel>
                <Select
                  value={form.did}
                  onChange={(e) => setForm(s => ({ ...s, did: e.target.value }))}
                  displayEmpty
                  fullWidth
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
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={6}>
                <InputLabel>Coach (*required)</InputLabel>
                <Select
                  value={form.coachid}
                  onChange={(e) => setForm(s => ({ ...s, coachid: e.target.value }))}
                  displayEmpty
                  fullWidth
                  renderValue={(val) => {
                    if (!val) return <em>Select a coach</em>;
                    const u = users.find(u => u.id === val);
                    return u ? userLabel(u) : val;
                  }}
                >
                  {users.map(u => (
                    <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>
                  ))}
                </Select>
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
