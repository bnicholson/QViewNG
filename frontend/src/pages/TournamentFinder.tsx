import AddIcon from '@mui/icons-material/Add';
import { PERMISSION } from '../auth/permissions'
import {
  Button, Card, CardActionArea, CardContent, FormControl, InputLabel, MenuItem, Select, TextField, useMediaQuery
} from '@mui/material';
import { useTheme } from '@mui/material/styles';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import dayjs, { Dayjs } from 'dayjs';
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom';
import { useAppDispatch } from '../app/hooks';
import { setTid, setTournament } from '../breadcrumb';
import { useAuth } from '../hooks/useAuth';
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI';
import { makeCancellable } from '../features/makeCancellable';
import { states } from '../features/states';
import { TournamentCardContent } from '../components/TournamentCardContent';
import { TournamentCardCompact } from '../components/TournamentCardCompact';
import { TournamentEditorDialog } from '../components/TournamentEditorDialog';

/**
 * This renders a tournament finder and tournament editor and serves as the main component for the
 * home page.
 */
export const TournamentFinder = () => {
  const [startDate, setStartDate] = useState<Dayjs | null>(dayjs().subtract(1, 'month'))
  const [stopDate, setStopDate] = useState<Dayjs | null>(dayjs().add(1, 'month'))
  const [selectedCountry, setSelectedCountry] = useState<string>("USA")
  const [selectedRegion, setSelectedRegion] = useState<string>("")
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [tournaments, setTournaments] = useState<TournamentTS[]>([])
  const [tournamentEditor, setTournamentEditor] = useState<{ isOpen: boolean, tournament: TournamentTS | undefined }>({ isOpen: false, tournament: undefined });
  const { session } = useAuth();
  // Creating a tournament is also what makes someone a manager of tournaments, so the same
  // permission gates the create dialog and the "tournaments I manage" shortcut.
  const isTournamentCreate = session?.can(PERMISSION.TOURNAMENT_CREATE) ?? false;
  const isTournamentManager = isTournamentCreate;
  // Below `md`, as elsewhere in the mobile layout: full-width compact cards in a single column.
  // noSsr so the first render already knows, rather than flashing the desktop cards.
  const isMobile = useMediaQuery(useTheme().breakpoints.down('md'), { noSsr: true });
  // On mobile the two dates share one row: an equal split that may shrink below the fields' natural
  // width, in the smaller field size.
  const dateFieldProps = isMobile
    ? {
        size: "small" as const,
        sx: {
          flex: "1 1 0",
          minWidth: 0,
          // Tighter insets so a full date still fits beside the calendar icon on a 320px phone.
          "& .MuiInputBase-root": { pr: 1.25 },
          "& .MuiInputBase-input": { pl: 1.25, pr: 0 },
          "& .MuiInputAdornment-root": { ml: 0 },
        },
      }
    : { sx: { flexGrow: 1, flexShrink: 1 } };
  const dispatcher = useAppDispatch();
  const navigate = useNavigate();
  const openTournament = (tournament: TournamentTS) => {
    dispatcher(setTournament(tournament.tname));
    dispatcher(setTid(Number(tournament.tid)));
    // navigate("/division");  <- previously
    navigate(`/tournament/${tournament.tid}`);
  }
  const closeTournamentEditor = () => setTournamentEditor({ isOpen: false, tournament: undefined });
  const publicTournaments = tournaments.filter(t => t.is_public);
  useEffect(() => {
    setIsLoading(true)
    const startMillis = startDate ? startDate.valueOf() : 0;
    const stopMillis = stopDate ? stopDate.valueOf() : dayjs().add(1, 'month').valueOf();
    const cancellable = makeCancellable(TournamentAPI.getByDate(startMillis, stopMillis));
    cancellable.promise
      .then((tournaments: TournamentTS[]) => {
        setTournaments(tournaments)
        setIsLoading(false)
      })
      .catch((error) => {
        if (error.isCancelled) {
          console.log('Info: The request to get Tournaments was cancelled');
        } else {
          console.error('Error: Could not load Tournaments:', error);
        }
        setIsLoading(false);
      })
    return () => cancellable.cancel();
  }, [selectedCountry, selectedRegion, startDate, stopDate])

  return (
    <div style={{ display: "flex", justifyContent: "center" }}>
      <div
        style={{
          alignItems: "flex-start",
          display: "flex",
          flexDirection: "column",
          maxWidth: "100%",
          // The page container already provides the gutter on mobile.
          marginLeft: isMobile ? 0 : "20px",
          marginRight: isMobile ? 0 : "20px",
          width: 650
        }}
      >
        <h1 style={isMobile ? { fontSize: "1.5rem", margin: "4px 0 16px" } : undefined}>
          Find a Tournament
        </h1>
        <div style={{
          display: "flex",
          flexWrap: "wrap",
          gap: "10px",
          marginBottom: 8,
          width: "100%"
        }}>
          <LocalizationProvider dateAdapter={AdapterDayjs}>
            <DatePicker
              label={"From"}
              format="MM/DD/YYYY"
              value={startDate}
              onChange={setStartDate}
              enableAccessibleFieldDOMStructure={false}
              slots={{textField: TextField}}
              slotProps={{ textField: dateFieldProps }}
            />
            <DatePicker
              label={"To"}
              format="MM/DD/YYYY"
              value={stopDate}
              onChange={setStopDate}
              enableAccessibleFieldDOMStructure={false}
              slots={{ textField: TextField }}
              slotProps={{ textField: dateFieldProps }}
            />
          </LocalizationProvider>
        </div>
        {isTournamentManager && session?.userId && (
          <Button
            variant="contained"
            onClick={() => navigate(`/user/${session.userId}/managed-tournaments`)}
            sx={{ backgroundColor: '#8e24aa', '&:hover': { backgroundColor: '#7b1fa2' }, mb: 0, alignSelf: 'flex-start' }}
          >
            View Tournaments that I Manage
          </Button>
        )}
        <div style={{
          display: "flex",
          flexWrap: "wrap",
          gap: "10px",
          marginBottom: 20,
          width: "100%"
        }}>
          {/* <FormControl sx={{ flexGrow: 1, flexShrink: 1 }}>
            <InputLabel htmlFor="home-tournament-filter-country">
              Country
            </InputLabel>
            <Select
              id="home-tournament-filter-country"
              label="Country"
              onChange={event => setSelectedCountry(event.target.value)}
              sx={{ textAlign: "left" }}
              value={selectedCountry}
            >
              <MenuItem value={"USA"}>
                United States of America
              </MenuItem>
            </Select>
          </FormControl> */}
          {/* <FormControl sx={{ flexGrow: 1, flexShrink: 1 }}>
            <InputLabel htmlFor="home-tournament-filter-region">
              Region
            </InputLabel>
            <Select
              id="home-tournament-filter-region"
              label="Region"
              onChange={event => setSelectedRegion(event.target.value)}
              sx={{ textAlign: "left" }}
              value={selectedRegion}
            >
              {states.map(state => (
                <MenuItem key={state.abbreviation} value={state.abbreviation}>
                  {state.name}
                </MenuItem>
              ))}
            </Select>
          </FormControl> */}
        </div>
        <div style={{
          display: "flex",
          flexDirection: isMobile ? "column" : "row",
          // Wrapping a column would size each card to its unwrapped text rather than the screen.
          flexWrap: isMobile ? "nowrap" : "wrap",
          gap: 10,
          width: isMobile ? "100%" : undefined
        }}>
          {isTournamentCreate && isMobile && (
            <Button
              variant="outlined"
              startIcon={<AddIcon />}
              onClick={() => setTournamentEditor({ isOpen: true, tournament: undefined })}
              sx={{ py: 1 }}
            >
              Create a New Tournament
            </Button>
          )}
          {isTournamentCreate && !isMobile && (
            <Card onClick={() => setTournamentEditor({ isOpen: true, tournament: undefined })}>
              <CardContent sx={{
                alignItems: "center", display: "flex", flexDirection: "column", height: "100%",
                justifyContent: "center", cursor: "pointer", margin: "0 10px"
                }}>
                <div style={{
                  alignItems: "center",
                  background: "#e5e5e5",
                  borderRadius: 40,
                  display: "flex",
                  height: 70,
                  justifyContent: "center",
                  width: 70,
                  marginBottom: "10px"
                }}>
                  <AddIcon fontSize="medium" />
                </div>
                Create a New<br/>Tournament
              </CardContent>
            </Card>
          )}
          {isLoading ? (
            "Loading tournaments..."
          ) : publicTournaments.length < 1 ? (
            "No Tournaments found based on current filter criteria."
          ) : (
            publicTournaments.map(tournament => isMobile ? (
              <Card key={tournament.tid}>
                <CardActionArea onClick={() => openTournament(tournament)}>
                  <TournamentCardCompact tournament={tournament} />
                </CardActionArea>
              </Card>
            ) : (
              <Card key={tournament.tid}>
                <TournamentCardContent onClick={() => openTournament(tournament)} tournament={tournament} />
                {/* Make the user view the profile first before editing:
                {isTournamentCreate && (
                  <CardActions sx={{ justifyContent: "flex-end" }}>
                    <Button onClick={() => setTournamentEditor({ isOpen: true, tournament })} size="small">
                      Edit
                    </Button>
                  </CardActions>
                )} */}
              </Card>
            ))
          )}
        </div>
      </div>
      <TournamentEditorDialog
        initialTournament={tournamentEditor.tournament}
        isOpen={tournamentEditor.isOpen}
        onCancel={closeTournamentEditor}
        onSave={tournament => {
          closeTournamentEditor();
          if (tournamentEditor.tournament === undefined) {
            setTournaments(tournaments => tournaments.concat([tournament]));
          } else {
            setTournaments(state => {
              const index = state.findIndex(t => t.tid === tournament.tid);
              state[index] = tournament;
              return state;
            })
          }
        }}
      />
    </div>
  )
}
