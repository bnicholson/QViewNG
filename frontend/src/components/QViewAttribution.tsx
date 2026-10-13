import { Link } from "@mui/material";
import versionInfo from "../version.json";

/**
 * "QView by QuizStuff® 2022-{year} (v{version})". The footer and the About QView page both show
 * it, so the version number on each comes from the same place.
 */
export default function QViewAttribution() {
    return (
        <>
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
        </>
    )
}
