import React from 'react'
import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Breadcrumbs, Link } from '@mui/material'
import Tab from '@mui/material/Tab';
import Tabs from '@mui/material/Tabs'
import StickyHeadTable from './GamesTable'
import TournamentSettings from '../components/TournamentSettings'
import RoomPanel from '../components/RoomPanel'
import DivisionPanel from '../components/DivisionPanel'
import SchedulePanel from '../components/SchedulePanel'
import TeamPanel from '../components/TeamPanel'
import RoomMonitor from '../components/RoomMonitor'

export const DivisionAPI = {
  get: async (page: number, size: number) =>
    await (await fetch(`/api/divisions?page=${page}&page_size=${size}`)).json(),
  create: async (division: string) =>
    await (
      await fetch('/api/divisions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text: division }),
      })
    ).json(),
  delete: async (id: number) =>
    await fetch(`/api/divisions/${id}`, { method: 'DELETE' }),
  update: async (id: number, division: string) =>
    await fetch(`/api/divisions/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ text: division }),
    }),
}

// const Transition = React.forwardRef(function Transition(
//   props: TransitionProps & {
//     children: React.ReactElement;
//   },
//   ref: React.Ref<unknown>,
// ) {
//   return <Slide direction="up" ref={ref} {...props} />;
// });

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function a11yProps(index: number) {
  return {
    id: `simple-tab-${index}`,
    'aria-controls': `simple-tabpanel-${index}`,
  };
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`simple-tabpanel-${index}`}
      aria-labelledby={`simple-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          <Typography>{children}</Typography>
        </Box>
      )}
    </div>
  );
}

export const TDEditor = () => {
  // const [expanded, setExpanded] = useState(false)
  const [processing, setProcessing] = useState<boolean>(false)
  const [displayDate, setDisplayDate] = useState<Date>(new Date())
  const [divisions, setDivisions] = useState<Division[]>([])
  // const [openDivisionEditor, setDivisionEditorOpen] = useState(false);

  // const handleEditorClickOpen = () => {
  //   setDivisionEditorOpen(true);
  // };

  const navigate = useNavigate();

  useEffect(() => {
    setProcessing(true)
    DivisionAPI.get(0, 25).then((divisions: Division[]) => {
      setDivisions(divisions)
      setProcessing(false)
    })
    console.log("In useeffect - pulling from api")
  }, [displayDate])

  const [value, setValue] = React.useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <div>
      <Box>
        <br/>
        <Breadcrumbs aria-label="breadcrumb" >
          <Link underline="hover" color="inherit" href="/">
            Home
          </Link>
          {/* <Link underline="hover" color="inherit" href="/t/q2022">
            Q2022
          </Link>
          <Link
            underline="hover"
            color="inherit"
            href="/t/q2022/district%20novice"
          >
            District Novice
          </Link>*/}
          <Typography color="text.primary" onClick={() => navigate("/tdeditor")} >Tournaments: name_of_tournament</Typography>
        </Breadcrumbs>
      </Box>
      <br/>
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={value} onChange={handleChange} aria-label="basic tabs example">
          <Tab label="Tournament Settings" {...a11yProps(0)} />
          <Tab label="Divisions" {...a11yProps(1)} />
          <Tab label="Rooms" {...a11yProps(2)} />
          <Tab label="Teams" {...a11yProps(3)} />
          <Tab label="Schedule" {...a11yProps(4)} />
          <Tab label="Games/Quizzes" {...a11yProps(5)} />
          <Tab label="Room Monitor" {...a11yProps(6)} />
        </Tabs>
      </Box>
      <TabPanel value={value} index={0}>
        {TournamentSettings()}
      </TabPanel>
      <TabPanel value={value} index={1}>
        {DivisionPanel()}
      </TabPanel>
      <TabPanel value={value} index={2}>
        {RoomPanel()}
      </TabPanel>
      <TabPanel value={value} index={3}>
        {TeamPanel()}
      </TabPanel>
      <TabPanel value={value} index={4}>
        {SchedulePanel()}
      </TabPanel>
      <TabPanel value={value} index={5}>
        {StickyHeadTable()}
      </TabPanel>
      <TabPanel value={value} index={6}>
        {RoomMonitor()}
      </TabPanel>
    </div >
  )
}
