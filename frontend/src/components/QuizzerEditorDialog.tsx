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
import { UserAPI, type NewUserPayload, type UserTS } from '../features/UserAPI'

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
}

const emptyState: FormState = {
  fname: '',
  mname: '',
  lname: '',
  username: '',
  email: '',
  password: '',
};

interface Props {
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (user: UserTS) => void;
}

export const QuizzerEditorDialog = (props: Props) => {
  const { isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<FormState>(emptyState);
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
  }, [isOpen]);

  const openCancelDialog = () => {
    const isDirty = Object.values(form).some(v => v !== '');
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
    if (!form.password.trim()) {
      setErrorMsg('Password is required.');
      setAlertOpened(true);
      return;
    }

    const payload: NewUserPayload = {
      fname: form.fname,
      mname: form.mname,
      lname: form.lname,
      username: form.username,
      email: form.email,
      hash_password: form.password,
      activated: false
    };

    let result: UserTS;
    try {
      result = await UserAPI.create(payload);
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
    title: 'Create quizzer account?',
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
            Create Quizzer Account
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
              <Grid item xs={12} sm={4}>
                <InputLabel>First Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="First Name"
                  value={form.fname}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, fname: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Middle Name</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Middle Name"
                  value={form.mname}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, mname: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Last Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Last Name"
                  value={form.lname}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, lname: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>Username (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Username"
                  value={form.username}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, username: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Email (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  type="email"
                  placeholder="Email"
                  value={form.email}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, email: e.target.value }))}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Password (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  type="password"
                  placeholder="Password"
                  value={form.password}
                  fullWidth
                  onChange={(e) => setForm(s => ({ ...s, password: e.target.value }))}
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
