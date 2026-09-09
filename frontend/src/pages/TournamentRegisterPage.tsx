import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Alert from '@mui/material/Alert'
import { TournamentGearRegistrationPanel } from '../components/TournamentGearRegistrationPanel'
import { TournamentTeamRegistrationPanel } from '../components/TournamentTeamRegistrationPanel'
import { TournamentVolunteerPanel } from '../components/TournamentVolunteerPanel'

type Tab = 'team' | 'gear' | 'volunteer'

export const TournamentRegisterPage = ({
  tid,
  tname,
  initialTab = 'team',
  useTeamRegistration = true,
  useGearRegistration = true,
  useVolunteerRegistration = true,
}: {
  tid: string;
  tname: string;
  initialTab?: 'team' | 'gear' | 'as-volunteer';
  useTeamRegistration?: boolean;
  useGearRegistration?: boolean;
  useVolunteerRegistration?: boolean;
}) => {
  const navigate = useNavigate()

  const [activeTab] = useState<Tab>(
    initialTab === 'gear' ? 'gear' : initialTab === 'as-volunteer' ? 'volunteer' : 'team'
  )

  const tabEnabled: Record<Tab, boolean> = {
    team: useTeamRegistration,
    gear: useGearRegistration,
    volunteer: useVolunteerRegistration,
  }

  // If the current tab's type isn't offered (e.g. direct navigation to a disabled type), show a
  // notice instead of its panel.
  const activeTabEnabled = tabEnabled[activeTab]

  return (
    <Box>

      {/* ── Tab Buttons ── only the registration types this tournament offers are shown ── */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1, mb: 3 }}>
        {useTeamRegistration && (
          <Button
            variant={activeTab === 'team' ? 'contained' : 'outlined'}
            onClick={() => navigate(`/tournament/${tid}/register/team`)}
          >
            Team
          </Button>
        )}
        {useGearRegistration && (
          <Button
            variant={activeTab === 'gear' ? 'contained' : 'outlined'}
            onClick={() => navigate(`/tournament/${tid}/register/gear`)}
          >
            Gear
          </Button>
        )}
        {useVolunteerRegistration && (
          <Button
            variant={activeTab === 'volunteer' ? 'contained' : 'outlined'}
            onClick={() => navigate(`/tournament/${tid}/register/as-volunteer`)}
          >
            As Volunteer
          </Button>
        )}
      </Box>

      {!activeTabEnabled ? (
        <Alert severity="info">This registration type is not available for this tournament.</Alert>
      ) : (
        <>
          {/* ── Team Tab ── */}
          {activeTab === 'team' && (
            <TournamentTeamRegistrationPanel tid={tid} />
          )}

          {/* ── Gear Tab ── */}
          {activeTab === 'gear' && (
            <TournamentGearRegistrationPanel tid={tid} />
          )}

          {/* ── Volunteer Tab ── */}
          {activeTab === 'volunteer' && (
            <TournamentVolunteerPanel tid={tid} tname={tname} />
          )}
        </>
      )}

    </Box>
  )
}
