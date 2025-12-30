
import { Navigate, useParams } from "react-router-dom";

export default function TournamentRedirect() {
  const { tid_str } = useParams();
  return <Navigate to={`/tournament/${tid_str}/divisions`} replace />;
}
