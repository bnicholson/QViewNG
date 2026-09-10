import { Typography, Grid } from "@mui/material"

interface InfoItemProps {
  label: string,
  value: string
}

export const InfoItem = ({label, value}: InfoItemProps) => {
  return (
    <>
      <Grid size={{ xs: 12, sm: 6, md: 4 }}>
        <Typography variant="body2" color="text.secondary">{label}</Typography>
        <Typography variant="body1">{value}</Typography>
      </Grid>
    </>
  )
}