import { Box, Container, Link, Typography } from "@mui/material";
import Grid from "@mui/material/Grid";
import { Link as RouterLink } from "react-router-dom";
import versionInfo from "../version.json";
import { useAuth } from "../hooks/useAuth";

export default function FooterGlobal() {
    const auth = useAuth();

    return (
        // Hidden below `md`: on phones the bottom nav takes its place, so the app feels like an app.
        <Box display={{ xs: 'none', md: 'block' }} px={{ xs: 3, sm: 10 }} py={{ xs: 5, sm: 10 }} bgcolor="text.secondary" color="white">
            <Typography variant="h6">
            <Container maxWidth='lg'>
                <Grid container spacing={5}>
                {/* The remaining Help options have no destinations yet. (This column also duplicated
                    the "Help" heading below, which carries Login/Register.)
                <Grid>
                    <Box borderBottom={1}>Help</Box>
                    <Box>Contact</Box>
                    <Box>Support</Box>
                    <Box>Privacy</Box>
                </Grid>
                */}
                <Grid>
                    <Box borderBottom={1}>Accounts</Box>
                    <Box>
                        {auth.isAuthenticated ? (
                            <Link component="button" onClick={() => auth.logout()} color="inherit" underline="hover" sx={{ font: 'inherit', verticalAlign: 'baseline' }}>
                                Logout
                            </Link>
                        ) : (
                            <Link component={RouterLink} to="/login" color="inherit" underline="hover">
                                Login
                            </Link>
                        )}
                    </Box>
                    <Box>
                        <Link component={RouterLink} to="/register" color="inherit" underline="hover">
                            Register
                        </Link>
                    </Box>
                    <Box>
                        <Link component={RouterLink} to="/recovery" color="inherit" underline="hover">
                            Forgot Password
                        </Link>
                    </Box>
                </Grid>
                {/* Messages isn't built yet.
                <Grid>
                    <Box borderBottom={1}>Messages</Box>
                    <Box>Backup</Box>
                    <Box>History</Box>
                    <Box>Roll</Box>
                </Grid>
                */}
                </Grid>
            </Container>
            <Box textAlign="center" pt={{ xs: 5, sm: 10 }} pb={{ xs: 5, sm: 0 }}>
                QView by{' '}
                <Link
                    href="https://www.quizstuff.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    color="inherit"
                    underline="hover"
                >
                    QuizStuff
                </Link>
                &reg; 2022-{new Date().getFullYear()} (v{versionInfo.version})
            </Box>
            </Typography>
        </Box>
    )
}
