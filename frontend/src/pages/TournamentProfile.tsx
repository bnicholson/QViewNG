import React, { useState } from 'react'
import { useEffect } from 'react'
import { Navigate, useNavigate, useParams } from 'react-router'
import Card from "@mui/material/Card"
import CardHeader from '@mui/material/CardHeader'
import CardContent from "@mui/material/CardContent"
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import IconButton from '@mui/material/IconButton'
import { Breadcrumbs } from '@mui/material'
// import { Breadcrumbs, Link } from '@mui/material'
import { Link } from 'react-router-dom'
import SettingsIcon from '@mui/icons-material/Settings'
import AppBar from '@mui/material/AppBar'
import Dialog from '@mui/material/Dialog'
import Toolbar from '@mui/material/Toolbar'
import CloseIcon from '@mui/icons-material/Close'
import Slide from '@mui/material/Slide'
import { type TransitionProps } from '@mui/material/transitions'
import Button from '@mui/material/Button';
import ListItemText from '@mui/material/ListItemText';
import ListItem from '@mui/material/ListItem';
import List from '@mui/material/List';
import Divider from '@mui/material/Divider';
import Fab from '@mui/material/Fab'
import AddIcon from "@mui/icons-material/Add"
// import { selectDisplayDate, selectTournament, setDisplayDate, setTournament, toggleIsOn, setTid } from '../breadcrumb'
// import { useAppSelector } from '../app/hooks';
import Tooltip from '@mui/material/Tooltip';
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { makeCancellable } from '../features/makeCancellable'
import { DivisionAPI } from '../features/DivisionAPI'
// import {createRoot} from 'react-dom/client'
// import Markdown from 'react-markdown'
// import remarkGfm from 'remark-gfm'
// import MDEditor from '@uiw/react-md-editor';

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & {
    children: React.ReactElement;
  },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

const DIVISIONS_PAGE = 0
const DIVISIONS_PAGE_SIZE = 30

