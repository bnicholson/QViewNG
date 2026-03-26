import React, { useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import Dialog from '@mui/material/Dialog'
import FormControlLabel from '@mui/material/FormControlLabel'
import Grid from '@mui/material/Grid'
import IconButton from '@mui/material/IconButton'
import InputLabel from '@mui/material/InputLabel'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import Slide from '@mui/material/Slide'
import Switch from '@mui/material/Switch'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { UserAPI, type NewUserPayload, type UserChangeset, type UserTS } from '../features/UserAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface FormState {
  fname: string;
  mname: string;
  lname: string;
  username: string;
  email: string;
  password: string;
  activated: boolean;
}

function toFormState(user: UserTS): FormState {
  return {
    fname: user.fname,
    mname: user.mname,
    lname: user.lname,
    username: user.username,
    email: user.email,
    password: '',
    activated: user.activated,
  };
}

const emptyState: FormState = {
  fname: '', mname: '', lname: '',
  username: '', email: '', password: '',
  activated: false,
};

interface Props {
  initialUser?: UserTS;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (user: UserTS) => void;
}

export const UserEditorDialog = (props: Props) => {
  const { initialUser, isOpen, onCancel, onSave } = props;
  const isEditMode = initialUser !== undefined;

  const [form, setForm] = useState<FormState>(isEditMode ? toFormState(initialUser!) : emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(isEditMode ? toFormState(initialUser!) : emptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg('');
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
  }, [isOpen, initialUser]);

  const isDirty = (): boolean => {
    if (isEditMode) {
      const orig = toFormState(initialUser!);
      return (Object.keys(orig) as (keyof FormState)[]).some(k => form[k] !== orig[k]);
    }
    return Object.values(form).some(v => v !== '' && v !== false);
  };

  const openCancelDialog = () => {
    if (!isDirty()) {
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
    if (!form.fname.trim() || !form.lname.trim()) {
      setErrorMsg('First and last name are required.');
      setAlertOpened(true);
      return;
    }
    if (!form.username.trim()) {
      setErrorMsg('Username is required.');
      setAlertOpened(true);
      return;
    }
    if (!form.email.trim()) {
      setErrorMsg('Email is required.');
      setAlertOpened(true);
      return;
    }
    if (!isEditMode && !form.password.trim()) {
      setErrorMsg('Password is required.');
      setAlertOpened(true);
      return;
    }

    let result: UserTS;
    try {
      if (isEditMode) {
        const changeset: UserChangeset = {
          fname: form.fname,
          mname: form.mname,
          lname: form.lname,
          username: form.username,
          email: form.email,
          activated: form.activated,
        };
        result = await UserAPI.update(initialUser!.id, changeset);
      } else {
        const payload: NewUserPayload = {
          fname: form.fname,
          mname: form.mname,
          lname: form.lname,
          username: form.username,
          email: form.email,
          hash_password: form.password,
          activated: form.activated,
        };
        result = await UserAPI.create(payload);
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
    message: 'Cancel if you want to make more changes.',
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
    title: isEditMode ? 'Save changes to user?' : 'Create user account?',
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
            {isEditMode ? 'Edit User' : 'Create User'}
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
          {/* Name row */}
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>First Name (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="First Name"
                  value={form.fname}
                  onChange={(e) => setForm(s => ({ ...s, fname: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Middle Name</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="Middle Name"
                  value={form.mname}
                  onChange={(e) => setForm(s => ({ ...s, mname: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Last Name (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="Last Name"
                  value={form.lname}
                  onChange={(e) => setForm(s => ({ ...s, lname: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          {/* Account row */}
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>Username (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="Username"
                  value={form.username}
                  onChange={(e) => setForm(s => ({ ...s, username: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Email (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth type="email" placeholder="Email"
                  value={form.email}
                  onChange={(e) => setForm(s => ({ ...s, email: e.target.value }))}
                />
              </Grid>
              {!isEditMode && (
                <Grid item xs={12} sm={4}>
                  <InputLabel>Password (*required)</InputLabel>
                  <TextField
                    variant="outlined" fullWidth type="password" placeholder="Password"
                    value={form.password}
                    onChange={(e) => setForm(s => ({ ...s, password: e.target.value }))}
                  />
                </Grid>
              )}
              <Grid item xs={12} sm={4} sx={{ display: 'flex', alignItems: 'flex-end', pb: 1 }}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={form.activated}
                      onChange={(e) => setForm(s => ({ ...s, activated: e.target.checked }))}
                    />
                  }
                  label="Activated"
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
