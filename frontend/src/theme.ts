import { createTheme } from "@mui/material"

const theme = createTheme({
  palette: {
    primary: {
      main: "#008080"
    }
  },
  typography: {
    button: {
      textTransform: 'none'
    }
  }
})

export default theme