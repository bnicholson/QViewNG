import React, { useEffect, useState } from 'react'
import { useAuth } from '../hooks/useAuth'
import { styled } from '@mui/material/styles'
import Typography from '@mui/material/Typography'
import Checkbox from '@mui/material/Checkbox'
import FormControlLabel from '@mui/material/FormControlLabel'
import FormGroup from '@mui/material/FormGroup'
import FormLabel from '@mui/material/FormLabel'
import Box from '@mui/material/Box'
import IconButton from '@mui/material/IconButton'
import Collapse from '@mui/material/Collapse'
import { TournamentAPI, type TournamentCreateUpdateResult, type TournamentTS } from '../features/TournamentAPI'
import Paper from '@mui/material/Paper';
import Grid from '@mui/material/Grid'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import InputLabel from '@mui/material/InputLabel'
import TextField from '@mui/material/TextField'
import dayjs, { Dayjs } from 'dayjs'
import AppBar from '@mui/material/AppBar'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogContentText from '@mui/material/DialogContentText'
import Toolbar from '@mui/material/Toolbar'
import CloseIcon from '@mui/icons-material/Close'
import Slide from '@mui/material/Slide'
import { type TransitionProps } from '@mui/material/transitions'
import Button from '@mui/material/Button';
import { SaveButton } from './SaveButton';
import ListItem from '@mui/material/ListItem'
import List from '@mui/material/List';
import TextareaAutosize from '@mui/material/TextareaAutosize';
import Alert from '@mui/material/Alert'
import AlertTitle from '@mui/material/AlertTitle'
import { ConfirmDialog, confirmDialogDefaultState } from './ConfirmDialog'
import { DesktopDatePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import MDEditor from '@uiw/react-md-editor'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & {
    children: React.ReactElement;
  },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface TournamentChangesetTS extends Omit<TournamentChangeset, "fromdate" | "todate" | "registration_open_date" | "registration_close_date"> {
  fromdate: Dayjs | null;
  todate: Dayjs | null;
  registration_open_date: Dayjs | null;
  registration_close_date: Dayjs | null;
}

/**
 * Starter markdown pre-filled into the "Full Description" editor when creating a new
 * tournament. Intentionally avoids fields already surfaced as InfoItems on the Overview
 * page (dates, registration window, venue, address, contact, organization, visibility,
 * pairing code) so the two can't drift out of sync.
 */
const DEFAULT_INFO_TEMPLATE = `## Welcome

_Write a short welcome and overview of what makes this tournament special._

## Official Website

For the latest information and updates, visit our website: [https://example.com](https://example.com)

## Schedule & Agenda

_Outline the flow of the event — e.g., check-in, opening session, rounds, breaks, and awards._

## Registration Details

_Explain who is eligible, any fees, deadlines, and how to complete registration._

## Rules & Format

_Summarize the ruleset, divisions, and how games are played._

## Lodging & Travel

_Recommend nearby hotels, parking, and directions to help attendees plan their trip._

## Meals & Amenities

_Note any provided meals, concessions, or on-site amenities._

## What to Bring

_List anything participants, coaches, or volunteers should bring._

## Frequently Asked Questions

**Q:** _Add a common question here._

**A:** _Add the answer here._
`;

const tournamentEmptyState: TournamentChangesetTS = {
  breadcrumb: "",
  city: "",
  contact: "",
  contactemail: "",
  country: "",
  fromdate: null,
  is_public: false,
  info: DEFAULT_INFO_TEMPLATE,
  organization: "Nazarene",
  shortinfo: "",
  tname: "",
  todate: null,
  venue: "",
  pairing_code: "",
  address_line_1: "",
  address_line_2: "",
  state: "",
  zip_code: "",
  registration_open_date: null,
  registration_close_date: null,
  use_team_registration: true,
  use_gear_registration: true,
  use_volunteer_registration: true
}

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode == 'dark' ? '#1a2027' : '#fff',
  ...theme.typography.body2,
  padding: theme.spacing(1),
  textAlign: 'center',
  color: theme.palette.text.secondary,
}));

