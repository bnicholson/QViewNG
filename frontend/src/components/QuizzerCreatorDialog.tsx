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
import { RosterAPI } from '../features/RosterAPI'
import type { UserTS } from '../features/UserAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

const validatePassword = (password: string): string | null => {
  if (password.length < 8) return 'Password must be at least 8 characters long.'
  if (!/[A-Z]/.test(password)) return 'Must contain at least one uppercase letter.'
  if (!/[a-z]/.test(password)) return 'Must contain at least one lowercase letter.'
  if (!/[0-9]/.test(password)) return 'Must contain at least one number.'
  if (!/[^A-Za-z0-9]/.test(password)) return 'Must contain at least one special character.'
  return null
}

interface FormState {
  fname: string;
  mname: string;
  lname: string;
  username: string;
  email: string;
  password: string;
}

const emptyState: FormState = { fname: '', mname: '', lname: '', username: '', email: '', password: '' };

interface Props {
  rosterId: string;
  rosterName: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (user: UserTS) => void;
}

export const QuizzerCreatorDialog = ({ rosterId, rosterName, isOpen, onCancel, onSave }: Props) => {
  const [form, setForm] = useState<FormState>(emptyState);
  const [fieldErrors, setFieldErrors] = useState<Partial<Record<keyof FormState, string>>>({});
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [saving, setSaving] = useState(false);
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(emptyState);
    setFieldErrors({});
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg('');
    setAlertOpened(false);
    setSaving(false);
  };

  useEffect(() => {
    if (isOpen) resetState();
  }, [isOpen]);

  const setField = (field: keyof FormState, value: string) => {
    setForm(s => ({ ...s, [field]: value }));
    setFieldErrors(s => ({ ...s, [field]: undefined }));
  };

  const validateField = (field: keyof FormState) => {
    const v = form[field];
    let err: string | undefined;
    if (field === 'fname' && !v.trim()) err = 'First name is required.';
    if (field === 'lname' && !v.trim()) err = 'Last name is required.';
    if (field === 'email' && v && !EMAIL_REGEX.test(v)) err = 'Please enter a valid email address.';
    if (field === 'password' && v) err = validatePassword(v) ?? undefined;
    setFieldErrors(s => ({ ...s, [field]: err }));
  };

  const isDirty = () => Object.values(form).some(v => v !== '');

  const openCancelDialog = () => {
    if (!isDirty()) { onCancel(); return; }
    setConfirmDialog({
      isOpen: true,
      message: "Any information you've entered will be lost.",
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { onCancel(); resetState(); },
      title: 'Are you sure you want to cancel?',
    });
  };

  const handleSave = async () => {
    // Validate all fields
    const errors: Partial<Record<keyof FormState, string>> = {};
    if (!form.fname.trim()) errors.fname = 'First name is required.';
    if (!form.lname.trim()) errors.lname = 'Last name is required.';
    if (form.email && !EMAIL_REGEX.test(form.email)) errors.email = 'Please enter a valid email address.';
    if (form.password) {
      const pwdErr = validatePassword(form.password);
      if (pwdErr) errors.password = pwdErr;
    }
    setFieldErrors(errors);
    if (Object.keys(errors).length > 0) return;

    setSaving(true);
    try {
      // Step 1: Register the user via the auth endpoint (handles hashing + activation token)
      let stringified_body = JSON.stringify({
        fname: form.fname,
        mname: form.mname || "",
        lname: form.lname,
        username: form.username || "",
        email: form.email,
        password: form.password || "",
      });

      console.log(stringified_body);
      
      const registerResponse = await fetch('/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: stringified_body,
      });
      if (!registerResponse.ok) {
        const body = await registerResponse.json().catch(() => ({}));
        throw new Error(body.error ?? `Registration failed (${registerResponse.status})`);
      }
      const newUser: UserTS = await registerResponse.json();

      // Step 2: Add the new user to the roster as a quizzer
      await RosterAPI.addQuizzer(rosterId, newUser.id);

      onSave(newUser);
      resetState();
    } catch (err: any) {
      setErrorMsg(err.message ?? 'Failed to create quizzer.');
      setAlertOpened(true);
    } finally {
      setSaving(false);
    }
  };

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    message: `This will create a new user account and add them to "${rosterName}".`,
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
    title: 'Create quizzer and add to roster?',
  });

  return (
    <Dialog fullScreen open={isOpen} onClose={openCancelDialog} slots={{ transition: Transition }}>
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={openCancelDialog} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            Create New Quizzer &mdash; {rosterName}
          </Typography>
          <Button autoFocus color="inherit" onClick={openSaveDialog} disabled={saving}>
            {saving ? 'Saving...' : 'Create & Add'}
          </Button>
        </Toolbar>
      </AppBar>

      <Box component="form" sx={{ maxWidth: 700, mx: 'auto', width: '100%' }}>
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
              <Grid item xs={5}>
                <InputLabel>First Name *</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="First Name"
                  value={form.fname}
                  fullWidth
                  onChange={e => setField('fname', e.target.value)}
                  onBlur={() => validateField('fname')}
                  error={!!fieldErrors.fname}
                  helperText={fieldErrors.fname}
                />
              </Grid>
              <Grid item xs={2}>
                <InputLabel>Middle</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="M.I."
                  value={form.mname}
                  fullWidth
                  onChange={e => setField('mname', e.target.value)}
                />
              </Grid>
              <Grid item xs={5}>
                <InputLabel>Last Name *</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Last Name"
                  value={form.lname}
                  fullWidth
                  onChange={e => setField('lname', e.target.value)}
                  onBlur={() => validateField('lname')}
                  error={!!fieldErrors.lname}
                  helperText={fieldErrors.lname}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <InputLabel>Username</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Username"
                  value={form.username}
                  fullWidth
                  onChange={e => setField('username', e.target.value)}
                  onBlur={() => validateField('username')}
                  error={!!fieldErrors.username}
                  helperText={fieldErrors.username}
                />
              </Grid>
              <Grid item xs={6}>
                <InputLabel>Email</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="email@example.com"
                  value={form.email}
                  fullWidth
                  onChange={e => setField('email', e.target.value)}
                  onBlur={() => validateField('email')}
                  error={!!fieldErrors.email}
                  helperText={fieldErrors.email}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <InputLabel>Password</InputLabel>
                <TextField
                  variant="outlined"
                  type="password"
                  placeholder="Password"
                  value={form.password}
                  fullWidth
                  onChange={e => setField('password', e.target.value)}
                  onBlur={() => validateField('password')}
                  error={!!fieldErrors.password}
                  helperText={fieldErrors.password ?? 'Optional. 8+ chars, upper, lower, number, special'}
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
