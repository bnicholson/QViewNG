import { Box, Container, Grid, Typography } from "@mui/material";

export default function FooterGlobal() {
    return (
        <Box px={{ xs: 3, sm: 10 }} py={{ xs: 5, sm: 10 }} bgcolor="text.secondary" color="white">
            <Typography variant="h6">
            <Container maxWidth='lg'>
                <Grid container spacing={5}>
                <Grid>
                    <Box borderBottom={1}>Help</Box>
                    <Box>Contact</Box>
                    <Box>Support</Box>
                    <Box>Privacy</Box>
                </Grid>
                <Grid>
                    <Box borderBottom={1}>Help</Box>
                    <Box>Login</Box>
                    <Box>Register</Box>
                </Grid>
                <Grid>
                    <Box borderBottom={1}>Messages</Box>
                    <Box>Backup</Box>
                    <Box>History</Box>
                    <Box>Roll</Box>
                </Grid>
                </Grid>
            </Container>
            <Box textAlign="center" pt={{ xs: 5, sm: 10 }} pb={{ xs: 5, sm: 0 }}>
                QView by QuizStuff &reg; 2022-{new Date().getFullYear()}
            </Box>
            </Typography>
        </Box>
    )
}
