import { useState, useEffect } from 'react'
import { Navigate, useParams } from 'react-router'
import Card from "@mui/material/Card"
// import CardHeader from '@mui/material/CardHeader'
import CardContent from "@mui/material/CardContent"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Breadcrumbs } from '@mui/material'
// import { Breadcrumbs, Link } from '@mui/material'
import { Link } from 'react-router-dom'
// import SettingsIcon from '@mui/icons-material/Settings'
import Button from '@mui/material/Button';
// import { selectDisplayDate, selectTournament, setDisplayDate, setTournament, toggleIsOn, setTid } from '../breadcrumb'
// import { useAppSelector } from '../app/hooks';
// import Tooltip from '@mui/material/Tooltip';
// import { TournamentAPI, type Tournament } from '../features/TournamentAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { makeCancellable } from '../features/makeCancellable'
import DivisionsTable from '../components/DivisionsTable'
import TournamentTabBar from '../components/TournamentTabBar'
import RoomsTable from '../components/RoomsTable'
import RoundsTable from '../components/RoundsTable'
import { TournamentEditorDialog } from '../features/TournamentEditorDialog'
// import {createRoot} from 'react-dom/client'
// import Markdown from 'react-markdown'
// import remarkGfm from 'remark-gfm'
// import MDEditor from '@uiw/react-md-editor';

