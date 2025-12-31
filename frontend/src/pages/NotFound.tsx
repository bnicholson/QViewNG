import { Link } from "react-router-dom";

export default function NotFound() {
    return (
        <>
            <div style={{ marginTop: "25vh", marginBottom: "20vh" }}>
                <h2>
                    404 Error - Page Not Found
                </h2>
                <p>
                    The page you are looking for isn't here!
                </p>
                <Link to="/">Click here to return to the Home page.</Link>
            </div>
        </>
    )
}
