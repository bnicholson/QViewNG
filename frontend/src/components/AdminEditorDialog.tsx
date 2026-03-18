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
import { AdminAPI, type NewTournamentAdminPayload } from '../features/AdminAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface FormState {
  adminid: string;
  role_description: string;
  access_lvl: string;
}

const emptyState: FormState = {
  adminid: '',
  role_description: '',
  access_lvl: '1',
};

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (admin: UserTS) => void;
}

export const AdminEditorDialog = (props: Props) => {
  const { tid, isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<FormState>(emptyState);
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
    UserAPI.get(0, 200)
      .then(result => setUsers(result.items))
      .catch(() => console.error('Failed to load users for admin form'));
  }, [isOpen]);

  const openCancelDialog = () => {
    const isDirty = form.adminid !== '' || form.role_description !== '';
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
    if (!form.adminid) {
      setErrorMsg('A user must be selected.');
      setAlertOpened(true);
      return;
    }
    const lvl = parseInt(form.access_lvl, 10);
    if (isNaN(lvl) || lvl < 1) {
      setErrorMsg('Access level must be a positive number.');
      setAlertOpened(true);
      return;
    }

    const payload: NewTournamentAdminPayload = {
      tournamentid: tid,
      adminid: form.adminid,
      role_description: form.role_description,
      access_lvl: lvl,
    };

    let result: UserTS;
    try {
      result = await AdminAPI.create(tid, payload);
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
    title: 'Add admin to tournament?',
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
            Add Tournament Admin
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
                <InputLabel>User (*required)</InputLabel>
                <Select
                  value={form.adminid}
                  onChange={(e) => setForm(s => ({ ...s, adminid: e.target.value }))}
                  displayEmpty
                  fullWidth
                  renderValue={(val) => {
                    if (!val) return <em>Select a user</em>;
                    const u = users.find(u => u.id === val);
                    return u ? `${u.fname} ${u.lname} (${u.username})` : val;
                  }}
                >
                  {users.map(u => (
                    <MenuItem key={u.id} value={u.id}>
                      {u.fname} {u.lname} — {u.username} &lt;{u.email}&gt;
                    </MenuItem>
                  ))}
                </Select>
              </Grid>
              <Grid item xs={12} sm={3}>
                <InputLabel>Access Level (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  type="number"
                  inputProps={{ min: 1 }}
                  value={form.access_lvl}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, access_lvl: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={6}>
                <InputLabel>Role Description</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="e.g. Tournament Director"
                  value={form.role_description}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, role_description: e.target.value }))}
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
