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
import { PoolBracketAPI, type PoolBracketTS } from '../features/PoolBracketAPI'
import { DivisionSessionAPI, type DivisionSessionTS } from '../features/DivisionSessionAPI'
import { useAuth } from '../hooks/useAuth'

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

/** Label a round by its name, followed by its scheduled start time in parentheses (when set). */
function roundLabel(round: RoundTS | undefined): string {
  if (!round) return '';
  return round.scheduled_start_time
    ? `${round.name} (${formatDateTime(round.scheduled_start_time)})`
    : round.name;
}

interface GameFormState {
  org: string;
  divisionid: string;
  division_session_id: string;
  poolbracket_id: string;
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
  division_session_id: '',
  poolbracket_id: '',
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
  /** When set, the Division is fixed to this id and its dropdown is disabled (e.g. from a Division profile). */
  lockedDivisionId?: string;
  onCancel: VoidFunction;
  onSave: (game: GameTS) => void;
}

export const GameEditorDialog = (props: Props) => {
  const { tid, isOpen, lockedDivisionId, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const [form, setForm] = useState<GameFormState>(emptyState);
  const [divisions, setDivisions] = useState<DivisionTS[]>([]);
  const [rooms, setRooms] = useState<RoomTS[]>([]);
  const [rounds, setRounds] = useState<RoundTS[]>([]);
  const [divisionSessions, setDivisionSessions] = useState<DivisionSessionTS[]>([]);
  const [poolBrackets, setPoolBrackets] = useState<PoolBracketTS[]>([]);
  const [teams, setTeams] = useState<TeamTS[]>([]);
  const [users, setUsers] = useState<UserTS[]>([]);
  const [qmFromRoom, setQmFromRoom] = useState(false);
  const [cjFromRoom, setCjFromRoom] = useState(false);
  const [alertOpened, setAlertOpened] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);

  const resetState = () => {
    setForm(lockedDivisionId ? { ...emptyState, divisionid: lockedDivisionId } : emptyState);
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
      DivisionAPI.getByTournament(tid, 0, 100),
      RoomAPI.getByTournament(tid, 0, 100),
      RoundAPI.getByTournament(tid, 0, 200),
      TeamAPI.getByTournament(tid, 0, 200),
      UserAPI.get(0, 200),
    ])
      .then(([divs, rms, rnds, tms, usrs]) => {
        setDivisions(divs);
        setRooms(rms);
        setRounds(rnds);
        setTeams(tms.items);
        const displayName = (u: UserTS) => [u.fname, u.mname, u.lname].filter(Boolean).join(' ');
        setUsers([...usrs.items].sort((a, b) =>
          displayName(a).localeCompare(displayName(b), undefined, { sensitivity: 'base' })
        ));
      })
      .catch(() => console.error('Failed to load form data for game editor'));
  }, [isOpen, tid, lockedDivisionId]);

  // Division Sessions and Pool/Brackets are scoped to the chosen Division, so (re)load them whenever
  // it changes. (The Pool/Bracket dropdown is then further filtered to the chosen Division Session.)
  useEffect(() => {
    if (!isOpen || !form.divisionid) {
      setDivisionSessions([]);
      setPoolBrackets([]);
      return;
    }
    DivisionSessionAPI.getByDivision(form.divisionid)
      .then(setDivisionSessions)
      .catch(() => { console.error('Failed to load division sessions'); setDivisionSessions([]); });
    PoolBracketAPI.getByDivision(form.divisionid)
      .then(setPoolBrackets)
      .catch(() => { console.error('Failed to load pool brackets'); setPoolBrackets([]); });
  }, [isOpen, form.divisionid]);

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
    if (!form.divisionid) { setErrorMsg('Division is required.'); setAlertOpened(true); return; }
    if (!form.division_session_id) { setErrorMsg('Division Session is required.'); setAlertOpened(true); return; }
    if (!form.poolbracket_id) { setErrorMsg('Pool/Bracket is required.'); setAlertOpened(true); return; }
    if (!form.roomid) { setErrorMsg('Room is required.'); setAlertOpened(true); return; }
    if (!form.roundid) { setErrorMsg('Round is required.'); setAlertOpened(true); return; }
    if (!form.leftteamid) { setErrorMsg('Left team is required.'); setAlertOpened(true); return; }
    if (!form.rightteamid) { setErrorMsg('Right team is required.'); setAlertOpened(true); return; }
    if (!form.quizmasterid) { setErrorMsg('Quizmaster is required.'); setAlertOpened(true); return; }

    const payload: NewGamePayload = {
      // Org, Ruleset and Ignore are no longer collected in the form; send backend-safe defaults.
      // Neither Division nor Tournament is sent — both are derived server-side from the chosen pool
      // bracket. The Division/Session selectors here only scope the pool bracket / round / team choices.
      org: '',
      poolbracket_id: form.poolbracket_id,
      roomid: form.roomid,
      roundid: form.roundid,
      ruleset: '',
      ignore: false,
      leftteamid: form.leftteamid,
      centerteamid: form.centerteamid || null,
      rightteamid: form.rightteamid,
      quizmasterid: form.quizmasterid,
      contentjudgeid: form.contentjudgeid || null,
    };

    let result: GameTS;
    try {
      result = await GameAPI.create(payload, accessToken);
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

  // Rounds and Teams selectable for this Game are scoped to the chosen Division (not just the Tournament).
  // Until a Division is chosen, the Round/Team dropdowns stay disabled.
  const divisionChosen = !!form.divisionid;
  const sessionChosen = !!form.division_session_id;
  const divisionRounds = divisionChosen ? rounds.filter(r => r.did === form.divisionid) : [];
  const divisionTeams = divisionChosen ? teams.filter(t => t.did === form.divisionid) : [];
  // The Pool/Bracket dropdown lists only brackets in the chosen Division Session.
  const sessionPoolBrackets = sessionChosen
    ? poolBrackets.filter(b => b.division_session_id === form.division_session_id)
    : [];

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
          <SaveButton onClick={openSaveDialog} />
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
          {/* Row 1: Division, Division Session, Pool/Bracket, Room, Round */}
          <ListItem>
            <Grid container spacing={2} sx={{ width: '100%' }}>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Room (*required)</InputLabel>
                <Select value={form.roomid} onChange={(e) => handleRoomChange(e.target.value)}
                  displayEmpty fullWidth
                  renderValue={(v) => v ? (rooms.find(r => r.roomid === v)?.name ?? v) : <em>Select a room</em>}
                >
                  {rooms.map(r => <MenuItem key={r.roomid} value={r.roomid}>{r.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Division (*required)</InputLabel>
                <Select value={form.divisionid} onChange={(e) => set({ divisionid: e.target.value, division_session_id: '', poolbracket_id: '', roundid: '', leftteamid: '', centerteamid: '', rightteamid: '' })}
                  displayEmpty fullWidth disabled={!!lockedDivisionId}
                  renderValue={(v) => v ? (divisions.find(d => d.did === v)?.dname ?? v) : <em>Select a division</em>}
                >
                  {divisions.map(d => <MenuItem key={d.did} value={d.did}>{d.dname}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Division Session (*required)</InputLabel>
                <Select value={form.division_session_id} onChange={(e) => set({ division_session_id: e.target.value, poolbracket_id: '' })}
                  displayEmpty fullWidth disabled={!divisionChosen}
                  renderValue={(v) => v ? (divisionSessions.find(s => s.division_session_id === v)?.name ?? v) : <em>Select a division session</em>}
                >
                  {divisionSessions.map(s => <MenuItem key={s.division_session_id} value={s.division_session_id}>{s.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Pool/Bracket (*required)</InputLabel>
                <Select value={form.poolbracket_id} onChange={(e) => set({ poolbracket_id: e.target.value })}
                  displayEmpty fullWidth disabled={!sessionChosen}
                  renderValue={(v) => v ? (poolBrackets.find(b => b.pool_bracket_id === v)?.name ?? v) : <em>Select a pool/bracket</em>}
                >
                  {sessionPoolBrackets.map(b => <MenuItem key={b.pool_bracket_id} value={b.pool_bracket_id}>{b.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Round (*required)</InputLabel>
                <Select value={form.roundid} onChange={(e) => set({ roundid: e.target.value })}
                  displayEmpty fullWidth disabled={!divisionChosen}
                  renderValue={(v) => v ? roundLabel(rounds.find(r => r.roundid === v)) : <em>Select a round</em>}
                >
                  {divisionRounds.map(r => <MenuItem key={r.roundid} value={r.roundid}>{roundLabel(r)}</MenuItem>)}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          {/* Row 2: Left Team, Center Team, Right Team */}
          <ListItem>
            <Grid container spacing={2} sx={{ width: '100%' }}>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Left Team (*required)</InputLabel>
                <Select value={form.leftteamid} onChange={(e) => set({ leftteamid: e.target.value })}
                  displayEmpty fullWidth disabled={!divisionChosen}
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>Select a team</em>}
                >
                  {divisionTeams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Center Team</InputLabel>
                <Select value={form.centerteamid} onChange={(e) => set({ centerteamid: e.target.value })}
                  displayEmpty fullWidth disabled={!divisionChosen}
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>Select a team</em>}
                >
                  {divisionTeams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
              <Grid size={{ xs: 12, md: 7 }}>
                <InputLabel>Right Team (*required)</InputLabel>
                <Select value={form.rightteamid} onChange={(e) => set({ rightteamid: e.target.value })}
                  displayEmpty fullWidth disabled={!divisionChosen}
                  renderValue={(v) => v ? (teams.find(t => t.teamid === v)?.name ?? v) : <em>Select a team</em>}
                >
                  {divisionTeams.map(t => <MenuItem key={t.teamid} value={t.teamid}>{t.name}</MenuItem>)}
                </Select>
              </Grid>
            </Grid>
          </ListItem>

          {/* Row 3: Quizmaster, Content Judge */}
          <ListItem>
            <Grid container spacing={2} sx={{ width: '100%' }}>
              <Grid size={{ xs: 12, md: 7 }}>
                <Typography variant="body2" color="text.secondary">
                  Note: At the time of Game creation, Games that have Rooms specified inherit the Quizmaster and Content Judge of the Room.
                </Typography>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container spacing={2} sx={{ width: '100%' }}>
              <Grid size={{ xs: 12, md: 7 }}>
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
              <Grid size={{ xs: 12, md: 7 }}>
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