export const TournamentProfile = (props: { tab: string }) => {

  const { tid } = useParams();
  if (tid === undefined) return (<></>)

  const navigate = useNavigate();

  const validTabs = ['divisions', 'rooms', 'teams', 'rounds', 'quizzers', 'games', 'admins', 'stats-groups'];
  if (props.tab === undefined) { props.tab = validTabs[0] }

  if (!validTabs.includes(props.tab)) {
    return <Navigate to={`/tournament/${tid}`} replace />;
  }

  // const [expanded, setExpanded] = useState(false)
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const stillLoading = () => isLoading || tournament == null || tournament == undefined
  // const tournament = useAppSelector((state) => state.breadCrumb.tournament);
  const [tournament, setTournament] = useState<TournamentTS>()
  const [divisions, setDivisions] = useState<Division[]>([])
  const [openDivisionEditor, setDivisionEditorOpen] = useState(false);

  // let displayDate = useAppSelector((state) => state.breadCrumb.displayDate);
  // const tid = useAppSelector((state) => state.breadCrumb.tid)
  // const division = useAppSelector((state) => state.breadCrumb.division);
  // const did = useAppSelector((state) => state.breadCrumb.did);
  const handleEditorClickOpen = () => {
    setDivisionEditorOpen(true);
  };

  useEffect(() => {
    setIsLoading(true)
    // const startMillis = startDate ? startDate.valueOf() : 0;
    // const stopMillis = stopDate ? stopDate.valueOf() : dayjs().add(1, 'month').valueOf();
    // const cancellable = makeCancellable(TournamentAPI.get(0, 10));  // for temporary testing
    const cancellable = makeCancellable(TournamentAPI.getById(tid));
    cancellable.promise
      .then((returnedTournament: TournamentTS) => {
        setTournament(returnedTournament)
        // setIsLoading(false)
      })
      .catch((error) => {
        if (error.isCancelled) {
          console.log('Info: The request to get Tournaments was cancelled');
        } else {
          console.error('Error: Could not load Tournaments:', error);
        }
        setIsLoading(false);
        cancellable.cancel();
      })
    // return () => cancellable.cancel();
      
    // setIsLoading(true)
    DivisionAPI.get(DIVISIONS_PAGE, DIVISIONS_PAGE_SIZE).then((divisions: Division[]) => {
      setDivisions(divisions)
      // setIsLoading(false)
    })
    console.log("In useeffect - pulling from api")

    // make sure we have a valid division and tid
    // TODO -> redirect to 404 Not Found page (*to be added soon)

    setIsLoading(false)
  }, [tid])

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
      {stillLoading() && 
        <div>Loading Tournament...</div>
      }
      {!stillLoading() &&
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
            {/* <Typography>Tournament ID: {tournament} {tid} </Typography> */}
            <br/>
            <Box sx={{ border: 1 }} style={{ textAlign: 'left', borderRadius: '20px', padding: '12px'}}>
              <div>*Project Planning Note: The breadcrumb path above could alternatively be a list like the following
                (with links to each level for quick access):</div>
              <ul>
                <li><Link color="inherit" to={`/tournament/${tid}`}>Tournament: {tournament?.tname}</Link></li>
                <li>
                  Division: division_name
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                  // this would be visible to children entities of the Division entity.</li>
                <li>
                  Round: round_number
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                  // this would be visible to children entities of the Round entity, namely Games.</li>
                <li>
                  Room: room_number/name
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                  // this would be visible to children entities of the Room entity, namely Games.</li>
              </ul>
            </Box>
          </Box>
          <div className="Form">
            <Card>
              {/* <CardHeader></CardHeader> */}
              <Box sx={{ display: 'flex' }}>
                <CardContent>
                  <Typography align="left" variant="h5" color="primary" >
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/divisions`}
                    >
                      Divisions
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/rooms`}
                    >
                      Rooms
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/teams`}
                    >
                      Teams
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/rounds`}
                    >
                      Rounds
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/quizzers`}
                    >
                      Quizzers
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/games`}
                    >
                      Games
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/Admins`}
                    >
                      Admins
                    </Link>
                    &nbsp;&nbsp;&nbsp;&nbsp;
                    <Link
                      color="inherit"
                      to={`/tournament/${tid}/stats-groups`}
                    >
                      Stats Groups
                    </Link>
                  </Typography>
                </CardContent>
              </Box>
            </Card>
            {/* {divisions.map((division) => */}
              {/* <Card key={division.dname}> */}
              <Card>
                {/* <CardHeader
                  action={
                    <Tooltip title="Edit this division" arrow>
                      <IconButton onClick={() => handleEditorClickOpen()} aria-label="settings">
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
                <Box sx={{ display: 'flex' }}>
                  <CardContent>
                    {props.tab === 'divisions' && <div>***where the DIVISIONS data grid will go***</div>}
                    {props.tab === 'rooms' && <div>***where the ROOMS data grid will go***</div>}
                    {props.tab === 'teams' && <div>***where the TEAMS data grid will go***</div>}
                    {props.tab === 'rounds' && <div>***where the ROUNDS data grid will go***</div>}
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
            {divisions.map((division) =>
              <Card key={division.dname}>
                <CardHeader
                  action={
                    <Tooltip title="Edit this division" arrow>
                      <IconButton onClick={() => handleEditorClickOpen()} aria-label="settings">
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
                  </Typography> */}
                  </CardContent>
                </Box>
              </Card>
            )}
            {DivisionEditor(openDivisionEditor, setDivisionEditorOpen)}
          </div>
          <br/>
          <Fab color="primary" onClick={() => handleEditorClickOpen()} aria-label="Add Tournament">
            <AddIcon />
          </Fab>
        </div >
      }
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
const DivisionEditor = (openDivisionEditor: boolean, setDivisionEditorOpen: React.Dispatch<React.SetStateAction<boolean>>) => {
  const handleDivisionEditorClose = () => {
    setDivisionEditorOpen(false);
  };

  return (
    <Dialog
      fullScreen
      open={openDivisionEditor}
      onClose={handleDivisionEditorClose}
      TransitionComponent={Transition}
    >
      <AppBar sx={{ position: 'relative' }}>
        <Toolbar>
          <IconButton
            edge="start"
            color="inherit"
            onClick={handleDivisionEditorClose}
            aria-label="close"
          >
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            Division Settings
          </Typography>
          <Button autoFocus color="inherit" onClick={handleDivisionEditorClose}>
            save
          </Button>
        </Toolbar>
      </AppBar>
      <Box sx={{ width: '100%' }}>
        <List>
          <ListItem>
            {/* <ListItem button> */}
            <ListItemText primary="Phone ringtone" secondary="Titania" />
          </ListItem>
          <Divider />
          <ListItem>
            {/* <ListItem button> */}
            <ListItemText
              primary="Default notification ringtone"
              secondary="Tethys"
            />
          </ListItem>
        </List>
      </Box>
    </Dialog>
  )
}