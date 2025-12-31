import AddIcon from '@mui/icons-material/Add';
import { 
  Button, Card, CardActions, CardContent, FormControl, InputLabel, MenuItem, Select, TextField
} from '@mui/material';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import dayjs, { Dayjs } from 'dayjs';
import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom';
import { useAppDispatch } from '../app/hooks';
import { setTid, setTournament } from '../breadcrumb';
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI';
import { makeCancellable } from '../features/makeCancellable';
import { states } from '../features/states';
import { TournamentCardContent } from '../features/TournamentCardContent';
import { TournamentEditorDialog } from '../features/TournamentEditorDialog';

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
  const isUserAdmin = true;
  const dispatcher = useAppDispatch();
  const navigate = useNavigate();
  const openTournament = (tournament: TournamentTS) => {
    dispatcher(setTournament(tournament.tname));
    dispatcher(setTid(tournament.tid));
    // navigate("/division");  <- previously
    navigate(`/tournament/${tournament.tid}/divisions`);
  }
  const closeTournamentEditor = () => setTournamentEditor({ isOpen: false, tournament: undefined });
  React.useEffect(() => {
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
          marginLeft: "20px",
          marginRight: "20px",
          width: 650
        }}
      >
        <h1>
          Find a Tournament
        </h1>
        <div style={{
          display: "flex",
          flexWrap: "wrap",
          gap: "10px",
          marginBottom: 20,
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
              slotProps={{
                textField: { 
                  sx: { flexGrow: 1, flexShrink: 1 }
                }
              }}
            />
            <DatePicker
              label={"To"}
              format="MM/DD/YYYY"
              value={stopDate}
              onChange={setStopDate}
              enableAccessibleFieldDOMStructure={false}
              slots={{ textField: TextField }}
              slotProps={{
                textField:{
                  sx:{ flexGrow: 1, flexShrink: 1 }
                }
              }}
            />
          </LocalizationProvider>
        </div>
        <div style={{
          display: "flex",
          flexWrap: "wrap",
          gap: "10px",
          marginBottom: 20,
          width: "100%"
        }}>
          <FormControl sx={{ flexGrow: 1, flexShrink: 1 }}>
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
          </FormControl>
          <FormControl sx={{ flexGrow: 1, flexShrink: 1 }}>
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
          </FormControl>
        </div>
        <div style={{
          display: "flex",
          flexWrap: "wrap",
          gap: 10
        }}>
          {isUserAdmin && (
            <Card onClick={() => setTournamentEditor({ isOpen: true, tournament: undefined })}>
              <CardContent sx={{ 
                alignItems: "center", display: "flex", flexDirection: "column", height: "100%", 
                justifyContent: "center", cursor: "pointer"
                }}>
                <div style={{
                  alignItems: "center",
                  background: "#e5e5e5",
                  borderRadius: 40,
                  display: "flex",
                  height: 80,
                  justifyContent: "center",
                  width: 80
                }}>
                  <AddIcon fontSize="large" />
                </div>
                Create a New Tournament
              </CardContent>
            </Card>
          )}
          {isLoading ? (
            "Loading tournaments..."
          ) : tournaments.length < 1 ? (
            "No Tournaments found based on current filter criteria."
          ) : (
            tournaments.map(tournament => (
              <Card key={tournament.tid}>
                <TournamentCardContent onClick={() => openTournament(tournament)} tournament={tournament} />
                {isUserAdmin && (
                  <CardActions sx={{ justifyContent: "flex-end" }}>
                    <Button onClick={() => setTournamentEditor({ isOpen: true, tournament })} size="small">
                      Edit
                    </Button>
                  </CardActions>
                )}
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