export const TournamentProfile = (props: { tab: string }) => {

  const isUserAdmin = true;

  // const tid = useAppSelector((state) => state.breadCrumb.tid)
  const { tid } = useParams();
  if (tid === undefined) return (<></>)

  const validTabs = ['divisions', 'rooms', 'teams', 'rounds', 'quizzers', 'games', 'admins', 'stats-groups'];
  if (props.tab === undefined) { props.tab = validTabs[0] }

  if (!validTabs.includes(props.tab)) {
    return <Navigate to={`/tournament/${tid}`} replace />;
  }

  // const [expanded, setExpanded] = useState(false)
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const stillLoading = () => isLoading || tournament == null || tournament == undefined
  const [notFound, setNotFound] = useState<boolean>(false)
  // const tournament = useAppSelector((state) => state.breadCrumb.tournament);
  const [tournament, setTournament] = useState<TournamentTS>()
  // const [divisionEditorIsOpen, setDivisionEditorIsOpen] = useState(false);
  const [tournamentEditorIsOpen, setTournamentEditorIsOpen] = useState(false);

  // let displayDate = useAppSelector((state) => state.breadCrumb.displayDate);
  // const division = useAppSelector((state) => state.breadCrumb.division);
  // const did = useAppSelector((state) => state.breadCrumb.did);

  useEffect(() => {
    setIsLoading(true)
    const cancellable = makeCancellable(TournamentAPI.getById(tid));
    try {
      cancellable.promise
        .then((returnedTournament: TournamentTS) => {
          setTournament(returnedTournament)
          setIsLoading(false)
        })
        .catch((error) => {
          if (error.isCancelled) {
            console.log('Info: The request to get Tournament by ID was cancelled');
          } else {
            console.error('Error: Could not load Tournament:', error);
          }
          setIsLoading(false);
          setNotFound(true)
        })
    } catch (err: any) {
      if (err instanceof Error) {
        console.error(err.message)
        setIsLoading(false);
        setNotFound(true)
      }
    }
    console.log("In useeffect - pulling from api")

    setIsLoading(false)
  }, [tid])

  if (notFound) return <Navigate to="/404" replace />
  if (stillLoading()) return <div>Loading Tournament...</div>

  return (
    <div>
      {/* <div>
        <Markdown remarkPlugins={[remarkGfm]}>{valuemd}</Markdown>
      <MDEditor
        value={valuemd}
        onChange={setValue}
      />
      <MDEditor.Markdown source={valuemd} style={{ whiteSpace: 'pre-wrap' }} />
      </div> */}
      <br/>
      <div>
        <Box>
          <Breadcrumbs aria-label="breadcrumb" >
            <Link color="inherit" to="/">
              Home
            </Link>
            {/* <Link color="inherit" to="/t/q2022">
              Q2022
            </Link>
            <Link
              
              color="inherit"
              href="/t/q2022/district%20novice"
            >
              District Novice
            </Link>&nbsp;&nbsp;
            <Link href="/tdeditor">
              <Typography color="text.primary" >Teams</Typography>
            </Link>
            <Link href="/roundsinprogress">
              <Typography color="text.primary" >Rounds</Typography>
            </Link> */}
            <Link color="inherit" to={`/tournament/${tid}`}>
              <Typography color="text.primary" >{tournament?.tname} (tournament)</Typography>
            </Link>
          </Breadcrumbs>
        </Box> 
        { tournament != undefined && 
          <Box style={{ textAlign: "left" }}>
            <br/>
            <div style={{ flex: 1 }}>
              <h1>
                {tournament.tname}
                &nbsp;&nbsp;
                {isUserAdmin && (
                  <Button onClick={() => setTournamentEditorIsOpen(true)}>
                    Edit
                  </Button>
                )}
              </h1>
              <h4>
                General Info:
              </h4>
              <div>
                ID: {tournament.tid}
              </div><div>
              </div><div>
                Org: {tournament.organization}
              </div><div>
                At: {tournament.venue}, {tournament.city}, {tournament.region}, {tournament.country}
              </div><div>
                Contacts: {tournament.contact}
              </div><div>
                Contact Email: {tournament.contactemail}
              </div><div>
              </div><div>
                IsPublic: {tournament.is_public.toString()}
              </div><div>
                Short Info: {tournament.shortinfo}
              </div><div>
                More Info: {tournament.info}
              </div>
            </div>
          </Box>
        }
        <div className="Form">
          {/* {divisions.map((division) => */}
          {/* <Card key={division.dname}> */}
          <Card>
              {/* <CardHeader
                action={
                  <Tooltip title="Edit this division" arrow>
                    <IconButton onClick={() => openTournamentEditor()} aria-label="settings">
                      <SettingsIcon />
                    </IconButton>
                  </Tooltip>
                }
                title={<Typography variant="h5">
                  <Link
                    
                    color="primary"
                    href="#">{division.dname}</Link>
                </Typography>}
                subheader={<Typography variant="h6"> Need to put something here for now nothing. </Typography>}
              /> */}
              <TournamentTabBar tid={String(tournament?.tid)}/>
              <Box sx={{ display: 'flex' }}>
                <CardContent>
                  {/* {props.tab === 'divisions' && <div>***where the DIVISIONS data grid will go***</div>} */}
                  {props.tab === 'divisions' && <DivisionsTable tid={String(tournament?.tid)}/>}
                  {props.tab === 'rooms' && <RoomsTable tid={String(tournament?.tid)}/>}
                  {props.tab === 'teams' && <div>***where the TEAMS data grid will go***</div>}
                  {props.tab === 'rounds' && <RoundsTable tid={String(tournament?.tid)}/>}
                  {props.tab === 'quizzers' && <div>***where the QUIZZERS data grid will go***</div>}
                  {props.tab === 'games' && <div>***where the GAMES data grid will go***</div>}
                  {props.tab === 'admins' && <div>***where the ADMINS data grid will go***</div>}
                  {props.tab === 'stats-groups' && <div>***where the STATS GROUPS data grid will go***</div>}
                  
                  {/* <Typography align="left" variant="h5" color="primary" >
                    <Link
                      
                      color="inherit"
                      href="/t/q2022/district%20novice"
                    >
                      Team Standings
                    </Link>&nbsp;&nbsp;
                    <Link
                      
                      color="inherit"
                      href="/t/q2022/district%20novice"
                    >
                      Individual Standings
                    </Link>
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    Breadcrumb: {division.breadcrumb}
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    ShortInfo: {division.shortinfo}
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    ID: {division.did}                   Hidden: {division.hide}
                  </Typography>
                  {/* <Typography align="left" variant="body1" color="text.primary" >
                  Created: {division.created_at} - Last Update: {division.updated_at}
                </Typography> */}
                </CardContent> 
              </Box>
            </Card>
          {/* )} */}
        </div>
        <div className="Form">
          <Card>
            {/* <CardHeader>

            </CardHeader> */}
            <Box sx={{ display: 'flex' }}>
              <CardContent>
                <Typography align="left" variant="h5" color="primary" >
                  <Link
                    color="inherit"
                    to="/roundsinprogress"
                  >
                    Rounds In Progress
                  </Link>
                  &nbsp;&nbsp;&nbsp;&nbsp;
                  <Link
                    color="inherit"
                    to="/tdeditor"
                  >
                    Tournament Editor
                  </Link>
                  &nbsp;&nbsp;&nbsp;&nbsp;
                  < a href="http://localhost:3000/swagger-ui/">Swagger UI</a>
                </Typography>
              </CardContent>
            </Box>
          </Card>
          {/* {divisions.map((division) =>
            <Card key={division.dname}>
              <CardHeader
                action={
                  <Tooltip title="Edit this division" arrow>
                    <IconButton onClick={() => openTournamentEditor()} aria-label="settings">
                      <SettingsIcon />
                    </IconButton>
                  </Tooltip>
                }
                title={<Typography variant="h5">
                  <Link
                    color="primary"
                    to="#">{division.dname}</Link>
                </Typography>}
                subheader={<Typography variant="h6">***the Division's GENERAL INFO goes here***</Typography>}
              />
              <Box sx={{ display: 'flex' }}>
                <CardContent>
                  <Typography align="left" variant="h5" color="primary" >
                    <Link
                      color="inherit"
                      to="/t/q2022/district%20novice"
                    >
                      Team Standings
                    </Link>&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to="/t/q2022/district%20novice"
                    >
                      Individual Standings
                    </Link>
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    Breadcrumb: {division.breadcrumb}
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    ShortInfo: {division.shortinfo}
                  </Typography>
                  <Typography align="left" variant="body1" color="text.primary" >
                    ID: {division.did}                   Hidden: {division.hide}
                  </Typography>
                  {/* <Typography align="left" variant="body1" color="text.primary" >
                  Created: {division.created_at} - Last Update: {division.updated_at}
                </Typography> 
                </CardContent>
              </Box>
            </Card>
          )} */}
        </div>
        {/* <br/>
        <Fab color="primary" onClick={() => setTournamentEditorIsOpen(true)} aria-label="Add Tournament">
          <AddIcon />
        </Fab> */}
      </div >
      <TournamentEditorDialog
        initialTournament={tournament}
        isOpen={tournamentEditorIsOpen}
        onCancel={() => setTournamentEditorIsOpen(false)}
        onSave={tournament => {
          setTournament(tournament);
          setTournamentEditorIsOpen(false);
        }}
      />
    </div>
  )
}

// const test = () => {
//   return (
//     <ListItem>
//     {/* <ListItem button> */}
//       <ListItemText
//         primary="Snake button"
//         secondary="Tethys"
//       />
//     </ListItem>
//   )
// }