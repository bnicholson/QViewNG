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
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { PoolBracketGroupAPI, type PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import { useAuth } from '../hooks/useAuth'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface PoolBracketGroupFormState {
  divisionid: string;
  name: string;
}

const emptyState: PoolBracketGroupFormState = {
  divisionid: "",
  name: "",
};

interface Props {
  tid: string;
  isOpen: boolean;
  /** When set, the Division is fixed to this id and its dropdown is disabled (e.g. from a Division profile). */
  lockedDivisionId?: string;
  /** When set, the dialog edits this existing group instead of creating a new one. */
  group?: PoolBracketGroupTS | null;
  onCancel: VoidFunction;
  onSave: (group: PoolBracketGroupTS) => void;
}

/**
 * Create / edit one poolbracketgroup — the grouping of pools and brackets that run concurrently
 * within a division (every team competes in exactly one pool per group).
 */
export const PoolBracketGroupEditorDialog = (props: Props) => {
  const { tid, isOpen, lockedDivisionId, group, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const isEdit = !!group;
  const [form, setForm] = useState<PoolBracketGroupFormState>(emptyState);
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    if (group) {
      setForm({ divisionid: group.divisionid, name: group.name });
    } else {
      setForm(lockedDivisionId ? { ...emptyState, divisionid: lockedDivisionId } : emptyState);
    }
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    DivisionAPI.getByTournament(tid, 0, 100)
      .then(items => setDivisions(items))
      .catch(() => console.error("Failed to load divisions for pool/bracket group form"));
  }, [isOpen, tid, lockedDivisionId, group]);

  const openCancelDialog = () => {
    const initial = group
      ? { divisionid: group.divisionid, name: group.name }
      : { divisionid: lockedDivisionId ?? "", name: "" };
    const isDirty = form.divisionid !== initial.divisionid || form.name !== initial.name;
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
    if (!form.divisionid) {
      setErrorMsg("Division is required.");
      setAlertOpened(true);
      return;
    }
    if (!form.name.trim()) {
      setErrorMsg("Pool/Bracket Group name is required.");
      setAlertOpened(true);
      return;
    }

    let result: PoolBracketGroupTS;
    try {
      if (group) {
        result = await PoolBracketGroupAPI.update(group.poolbracketgroupid, { name: form.name.trim() }, accessToken);
      } else {
        result = await PoolBracketGroupAPI.create({ divisionid: form.divisionid, name: form.name.trim() }, accessToken);
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
    title: isEdit ? "Save changes to this pool/bracket group?" : "Save new pool/bracket group?",
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
            {isEdit ? "Edit Pool/Bracket Group" : "Create Pool/Bracket Group"}
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
                <InputLabel>Pool/Bracket Group Name (*required)</InputLabel>
                <TextField
                  value={form.name}
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                  placeholder="e.g. Pool Play Round 1"
                  fullWidth
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Division (*required)</InputLabel>
                <Select
                  value={form.divisionid}
                  onChange={(e) => setForm(s => ({ ...s, divisionid: e.target.value }))}
                  displayEmpty
                  fullWidth
                  disabled={!!lockedDivisionId || isEdit}
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
