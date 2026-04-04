import Box from '@mui/material/Box'
// import Divider from '@mui/material/Divider'
// import Typography from '@mui/material/Typography'
import GamesTable from '../components/GamesTable'

interface Props {
  tid: string
  roundid: string
}

export const RoundProfileGamesPage = ({ tid, roundid }: Props) => {
  return (
    <Box>
      {/* <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
        Games
      </Typography> */}
      {/* <Divider sx={{ mb: 2 }} /> */}
      <GamesTable tid={tid} roundid={roundid} />
    </Box>
  )
}
