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
import {
  CreateTournamentApplicantAPI,
  type CreateTournamentApplicantTS,
} from '../features/CreateTournamentApplicantAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

const STATUS_OPTIONS = [
  { label: 'Pending',  value: 'pending'  },
  { label: 'Approved', value: 'approved' },
  { label: 'Declined', value: 'declined' },
] as const;

interface Props {
  applicant: CreateTournamentApplicantTS | undefined;
  isOpen: boolean;
  currentUserId: string;
  onCancel: VoidFunction;
  onSave: (updated: CreateTournamentApplicantTS) => void;
}

export const CreateTournamentApplicantEditorDialog = ({
  applicant,
  isOpen,
  currentUserId,
  onCancel,
  onSave,
}: Props) => {
  const [status, setStatus] = useState('pending');
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  useEffect(() => {
    if (!isOpen || !applicant) return;
    setStatus(applicant.status);
    setAlertOpened(false);
    setErrorMsg('');
    setConfirmDialog(confirmDialogDefaultState);
  }, [isOpen, applicant]);

  const isDirty = applicant !== undefined && status !== applicant.status;

  const openCancelDialog = () => {
    if (!isDirty) {
      onCancel();
      return;
    }
    setConfirmDialog({
      isOpen: true,
      title: 'Are you sure you want to cancel?',
      message: 'Any changes you\'ve made will be lost.',
      onCancel: () => setConfirmDialog(confirmDialogDefaultState),
      onConfirm: () => { onCancel(); setConfirmDialog(confirmDialogDefaultState); },
    });
  };

  const handleSave = async () => {
    if (!applicant) return;
    try {
      const updated = await CreateTournamentApplicantAPI.update(applicant.id, {
        status,
        last_modified_user_id: currentUserId,
      });
      onSave(updated);
    } catch (err: any) {
      setErrorMsg('Failed to save: ' + err.message);
      setAlertOpened(true);
    }
  };

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    title: 'Save changes?',
    message: 'Cancel if you want to make more changes.',
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleSave(); },
  });

  if (!applicant) return null;

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
            Edit Applicant
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
              <Grid item xs={6}>
                <InputLabel>User ID</InputLabel>
                <TextField
                  variant="outlined"
                  value={applicant.user_id}
                  fullWidth
                  disabled
                  size="small"
                />
              </Grid>
              <Grid item xs={6}>
                <InputLabel>Status</InputLabel>
                <Select
                  value={status}
                  onChange={(e) => setStatus(e.target.value)}
                  fullWidth
                  size="small"
                >
                  {STATUS_OPTIONS.map((opt) => (
                    <MenuItem key={opt.value} value={opt.value}>{opt.label}</MenuItem>
                  ))}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container>
              <Grid item xs={12}>
                <InputLabel>Request Context</InputLabel>
                <TextField
                  variant="outlined"
                  value={applicant.request_context ?? ''}
                  fullWidth
                  disabled
                  multiline
                  minRows={3}
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
