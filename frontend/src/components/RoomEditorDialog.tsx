import React, { useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import Autocomplete from '@mui/material/Autocomplete'
import Box from '@mui/material/Box'
import Checkbox from '@mui/material/Checkbox'
import FormControlLabel from '@mui/material/FormControlLabel'
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
import TextareaAutosize from '@mui/material/TextareaAutosize'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { RoomAPI, type NewRoomPayload, type RoomTS } from '../features/RoomAPI'
import { RoomGroupAPI, type RoomGroupTS } from '../features/RoomGroupAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { useAuth } from '../hooks/useAuth'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface RoomFormState {
  name: string;
  roomgroupid: string;        // "" = no building
  comments: string;
  clientkey: string;
  supports_threeteam_games: boolean;
  quizmaster_id: string | null;
  contentjudge_id: string | null;
}

const emptyState: RoomFormState = {
  name: "",
  roomgroupid: "",
  comments: "",
  clientkey: "",
  supports_threeteam_games: false,
  quizmaster_id: null,
  contentjudge_id: null,
};

const formFromRoom = (room: RoomTS): RoomFormState => ({
  name: room.name,
  roomgroupid: room.roomgroupid ?? "",
  comments: room.comments,
  clientkey: room.clientkey,
  supports_threeteam_games: room.supports_threeteam_games,
  quizmaster_id: room.quizmaster_id,
  contentjudge_id: room.contentjudge_id,
});

const personName = (u: UserTS) => `${u.fname} ${u.lname}`;

interface Props {
  tid: string;
  isOpen: boolean;
  /** When set, the dialog edits this existing room instead of creating a new one. */
  room?: RoomTS | null;
  /** When set, a newly created room is placed in this roomgroup (building), and the Building can't be changed. */
  lockedRoomGroupId?: string;
  /** Whether the Client Key is shown and editable (owner / system admin only). */
  showSensitiveFields?: boolean;
  onCancel: VoidFunction;
  onSave: (room: RoomTS) => void;
}

