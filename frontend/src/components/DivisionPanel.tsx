import Fab from '@mui/material/Fab'
import AddIcon from "@mui/icons-material/Add"
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import DeleteIcon from '@mui/icons-material/Delete';
import UpdateIcon from '@mui/icons-material/Update';
import { Tooltip } from '@mui/material';

function createData(
    name: string,
    breadcrumb: string,
    hide: string,
    shortinfo: string,
) {
    return { name, breadcrumb, hide, shortinfo };
}

const rows = [
    createData("Local-Experienced", "lx", "Y", "Teams that are in a local church and not novice"),
];

function handleRoomAdd() {
    //  rows.splice(rows.length, 0, { "456" : string, "Big room" : string, "Paul Baker" : string , "Michael Sherman" :string } );
}

export default function RoomPanel() {
    return (
        <div>
            <Tooltip title="Add a new division" arrow>
                <Fab color="primary" onClick={() => handleRoomAdd()} aria-label="Add Room">
                    <AddIcon />
                </Fab>
            </Tooltip>
            <TableContainer component={Paper}>
                <Table sx={{ minWidth: 650 }} aria-label="caption table">
                    <TableHead>
                        <TableRow>
                            <TableCell> </TableCell>
                            <TableCell align="right">Name</TableCell>
                            <TableCell align="right">Breadcrumb</TableCell>
                            <TableCell align="right">Hidden</TableCell>
                            <TableCell align="right">Short Info</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {rows.map((row) => (
                            <TableRow key={row.name}>
                                <TableCell component="th" scope="row">
                                <Tooltip title="Delete this division " arrow>
                                        <DeleteIcon />
                                    </Tooltip>
                                    <Tooltip title="Update this division" arrow>
                                        <UpdateIcon />
                                    </Tooltip>
                                </TableCell>
                                <TableCell align="right">{row.name}</TableCell>
                                <TableCell align="right">{row.breadcrumb}</TableCell>
                                <TableCell align="right">{row.hide}</TableCell>
                                <TableCell align="left">{row.shortinfo}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </TableContainer>
        </div>
    )
}