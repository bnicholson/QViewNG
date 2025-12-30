import { ChevronLeft, Inbox, Mail } from "@mui/icons-material";
import { 
    Divider, Drawer, IconButton, List, ListItem, ListItemButton, ListItemIcon, ListItemText, styled
} from "@mui/material";
import { useNavigate } from "react-router-dom";

// const DrawerHeader = styled('div')(({ theme }) => ({  <- previous version
const DrawerHeader = styled('div')(() => ({
    display: 'flex',
    alignItems: 'center',
    //padding: theme.spacing(0, 1),
    // necessary for content to be below app bar
    //    ...theme.mixins.toolbar,
    justifyContent: 'flex-end',
}));

export default function DrawerGlobal(props: {
    drawerWidth: number,
    drawerIsOpen: boolean,
    toggleDrawer: () => void
}) {
    const navigate = useNavigate();

    return (
        <Drawer
            sx={{
              width: props.drawerWidth,
              flexShrink: 0,
              '& .MuiDrawer-paper': {
                width: props.drawerWidth,
                boxSizing: 'border-box',
              },
            }}
            variant="persistent"
            anchor="left"
            open={props.drawerIsOpen}
          >
            <DrawerHeader>
              <IconButton onClick={props.toggleDrawer}>
                <ChevronLeft />
              </IconButton>
            </DrawerHeader>
            <Divider />
            <List>
              {['Tournament', 'Division', 'Room', 'Round'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 4) {
                      case 0:
                        navigate("/tournament");
                        break;
                      case 1:
                        navigate("/division");
                        break;
                      case 2:
                        alert("room");
                        break;
                      case 3:
                        alert('round');
                    }
                  }}>
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
            <Divider />
            <List>
              {['Quizzes', 'Team', 'Individual'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 3) {
                      case 0:
                        alert("quizzes");
                        break;
                      case 1:
                        alert("team");
                        break;
                      case 2:
                        alert("individual");
                        break;
                    }
                  }}>
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
            <Divider />
            <List>
              {['Files'].map((text, index) => (
                <ListItem key={text} disablePadding
                  onClick={() => {
                    switch (index % 3) {
                      case 0:
                        navigate("/files");
                        break;
                      // case 1:
                      //   navigate("/todos");
                      //   break;
                      // case 2:
                      //   navigate("/gql");
                      //   break;
                    }
                  }}
                >
                  <ListItemButton>
                    <ListItemIcon>
                      {index % 2 === 0 ? <Inbox /> : <Mail />}
                    </ListItemIcon>
                    <ListItemText primary={text} />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
          </Drawer>
    )
}
