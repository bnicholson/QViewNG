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
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import Slide from '@mui/material/Slide'
import Switch from '@mui/material/Switch'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { RoomAPI, type RoomTS } from '../features/RoomAPI'
import { RoundAPI, type RoundTS } from '../features/RoundAPI'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import { GameAPI, type NewGamePayload, type GameTS } from '../features/GameAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

function formatDateTime(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: 'numeric', minute: '2-digit',
  });
}

interface GameFormState {
  org: string;
  divisionid: string;
  roomid: string;
  roundid: string;
  ruleset: string;
  ignore: boolean;
  leftteamid: string;
  centerteamid: string;
  rightteamid: string;
  quizmasterid: string;
  contentjudgeid: string;
}

const emptyState: GameFormState = {
  org: '',
  divisionid: '',
  roomid: '',
  roundid: '',
  ruleset: 'Nazarene',
  ignore: false,
  leftteamid: '',
  centerteamid: '',
  rightteamid: '',
  quizmasterid: '',
  contentjudgeid: '',
};

interface Props {
  tid: string;
  isOpen: boolean;
  onCancel: VoidFunction;
  onSave: (game: GameTS) => void;
}

export const GameEditorDialog = (props: Props) => {
  const { tid, isOpen, onCancel, onSave } = props;
  const [form, setForm] = useState<GameFormState>(emptyState);
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [rooms, setRooms] = useState<RoomTS[]>([]);
  const [rounds, setRounds] = useState<RoundTS[]>([]);
  const [teams, setTeams] = useState<TeamTS[]>([]);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [qmFromRoom, setQmFromRoom] = useState(false);
  const [cjFromRoom, setCjFromRoom] = useState(false);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(emptyState);
    setQmFromRoom(false);
    setCjFromRoom(false);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg('');
    setAlertOpened(false);
  };

  const handleRoomChange = (roomid: string) => {
    const room = rooms.find(r => r.roomid === roomid);
    const patch: Partial<GameFormState> = { roomid };
    if (room?.quizmaster_id) {
      patch.quizmasterid = room.quizmaster_id;
      setQmFromRoom(true);
    } else {
      setQmFromRoom(false);
    }
    if (room?.contentjudge_id) {
      patch.contentjudgeid = room.contentjudge_id;
      setCjFromRoom(true);
    } else {
      setCjFromRoom(false);
    }
    set(patch);
  };

  useEffect(() => {
    if (!isOpen) return;
    resetState();
    Promise.all([
      DivisionAPI.get(0, 100),
      RoomAPI.get(0, 100),
      RoundAPI.get(0, 200),
      TeamAPI.get(0, 200),
      UserAPI.get(0, 200),
    ])
      .then(([divs, rms, rnds, tms, usrs]) => {
        setDivisions(divs.items);
        setRooms(rms.items);
        setRounds(rnds.items);
        setTeams(tms.items);
        setUsers(usrs.items);
      })
      .catch(() => console.error('Failed to load form data for game editor'));
  }, [isOpen]);

  const isDirty = () => Object.entries(form).some(([k, v]) => {
    const empty = (emptyState as any)[k];
    return v !== empty;
  });

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
    if (!form.org.trim()) { setErrorMsg('Org is required.'); setAlertOpened(true); return; }
    if (!form.divisionid) { setErrorMsg('Division is required.'); setAlertOpened(true); return; }
    if (!form.roomid) { setErrorMsg('Room is required.'); setAlertOpened(true); return; }
    if (!form.roundid) { setErrorMsg('Round is required.'); setAlertOpened(true); return; }
    if (!form.ruleset.trim()) { setErrorMsg('Ruleset is required.'); setAlertOpened(true); return; }
    if (!form.leftteamid) { setErrorMsg('Left team is required.'); setAlertOpened(true); return; }
    if (!form.rightteamid) { setErrorMsg('Right team is required.'); setAlertOpened(true); return; }
    if (!form.quizmasterid) { setErrorMsg('Quizmaster is required.'); setAlertOpened(true); return; }

    const payload: NewGamePayload = {
      org: form.org,
      tournamentid: tid,
      divisionid: form.divisionid,
      roomid: form.roomid,
      roundid: form.roundid,
      ruleset: form.ruleset,
      ignore: form.ignore,
      leftteamid: form.leftteamid,
      centerteamid: form.centerteamid || null,
      rightteamid: form.rightteamid,
      quizmasterid: form.quizmasterid,
      contentjudgeid: form.contentjudgeid || null,
    };

    let result: GameTS;
    try {
      result = await GameAPI.create(payload);
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
    title: 'Create game?',
  });

  const userLabel = (u: UserTS) =>
    [u.fname, u.mname, u.lname].filter(Boolean).join(' ');  // + ` (@${u.username})`;

  const set = (patch: Partial<GameFormState>) => setForm(s => ({ ...s, ...patch }));

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
            Create Game
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
          {/* Row 1: Division, Room, Round */}
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>Division (*required)</InputLabel>
                <Select value={form.divisionid} onChange={(e) => set({ divisionid: e.target.value })}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (divisions.find(d => d.did === v)?.dname ?? v) : <em>Select a division</em>}
                >
                  {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
                </Select>
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Room (*required)</InputLabel>
                <Select value={form.roomid} onChange={(e) => handleRoomChange(e.target.value)}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (rooms.find(r => r.roomid === v)?.name ?? v) : <em>Select a room</em>}
                >
                  {rooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Round (*required)</InputLabel>
                <Select value={form.roundid} onChange={(e) => set({ roundid: e.target.value })}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? formatDateTime(rounds.find(r => r.roundid === v)?.scheduled_start_time) : <em>Select a round</em>}
                >
                  {rounds.map(r => <MenuItem key={r.roundid} value={r.roundid}>{formatDateTime(r.scheduled_start_time)}</MenuItem>)}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          {/* Row 2: Left Team, Center Team, Right Team */}
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>Left Team (*required)</InputLabel>
                <Select value={form.leftteamid} onChange={(e) => set({ leftteamid: e.target.value })}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>Select a team</em>}
                >
                  {teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Center Team</InputLabel>
                <Select value={form.centerteamid} onChange={(e) => set({ centerteamid: e.target.value })}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>None</em>}
                >
                  <MenuItem value=""><em>None</em></MenuItem>
                  {teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Right Team (*required)</InputLabel>
                <Select value={form.rightteamid} onChange={(e) => set({ rightteamid: e.target.value })}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>Select a team</em>}
                >
                  {teams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          {/* Row 3: Quizmaster, Content Judge */}
          <ListItem sx={{ display: 'block' }}>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              Note: At the time of Game creation, Games that have Rooms specified inherit the Quizmaster and Content Judge of the Room.
            </Typography>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={6}>
                <InputLabel>Quizmaster (*required)</InputLabel>
                <Select value={form.quizmasterid} onChange={(e) => set({ quizmasterid: e.target.value })}
                  displayEmpty fullWidth disabled={qmFromRoom}
                  renderValue={(v) => {
                    if (!v) return <em>Select a quizmaster</em>;
                    const u = users.find(u => u.id === v);
                    return u ? userLabel(u) : v;
                  }}
                >
                  {users.map(u => <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>)}
                </Select>
                {qmFromRoom && (
                  <Typography variant="caption" color="text.secondary">Set by Room</Typography>
                )}
              </Grid>
              <Grid item xs={12} sm={6}>
                <InputLabel>Content Judge</InputLabel>
                <Select value={form.contentjudgeid} onChange={(e) => set({ contentjudgeid: e.target.value })}
                  displayEmpty fullWidth disabled={cjFromRoom}
                  renderValue={(v) => {
                    if (!v) return <em>None</em>;
                    const u = users.find(u => u.id === v);
                    return u ? userLabel(u) : v;
                  }}
                >
                  <MenuItem value=""><em>None</em></MenuItem>
                  {users.map(u => <MenuItem key={u.id} value={u.id}>{userLabel(u)}</MenuItem>)}
                </Select>
                {cjFromRoom && (
                  <Typography variant="caption" color="text.secondary">Set by Room</Typography>
                )}
              </Grid>
            </Grid>
          </ListItem>

          {/* Row 4: Org, Ruleset, Ignore */}
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <InputLabel>Org (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="Organization"
                  value={form.org}
                  onChange={(e) => set({ org: e.target.value })}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <InputLabel>Ruleset (*required)</InputLabel>
                <TextField
                  variant="outlined" fullWidth placeholder="Ruleset"
                  value={form.ruleset}
                  onChange={(e) => set({ ruleset: e.target.value })}
                />
              </Grid>
              <Grid item xs={12} sm={4} sx={{ display: 'flex', alignItems: 'flex-end', pb: 1 }}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={form.ignore}
                      onChange={(e) => set({ ignore: e.target.checked })}
                    />
                  }
                  label="Ignore"
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