export const RoomEditorDialog = (props: Props) => {
  const { tid, isOpen, room, lockedRoomGroupId, showSensitiveFields = false, onCancel, onSave } = props;
  const isEdit = !!room;
  const { accessToken } = useAuth();
  const [form, setForm] = useState<RoomFormState>(emptyState);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);
  // The tournament's buildings, for the Building dropdown.
  const [buildings, setBuildings] = useState<RoomGroupTS[]>([]);
  // Candidates for Quizmaster / Content Judge (edit only), and the officials already assigned to
  // the tournament's other rooms — a person can only officiate in one room.
  const [users, setUsers] = useState<UserTS[]>([]);
  const [occupiedIds, setOccupiedIds] = useState<Set<string>>(new Set());

  const initialForm = (): RoomFormState =>
    room ? formFromRoom(room) : { ...emptyState, roomgroupid: lockedRoomGroupId ?? "" };

  const resetState = () => {
    setForm(initialForm());
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("");
    setAlertOpened(false);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    RoomGroupAPI.getByTournament(tid)
      .then(bs => setBuildings([...bs].sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }))))
      .catch(() => setBuildings([]));
    if (!room) return;
    RoomAPI.getByTournament(tid, 0, 500).then(rooms => {
      const ids = new Set<string>();
      for (const r of rooms) {
        if (r.roomid === room.roomid) continue;
        if (r.quizmaster_id) ids.add(r.quizmaster_id);
        if (r.contentjudge_id) ids.add(r.contentjudge_id);
      }
      setOccupiedIds(ids);
    }).catch(() => setOccupiedIds(new Set()));
    // Everyone selectable, plus the current officials in case they fall outside that page.
    const current = [room.quizmaster_id, room.contentjudge_id].filter((id): id is string => !!id);
    Promise.all([
      UserAPI.get(0, 100).then(p => p.items).catch(() => [] as UserTS[]),
      ...current.map(id => UserAPI.getById(id).catch(() => null)),
    ]).then(([page, ...assigned]) => {
      const byId = new Map<string, UserTS>();
      for (const u of [...(page as UserTS[]), ...(assigned as (UserTS | null)[])]) if (u) byId.set(u.id, u);
      setUsers([...byId.values()].sort((a, b) => personName(a).localeCompare(personName(b))));
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpen, room, tid, lockedRoomGroupId]);

  const openCancelDialog = () => {
    const isDirty = JSON.stringify(form) !== JSON.stringify(initialForm());
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
    if (!form.name.trim()) {
      setErrorMsg("Room name is required.");
      setAlertOpened(true);
      return;
    }

    let result: RoomTS;
    try {
      if (room) {
        // The legacy free-text `building` is left as is; the Building is the roomgroup.
        result = await RoomAPI.update(room.roomid, {
          name: form.name,
          comments: form.comments,
          roomgroupid: form.roomgroupid || null,
          supports_threeteam_games: form.supports_threeteam_games,
          quizmaster_id: form.quizmaster_id,
          contentjudge_id: form.contentjudge_id,
          ...(showSensitiveFields ? { clientkey: form.clientkey } : {}),
        }, accessToken);
      } else {
        const payload: NewRoomPayload = {
          tid,
          name: form.name,
          // The free-text Building and the Client Key aren't collected on create; send blank strings.
          building: "",
          comments: form.comments,
          clientkey: "",
          roomgroupid: form.roomgroupid || null,
          supports_threeteam_games: form.supports_threeteam_games,
        };
        result = await RoomAPI.create(payload, accessToken);
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
    title: isEdit ? "Save changes to this room?" : "Save new room?",
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
            {isEdit ? "Edit Room" : "Create Room"}
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
              <Grid size={{ xs: 12 }}>
                <InputLabel>Room Name (*required)</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Room Name"
                  value={form.name}
                  sx={{ width: 600, maxWidth: '100%' }}
                  onChange={(e) => setForm(s => ({ ...s, name: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <Grid container>
              <Grid size={{ xs: 12 }}>
                <InputLabel>Building</InputLabel>
                <Select
                  value={form.roomgroupid}
                  displayEmpty
                  disabled={!isEdit && !!lockedRoomGroupId}
                  sx={{ width: 600, maxWidth: '100%' }}
                  onChange={(e) => setForm(s => ({ ...s, roomgroupid: e.target.value }))}
                >
                  <MenuItem value=""><em>None</em></MenuItem>
                  {buildings.map(b => <MenuItem key={b.roomgroupid} value={b.roomgroupid}>{b.name}</MenuItem>)}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          {isEdit && showSensitiveFields && (
            <ListItem>
              <Grid container>
                <Grid size={{ xs: 12 }}>
                  <InputLabel>Client Key</InputLabel>
                  <TextField
                    variant="outlined"
                    placeholder="Client Key"
                    value={form.clientkey}
                    sx={{ width: 600, maxWidth: '100%' }}
                    onChange={(e) => setForm(s => ({ ...s, clientkey: e.target.value }))}
                  />
                </Grid>
              </Grid>
            </ListItem>
          )}

          {isEdit && (
            <ListItem>
              <Grid container spacing={2}>
                {([
                  ['quizmaster_id', 'Quizmaster', 'contentjudge_id'],
                  ['contentjudge_id', 'Content Judge', 'quizmaster_id'],
                ] as const).map(([field, label, other]) => (
                  <Grid key={field} size={{ xs: 12, md: 6 }}>
                    <InputLabel>{label}</InputLabel>
                    <Autocomplete
                      options={users}
                      value={users.find(u => u.id === form[field]) ?? null}
                      onChange={(_e, u) => setForm(s => ({ ...s, [field]: u ? u.id : null }))}
                      // Not someone officiating in another room, nor this room's other official.
                      getOptionDisabled={(u) => occupiedIds.has(u.id) || u.id === form[other]}
                      getOptionLabel={personName}
                      isOptionEqualToValue={(a, b) => a.id === b.id}
                      renderOption={(optProps, u) => (
                        <li {...optProps} key={u.id}>{personName(u)} <Typography component="span" variant="body2" color="text.secondary" sx={{ ml: 1 }}>{u.username}</Typography></li>
                      )}
                      renderInput={(params) => <TextField {...params} placeholder={`No ${label.toLowerCase()} assigned`} />}
                      sx={{ width: 400, maxWidth: '100%' }}
                    />
                  </Grid>
                ))}
              </Grid>
            </ListItem>
          )}

          <ListItem>
            <Grid container>
              <Grid size={{ xs: 12 }}>
                <InputLabel>Comments</InputLabel>
                <TextareaAutosize
                  minRows={4}
                  placeholder="Any comments about this room"
                  style={{ width: 600 }}
                  value={form.comments}
                  onChange={(e) => setForm(s => ({ ...s, comments: e.target.value }))}
                />
              </Grid>
            </Grid>
          </ListItem>

          <ListItem>
            <FormControlLabel
              control={
                <Checkbox
                  checked={form.supports_threeteam_games}
                  onChange={(e) => setForm(s => ({ ...s, supports_threeteam_games: e.target.checked }))}
                />
              }
              label="Supports 3-team games"
            />
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
