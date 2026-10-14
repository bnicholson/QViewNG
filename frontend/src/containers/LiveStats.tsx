import { useCallback } from 'react'
import { useParams, useSearchParams } from 'react-router-dom'
import Box from '@mui/material/Box'
import Tab from '@mui/material/Tab'
import Tabs from '@mui/material/Tabs'
import { GamesInProgress } from './GamesInProgress'
import TeamStatsLive from '../components/TeamStatsLive'

type LiveView = 'games' | 'teams'

/**
 * The tournament's LIVE Stats page: Games in Progress and each division's Team Stats, both
 * pushed live by the server.
 *
 * The tab and the division live in the URL (`?view=teams&division=<did>`), so a spectator screen
 * or a shared link opens straight onto one view.
 */
export const LiveStats = () => {
  const { tid } = useParams()
  const [searchParams, setSearchParams] = useSearchParams()
  const view: LiveView = searchParams.get('view') === 'teams' ? 'teams' : 'games'
  const divisionId = searchParams.get('division')

  const setView = (next: LiveView) => {
    setSearchParams((params) => {
      if (next === 'games') params.delete('view')
      else params.set('view', next)
      return params
    })
  }

  // Replaces rather than pushes: picking the default division is not a navigation step to undo.
  const setDivision = useCallback((did: string) => {
    setSearchParams((params) => {
      params.set('division', did)
      return params
    }, { replace: true })
  }, [setSearchParams])

  // Mounted only under /tournament/:tid/live-stats, so a tid is always present.
  if (!tid) return null

  return (
    <div>
      <Box sx={{ borderBottom: 1, borderColor: 'divider', mb: 2 }}>
        <Tabs value={view} onChange={(_, next: LiveView) => setView(next)} aria-label="LIVE Stats views">
          <Tab label="Games in Progress" value="games" />
          <Tab label="Team Stats" value="teams" />
        </Tabs>
      </Box>
      {view === 'games'
        ? <GamesInProgress />
        : <TeamStatsLive tid={tid} divisionId={divisionId} onDivisionChange={setDivision} />}
    </div>
  )
}
