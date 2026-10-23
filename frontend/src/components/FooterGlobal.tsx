import { Box, Container, Typography } from "@mui/material";
import Grid from "@mui/material/Grid";
import QViewAttribution from "./QViewAttribution";
import { AccountsLinkSection, ExternalLinkSection } from "./LinkSections";
import { EXTERNAL_LINK_SECTIONS } from "./ExternalLinks";

export default function FooterGlobal() {
    return (
        // Hidden below `md`: on phones the bottom nav takes its place, so the app feels like an app.
        <Box display={{ xs: 'none', md: 'block' }} px={6} py={4} bgcolor="text.secondary" color="white">
            <Typography variant="h6" component="div">
            <Container maxWidth='lg'>
                <Grid container spacing={5} justifyContent="center">
                {/* The remaining Help options have no destinations yet. (This column also duplicated
                    the "Help" heading below, which carries Login/Register.)
                <Grid>
                    <Box borderBottom={1}>Help</Box>
                    <Box>Contact</Box>
                    <Box>Support</Box>
                    <Box>Privacy</Box>
                </Grid>
                */}
                {/* The footer is wrapped in an h6 Typography; each column resets to body text so its
                    sections render exactly as they do on the mobile About page. */}
                {EXTERNAL_LINK_SECTIONS.map(section => (
                    <Grid key={section.heading} sx={{ typography: 'body1' }}>
                        <ExternalLinkSection heading={section.heading} links={section.links} linkColor="inherit" />
                    </Grid>
                ))}
                <Grid sx={{ typography: 'body1' }}>
                    <AccountsLinkSection linkColor="inherit" />
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
            <Box textAlign="center" pt={4} fontWeight={600}>
                <QViewAttribution />
            </Box>
            </Typography>
        </Box>
    )
}
