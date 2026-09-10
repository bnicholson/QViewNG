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
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DivisionSessionAPI, type DivisionSessionTS } from '../features/DivisionSessionAPI'
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { useAuth } from '../hooks/useAuth'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface FormState {
  division_session_id: string;
  name: string;
}

const emptyState: FormState = {
  division_session_id: "",
  name: "",
};

interface Props {
  /** Tournament the division belongs to (unused directly; kept for symmetry with other dialogs). */
  tid: string;
  /** Division whose sessions the bracket may belong to. */
  did: string;
  /** The pool_brackets `type` this dialog manages (e.g. "pool" or "bracket"). */
  type: string;
  /** Singular label for the entity ("Pool", "Bracket"). */
  entityLabel: string;
  isOpen: boolean;
  /** When set, the dialog edits this existing bracket instead of creating a new one. */
  bracket?: PoolBracketTS | null;
  onCancel: VoidFunction;
  onSave: (bracket: PoolBracketTS) => void;
}

export const PoolBracketEditorDialog = (props: Props) => {
  const { tid, did, type, entityLabel, isOpen, bracket, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const isEdit = !!bracket;
  const [form, setForm] = useState<FormState>(emptyState);
  const [sessions, setSessions] = useState<DivisionSessionTS[]>([]);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    if (bracket) {
      setForm({ division_session_id: bracket.division_session_id, name: bracket.name });
    } else {
      setForm(emptyState);
    }
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    DivisionSessionAPI.getByDivision(did)
      .then(items => setSessions(items))
      .catch(() => console.error("Failed to load sessions for pool/bracket form"));
  }, [isOpen, did, bracket]);

  const openCancelDialog = () => {
    const initial = bracket
      ? { division_session_id: bracket.division_session_id, name: bracket.name }
      : { division_session_id: "", name: "" };
    const isDirty = form.division_session_id !== initial.division_session_id || form.name !== initial.name;
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
    if (!form.division_session_id) {
      setErrorMsg("Session is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.name.trim()) {
      setErrorMsg(`${entityLabel} name is required.`);
      setAlertOpened(true);
      return;
    }

    let result: PoolBracketTS;
    try {
      if (bracket) {
        result = await PoolBracketAPI.update(bracket.pool_bracket_id, { name: form.name.trim() }, accessToken);
      } else {
        result = await PoolBracketAPI.create({ division_session_id: form.division_session_id, name: form.name.trim(), type }, accessToken);
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
    title: isEdit ? `Save changes to this ${entityLabel.toLowerCase()}?` : `Save new ${entityLabel.toLowerCase()}?`,
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
            {isEdit ? `Edit ${entityLabel}` : `Create ${entityLabel}`}
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
                <InputLabel>{entityLabel} Name (*required)</InputLabel>
                <TextField
                  value={form.name}
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                  placeholder={`e.g. ${entityLabel} A`}
                  fullWidth
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Session (*required)</InputLabel>
                <Select
                  value={form.division_session_id}
                  onChange={(e) => setForm(s => ({ ...s, division_session_id: e.target.value }))}
                  displayEmpty
                  fullWidth
                  disabled={isEdit}
                  renderValue={(val) => {
                    if (!val) return <em>Select a session</em>;
                    return sessions.find(s => s.division_session_id === val)?.name ?? val;
                  }}
                >
                  {sessions.map(s => (
                    <MenuItem key={s.division_session_id} value={s.division_session_id}>{s.name}</MenuItem>
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
