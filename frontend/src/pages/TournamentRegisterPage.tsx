import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import { TournamentGearRegistrationPanel } from '../components/TournamentGearRegistrationPanel'
import { TournamentTeamRegistrationPanel } from '../components/TournamentTeamRegistrationPanel'
import { TournamentVolunteerPanel } from '../components/TournamentVolunteerPanel'

type Tab = 'team' | 'gear' | 'volunteer'

export const TournamentRegisterPage = ({ tid, tname, initialTab = 'team' }: { tid: string; tname: string; initialTab?: 'team' | 'gear' | 'as-volunteer' }) => {
  const navigate = useNavigate()

  const [activeTab] = useState<Tab>(
    initialTab === 'gear' ? 'gear' : initialTab === 'as-volunteer' ? 'volunteer' : 'team'
  )

  return (
    <Box>

      {/* ── Tab Buttons ── */}
      <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1, mb: 3 }}>
        <Button
          variant={activeTab === 'team' ? 'contained' : 'outlined'}
          onClick={() => navigate(`/tournament/${tid}/register/team`)}
        >
          Team
        </Button>
        <Button
          variant={activeTab === 'gear' ? 'contained' : 'outlined'}
          onClick={() => navigate(`/tournament/${tid}/register/gear`)}
        >
          Gear
        </Button>
        <Button
          variant={activeTab === 'volunteer' ? 'contained' : 'outlined'}
          onClick={() => navigate(`/tournament/${tid}/register/as-volunteer`)}
        >
          As Volunteer
        </Button>
      </Box>

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

    </Box>
  )
}
