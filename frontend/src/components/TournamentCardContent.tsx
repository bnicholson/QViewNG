import { CardContent, type CardContentProps } from "@mui/material";
import "./TournamentCardContent.css";
import { getTournamentStatus } from "../utils/tournamentStatus";

interface Props extends CardContentProps {
  tournament: Pick<Tournament, "tname" | "fromdate" | "todate" | "venue" | "shortinfo" | "city" | "state" | "country" | "organization">;
}

/**
 * This displays a tournament, presenting only details likely to be helpful for finding a tournament.
 */
export const TournamentCardContent = (props: Props) => {
  const { className, tournament, ...cardContentProps } = props;
  const status = (tournament.fromdate && tournament.todate)
    ? getTournamentStatus(tournament.fromdate, tournament.todate)
    : null;
  return (
    <CardContent className={`tournamentCardContent${className ? ` ${className}` : ""}`} {...cardContentProps}>
      {(tournament.tname || tournament.fromdate || tournament.todate) && (
        <h3>
          <span>{tournament.tname}</span>
          <span className="dates">{`${tournament.fromdate.format("YYYY-MM-DD")} - ${tournament.todate.format("YYYY-MM-DD")}`}</span>
          {status && <span className={`tournamentStatus ${status.className}`}>{status.label}</span>}
        </h3>
      )}
      {tournament.venue && (
        <p>
          <span className="descriptionTitle">Venue</span>
          <span>{tournament.venue}</span>
        </p>
      )}
      {tournament.shortinfo && (
        <p>
          <span className="descriptionTitle">Info</span>
          <span>{tournament.shortinfo}</span>
        </p>
      )}
      {(tournament.country || tournament.state || tournament.city) && (
        <p>
          <span className="descriptionTitle">Location</span>
          <span>
            {tournament.city}
            {tournament.city && tournament.state && ", "}
            {tournament.state}
          </span>
          <span>{tournament.country}</span>
        </p>
      )}
      {tournament.organization && (  
        <p>
          <span className="descriptionTitle">Organization</span>
          <span>{tournament.organization}</span>
        </p>
      )}
    </CardContent>
  );
};
