import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import Card from "@mui/material/Card";
import { useLocation, Link } from "react-router-dom";

const NAV_TABS = [
  { label: "Divisions",    path: "divisions"    },
  { label: "Rooms",        path: "rooms"        },
  { label: "Teams",        path: "teams"        },
  { label: "Rounds",       path: "rounds"       },
  { label: "Quizzers",     path: "quizzers"     },
  { label: "Games",        path: "games"        },
  { label: "Admins",       path: "Admins"       },
  { label: "Stats Groups", path: "stats-groups" },
];

export default function TournamentTabBar({ tid }: { tid: string }) {
  const { pathname } = useLocation();
  const currentTab = NAV_TABS.findIndex((t) => pathname.includes(t.path));

  return (
    <Card variant="outlined">
      <Tabs
        value={currentTab === -1 ? false : currentTab}
        indicatorColor="primary"
        textColor="primary"
        variant="scrollable"
        scrollButtons="auto"
      >
        {NAV_TABS.map((tab) => (
          <Tab
            key={tab.path}
            label={tab.label}
            component={Link}
            to={`/tournament/${tid}/${tab.path}`}
          />
        ))}
      </Tabs>
    </Card>
  );
}