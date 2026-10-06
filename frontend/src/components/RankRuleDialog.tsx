import Button from '@mui/material/Button'
import Dialog from '@mui/material/Dialog'
import DialogActions from '@mui/material/DialogActions'
import DialogContent from '@mui/material/DialogContent'
import DialogTitle from '@mui/material/DialogTitle'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import type { TeamStatTS, RankRuleExplanationTS } from '../features/StatsGroupAPI'

interface Props {
  /** The team whose placement is being explained; null closes the dialog. */
  team: TeamStatTS | null
  onClose: () => void
}

function Explanation({ heading, rule, team }: { heading: string; rule: RankRuleExplanationTS; team: string }) {
  return (
    <div>
      <Typography variant="subtitle2" sx={{ fontWeight: 700 }}>{heading}</Typography>
      <Typography variant="body2">
        Rule {rule.number} — <strong>{rule.label}</strong>: {team} {rule.detail.split(' vs ')[0]},{' '}
        {rule.other_team} {rule.detail.split(' vs ')[1]}.
      </Typography>
    </div>
  )
}

/**
 * Explains a team's position in the standings: which tie-breaking criterion separated it from the
 * team above it, and which separated it from the team below. Rule numbers match the order in the
 * Tie-breaking Rules dialog.
 */
export default function RankRuleDialog({ team, onClose }: Props) {
  const above = team?.rank_rule_above ?? null
  const below = team?.rank_rule_below ?? null

  return (
    <Dialog open={!!team} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>Rank Rules — {team?.name}</DialogTitle>
      <DialogContent>
        {team && (
          <Stack spacing={2}>
            <Typography variant="body2" color="text.secondary">
              {team.name} is in place {team.place} of the standings.
            </Typography>

            {above ? (
              <Explanation
                heading={`Why ${team.name} is below ${above.other_team}`}
                rule={above}
                team={team.name}
              />
            ) : (
              <Typography variant="body2">
                <strong>{team.name}</strong> is first in the standings, so no team is above it.
              </Typography>
            )}

            {below ? (
              <Explanation
                heading={`Why ${team.name} is above ${below.other_team}`}
                rule={below}
                team={team.name}
              />
            ) : (
              <Typography variant="body2">
                <strong>{team.name}</strong> is last in the standings, so no team is below it.
              </Typography>
            )}

            {(above || below) && (
              <Typography variant="caption" color="text.secondary">
                Rule numbers refer to the order set in “Tie-breaking Rules”.
              </Typography>
            )}
          </Stack>
        )}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose} variant="contained" disableElevation>Close</Button>
      </DialogActions>
    </Dialog>
  )
}