interface Props {
  initialTournament?: TournamentTS;
  isOpen: boolean;
  canViewPairingCode?: boolean;
  onCancel: VoidFunction;
  onSave: (tournament: TournamentTS) => void;
}

export const TournamentEditorDialog = (props: Props) => {
  const { initialTournament, isOpen, canViewPairingCode = false, onCancel, onSave } = props;
  const { accessToken } = useAuth();
  const [tournament, setTournament] = useState<TournamentChangesetTS>(initialTournament ? initialTournament : tournamentEmptyState);
  const [alertopened, setAlertOpened] = useState(false);
  const [errormsg, setErrorMsg] = useState<string>("Simple error message");
  const [confirmDialog, setConfirmDialog] = useState(confirmDialogDefaultState);
  const [pairingCodeWarningOpen, setPairingCodeWarningOpen] = useState(false);
  const [pairingCodeWarningAcknowledged, setPairingCodeWarningAcknowledged] = useState(false);

  /** Call this whenever the tournament editor is closed. */
  const resetState = () => {
    setTournament(tournamentEmptyState);
    setConfirmDialog(confirmDialogDefaultState);
    setErrorMsg("Simple error message");
    setPairingCodeWarningOpen(false);
    setPairingCodeWarningAcknowledged(false);
  };

  // If the initial tournament changes or the dialog opens, set or clear the initial fields.
  useEffect(() => {
    if (!isOpen) return;
    if (initialTournament !== undefined) {
      setTournament(initialTournament);
    } else {
      setTournament(tournamentEmptyState);
    }
  }, [initialTournament, isOpen])

  const openCancelDialog = () => {
    if (tournament == initialTournament || tournament == tournamentEmptyState) {
      onCancel();
      resetState();
    } else {
      setConfirmDialog({
        isOpen: true,
        message: "Any changes you've made to the tournament will be lost.",
        onCancel: () => setConfirmDialog(confirmDialogDefaultState),
        onConfirm: () => { onCancel(); resetState(); },
        title: "Are you sure you want to cancel tournament edit?"
      })
    }
  };

  const handleTournamentEditorSave = async () => {
    if (!tournament.fromdate || !tournament.todate || tournament.fromdate.isAfter(tournament.todate)) {
      setErrorMsg("Invalid dates - please fill in appropriate dates");
      setAlertOpened(true);
      return;
    }

    let tournamentCS: TournamentChangeset = {
      organization: tournament.organization,
      tname: tournament.tname,
      breadcrumb: tournament.breadcrumb,
      fromdate: tournament.fromdate?.format("YYYY-MM-DD"),
      todate: tournament.todate?.format("YYYY-MM-DD"),
      venue: tournament.venue,
      city: tournament.city,
      country: tournament.country,
      contact: tournament.contact,
      contactemail: tournament.contactemail,
      is_public: tournament.is_public,
      shortinfo: tournament.shortinfo,
      info: tournament.info,
      address_line_1: tournament.address_line_1,
      address_line_2: tournament.address_line_2,
      state: tournament.state,
      zip_code: tournament.zip_code,
      registration_open_date: tournament.registration_open_date ? tournament.registration_open_date.format("YYYY-MM-DD") : null,
      registration_close_date: tournament.registration_close_date ? tournament.registration_close_date.format("YYYY-MM-DD") : null,
      use_team_registration: tournament.use_team_registration,
      use_gear_registration: tournament.use_gear_registration,
      use_volunteer_registration: tournament.use_volunteer_registration,
    };
    if (canViewPairingCode) {
      tournamentCS.pairing_code = tournament.pairing_code;
    }

    // now send the data to the backend microservice
    let result: TournamentCreateUpdateResult;
    try {
      result = initialTournament
        ? await TournamentAPI.update(initialTournament.tid, tournamentCS, accessToken)
        : await TournamentAPI.create(tournamentCS, accessToken);
    } catch(err: any) {
      const msg = err?.message === "401"
        ? "You don't have permission to create or edit tournaments. Please contact an administrator."
        : "Something went wrong and the tournament could not be saved. Please try again.";
      setErrorMsg(msg);
      setAlertOpened(true);
      return;
    }

    // confirm operation success:
    if ((result.code < 200) || (result.code > 209)) {
      setErrorMsg(result.message + " " + result.code);
      setAlertOpened(true);
      return;
    }
    console.log(result);
    setErrorMsg("Tournament Saved");
    onSave(result.data);
  };

  const openSaveDialog = () => setConfirmDialog({
    isOpen: true,
    message: "Cancel if you want to make more changes.",
    onCancel: () => setConfirmDialog(confirmDialogDefaultState),
    onConfirm: () => { setConfirmDialog(confirmDialogDefaultState); handleTournamentEditorSave(); },
    title: "Save changes to the tournament?"
  });

  // When no registration type is offered, the registration window dates are irrelevant. Each flag
  // defaults to true (matching the checkboxes) so a missing/undefined value counts as enabled —
  // only an explicit all-false disables the dates.
  const registrationDisabled =
    !(tournament.use_team_registration ?? true) &&
    !(tournament.use_gear_registration ?? true) &&
    !(tournament.use_volunteer_registration ?? true);

  return (
    <Dialog
      fullScreen
      open={isOpen}
      onClose={openCancelDialog}
      slots={{ transition: Transition }}
    >
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton
            edge="start"
            color="inherit"
            onClick={openCancelDialog}
            aria-label="close"
          >
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            Create / Edit Tournament
          </Typography>
          <SaveButton onClick={openSaveDialog} />
        </Toolbar>
      </AppBar>
      <Box component="form">
        <Collapse in={alertopened}>
          <Alert severity="error"
            action={
              <IconButton
                aria-label="close"
                color="inherit"
                size="small"
                onClick={() => {
                  setAlertOpened(false);
                }}
              >

                <CloseIcon fontSize="inherit" />
              </IconButton>
            }
            sx={{ mb: 2 }}
          >
            <AlertTitle>Error</AlertTitle>
            {errormsg}
          </Alert>
        </Collapse>
        <List>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Name (*must be unique)</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Tournament Name"
                  value={tournament.tname}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, tname: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 6 }} >
                <InputLabel>Organization</InputLabel>
                <Select
                  labelId='demo-simple-select-label55'
                  id="select-organization"
                  value={tournament.organization}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, organization: event.target.value as string }));
                  }}
                >
                  <MenuItem value={"Nazarene"}>Nazarene</MenuItem>
                  <MenuItem value={"Other"}>Other</MenuItem>
                </Select>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6, md: 4 }}>
                <InputLabel>Start Date</InputLabel>
                <Item>
                  <LocalizationProvider dateAdapter={AdapterDayjs}>
                    <DesktopDatePicker
                      enableAccessibleFieldDOMStructure={false}
                      label=""
                      format="MM/DD/YYYY"
                      value={dayjs(tournament.fromdate)}
                      onChange={fromdate => setTournament(state => ({ ...state, fromdate }))}
                      slots={{textField: TextField}}
                    />
                  </LocalizationProvider>
                </Item>
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 6, md: 4 }}>
                <InputLabel>End Date</InputLabel>
                <Item >
                  <LocalizationProvider dateAdapter={AdapterDayjs}>
                    <DesktopDatePicker
                      enableAccessibleFieldDOMStructure={false}
                      label=""
                      format="MM/DD/YYYY"
                      value={dayjs(tournament.todate)}
                      onChange={todate => setTournament(state => ({ ...state, todate }))}
                      slots={{textField: TextField}}
                    />
                  </LocalizationProvider>
                </Item>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 4 }}>
                <InputLabel>Venue</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Venue"
                  value={tournament.venue}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, venue: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 4 }}>
                <InputLabel>Visbility</InputLabel>
                <Select
                  labelId='demo-simple-select-label55'
                  id="select-organization"
                  value={tournament.is_public ? "True" : "False"}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, is_public: event.target.value === "True" }));
                  }}
                >
                  <MenuItem value={"True"}>Public</MenuItem>
                  <MenuItem value={"False"}>Private</MenuItem>
                </Select>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Address Line 1</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Address Line 1"
                  value={tournament.address_line_1}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, address_line_1: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 6 }}>
                <InputLabel>Address Line 2</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Address Line 2"
                  value={tournament.address_line_2}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, address_line_2: event.target.value as string }));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 4 }}>
                <InputLabel>City</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="City"
                  value={tournament.city}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, city: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 4 }}>
                <InputLabel>State</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="State"
                  value={tournament.state}
                  onChange={(event) => {
                    setTournament(s => ({ ...s, state: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 4 }}>
                <InputLabel>Zip Code</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Zip Code"
                  value={tournament.zip_code}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, zip_code: event.target.value as string }));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Country</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Country"
                  value={tournament.country}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, country: event.target.value as string }));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 12 }}>
                <FormLabel component="legend">Registration Types</FormLabel>
                <Typography variant="caption" color="text.secondary" sx={{ display: 'block', mb: 0.5 }}>
                  Controls which registration tabs are available on the tournament. Disabling all three
                  turns registration off entirely.
                </Typography>
                <FormGroup row>
                  <FormControlLabel
                    control={
                      <Checkbox
                        checked={tournament.use_team_registration ?? true}
                        onChange={e => setTournament(state => ({ ...state, use_team_registration: e.target.checked }))}
                      />
                    }
                    label="Team Registration&nbsp;&nbsp;&nbsp;|"
                  />
                  <FormControlLabel
                    control={
                      <Checkbox
                        checked={tournament.use_gear_registration ?? true}
                        onChange={e => setTournament(state => ({ ...state, use_gear_registration: e.target.checked }))}
                      />
                    }
                    label="Gear Registration&nbsp;&nbsp;&nbsp;|"
                  />
                  <FormControlLabel
                    control={
                      <Checkbox
                        checked={tournament.use_volunteer_registration ?? true}
                        onChange={e => setTournament(state => ({ ...state, use_volunteer_registration: e.target.checked }))}
                      />
                    }
                    label="Volunteer Registration"
                  />
                </FormGroup>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6, md: 4 }}>
                <InputLabel sx={{ color: registrationDisabled ? 'text.disabled' : undefined }}>Registration Open Date</InputLabel>
                <Item>
                  <LocalizationProvider dateAdapter={AdapterDayjs}>
                    <DesktopDatePicker
                      enableAccessibleFieldDOMStructure={false}
                      label=""
                      format="MM/DD/YYYY"
                      disabled={registrationDisabled}
                      value={tournament.registration_open_date}
                      onChange={registration_open_date => setTournament(state => ({ ...state, registration_open_date }))}
                      slotProps={{ field: { clearable: true } }}
                      slots={{ textField: TextField }}
                    />
                  </LocalizationProvider>
                </Item>
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 6, md: 4 }}>
                <InputLabel sx={{ color: registrationDisabled ? 'text.disabled' : undefined }}>Registration Close Date</InputLabel>
                <Item>
                  <LocalizationProvider dateAdapter={AdapterDayjs}>
                    <DesktopDatePicker
                      enableAccessibleFieldDOMStructure={false}
                      label=""
                      format="MM/DD/YYYY"
                      disabled={registrationDisabled}
                      value={tournament.registration_close_date}
                      onChange={registration_close_date => setTournament(state => ({ ...state, registration_close_date }))}
                      slotProps={{ field: { clearable: true } }}
                      slots={{ textField: TextField }}
                    />
                  </LocalizationProvider>
                </Item>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 6 }}>
                <InputLabel>Contact </InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Contact"
                  value={tournament.contact}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, contact: event.target.value as string }));
                  }}
                />
              </Grid>
              &nbsp;&nbsp;
              <Grid size={{ xs: 6 }}>
                <InputLabel>Contact Email</InputLabel>
                <TextField
                  variant="outlined"
                  sx={{ width: 500, maxWidth: '100%' }}
                  placeholder="Contact Email"
                  value={tournament.contactemail}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, contactemail: event.target.value as string }));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          {canViewPairingCode && (
            <ListItem>
              <Grid container>
                <Grid size={{ xs: 6 }}>
                  <InputLabel>Pairing Code (6-digit)</InputLabel>
                  <TextField
                    variant="outlined"
                    sx={{ width: 500, maxWidth: '100%' }}
                    placeholder="Pairing Code"
                    value={tournament.pairing_code ?? ""}
                    inputProps={{ maxLength: 64 }}
                    onFocus={() => {
                      if (!pairingCodeWarningAcknowledged) {
                        setPairingCodeWarningOpen(true);
                      }
                    }}
                    onChange={(event) => {
                      setTournament(state => ({ ...state, pairing_code: event.target.value as string }));
                    }}
                  />
                </Grid>
              </Grid>
            </ListItem>
          )}
          <ListItem>
            <Grid container>
              <Grid size={{ xs: 12 }}>
                <InputLabel>One line of information about the Tournament which appears when searching for Tournaments.</InputLabel>
                <TextField
                  variant="outlined"
                  placeholder="Short Information"
                  value={tournament.shortinfo}
                  style={{ width: 900 }}
                  onChange={(event) => {
                    setTournament(state => ({ ...state, shortinfo: event.target.value as string }));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem sx={{ width: '100%' }}>
            <Grid container sx={{ width: '100%' }}>
              <Grid size={{ xs: 12 }} sx={{ p: 2 }}>
                <InputLabel style={{ paddingBottom: 10 }}>Full Description (Written using{' '}
                  <a href="/markdown-cheatsheet" target="_blank" rel="noopener noreferrer">Markdown</a>
                  . Edit on the left and view the appearance on the right.)</InputLabel>
                <MDEditor
                  value={tournament.info}
                  onChange={(val) => setTournament(state => ({ ...state, info: val ?? '' }))}
                  height="80vh"
                  style={{ width: '100%' }}
                />
                {/* <div style={{textAlign:'left', margin:'3px'}}>
                  <MDEditor.Markdown source={valuemd} style={{ whiteSpace: 'pre-wrap' }} />
                </div> */}
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
      <Dialog
        open={pairingCodeWarningOpen}
        onClose={() => {
          setPairingCodeWarningOpen(false);
          setPairingCodeWarningAcknowledged(true);
        }}
      >
        <DialogContent>
          <DialogContentText sx={{ whiteSpace: 'pre-line' }}>
            {"Pairing code allows QuizMachine to receive Games from QView. Changing the pairing code means all future pairings will need to use the new code. Changing the pairing code does not recall Games that QuizMachine has already received from QView.\n\nIt is recommended to change the pairing code only if it is discovered that the pairing code has been leaked or compromised and no longer ensures pairing is limited only to Tournament Managers and Admins."}
          </DialogContentText>
        </DialogContent>
        <DialogActions sx={{ justifyContent: 'center' }}>
          <Button
            variant="contained"
            onClick={() => {
              setPairingCodeWarningOpen(false);
              setPairingCodeWarningAcknowledged(true);
            }}
          >
            I Understand
          </Button>
        </DialogActions>
      </Dialog>
    </Dialog >
  )
};
