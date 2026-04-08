import { Route, Routes } from "react-router-dom";
import { TournamentFinder } from "./pages/TournamentFinder";
import { LoginPage } from "./pages/LoginPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { ResetPage } from "./containers/ResetPage";
import { ActivationPage } from "./pages/ActivationPage";
import { UserRegistrationPage } from "./pages/UserRegistrationPage";
import { UserProfilePage } from "./pages/UserProfilePage";
import { TournamentsPage } from "./pages/TournamentsPage";
import { TournamentProfile } from "./pages/TournamentProfile";
import { TournamentGroupProfile } from "./pages/TournamentGroupProfile"
import { RoundProfile } from "./pages/RoundProfile"
import { DivisionProfile } from "./pages/DivisionProfile"
import { RoomProfile } from "./pages/RoomProfile";
import { GameProfile } from "./pages/GameProfile";
import { TeamProfile } from "./pages/TeamProfile";
import { TDEditor } from "./containers/TDEditor";
import { RoundsInProgress } from "./containers/RoundsInProgress";
import { Files } from "./containers/Files";
import { Swagger } from "./containers/Swagger";
import NotFound from "./pages/NotFound";
import { CRMProfile } from "./pages/CRMProfile";
import { DevPage } from "./pages/DevPage";

// NOTE: This component was created for ease-of-access and modularity, not reusability.
//       ONLY the App.tsx file should use this component. 

export default function QViewRoutes() {
    return (
        <Routes>
            <Route path="/" element={<TournamentFinder />} />
            <Route path="/tournaments" element={<TournamentFinder />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/recovery" element={<RecoveryPage />} />
            <Route path="/reset" element={<ResetPage />} />
            <Route path="/activate" element={<ActivationPage />} />
            <Route path="/register" element={<UserRegistrationPage />} />
            <Route path="/user/:user_id" element={<UserProfilePage />} />
            <Route path="/user/:user_id/overview" element={<UserProfilePage childRoute="overview" />} />
            <Route path="/user/:user_id/permissions" element={<UserProfilePage childRoute="permissions" />} />
            <Route path="/user/:user_id/change-password" element={<UserProfilePage childRoute="change-password" />} />
            <Route path="/user/:user_id/sessions" element={<UserProfilePage childRoute="sessions" />} />
            <Route path="/user/:user_id/teams" element={<UserProfilePage childRoute="teams" />} />
            <Route path="/user/:user_id/my-rosters" element={<UserProfilePage childRoute="my-rosters" />} />
            <Route path="/user/:user_id/my-gear" element={<UserProfilePage childRoute="my-gear" />} />
            <Route path="/user/:user_id/as-admin" element={<UserProfilePage childRoute="as-admin" />} />
            <Route path="/user/:user_id/as-quizmaster" element={<UserProfilePage childRoute="as-quizmaster" />} />
            <Route path="/user/:user_id/as-content-judge" element={<UserProfilePage childRoute="as-content-judge" />} />
            <Route path="/user/:user_id/managed-tournaments" element={<UserProfilePage childRoute="managed-tournaments" />} />
            <Route path="/user/:user_id/managed-tournament-groups" element={<UserProfilePage childRoute="managed-tournament-groups" />} />
            <Route path="/crm" element={<CRMProfile />} />
            <Route path="/crm/users" element={<CRMProfile childRoute="users" />} />
            <Route path="/crm/create-tournament-applicants" element={<CRMProfile childRoute="create-tournament-applicants" />} />
            <Route path="/tournaments-page" element={<TournamentsPage />} />
            <Route path="/tournament/:tid" element={<TournamentProfile/>} />
            <Route path="/tournament/:tid/overview" element={<TournamentProfile childRoute="overview" />} />
            <Route path="/tournament/:tid/register" element={<TournamentProfile childRoute="register/team" />} />
            <Route path="/tournament/:tid/register/team" element={<TournamentProfile childRoute="register/team" />} />
            <Route path="/tournament/:tid/register/gear" element={<TournamentProfile childRoute="register/gear" />} />
            <Route path="/tournament/:tid/register/as-volunteer" element={<TournamentProfile childRoute="register/volunteer" />} />
            <Route path="/tournament/:tid/divisions" element={<TournamentProfile childRoute="divisions" />} />
            <Route path="/tournament/:tid/rooms" element={<TournamentProfile childRoute="rooms" />} />
            <Route path="/tournament/:tid/teams" element={<TournamentProfile childRoute="teams" />} />
            <Route path="/team/:teamid" element={<TeamProfile />} />
            <Route path="/team/:teamid/overview" element={<TeamProfile childRoute="overview" />} />
            <Route path="/team/:teamid/quizzers" element={<TeamProfile childRoute="quizzers" />} />
            <Route path="/tournament/:tid/rounds" element={<TournamentProfile childRoute="rounds" />} />
            <Route path="/tournament/:tid/quizzers" element={<TournamentProfile childRoute="quizzers" />} />
            <Route path="/tournament/:tid/games" element={<TournamentProfile childRoute="games" />} />
            <Route path="/tournament/:tid/gear" element={<TournamentProfile childRoute="gear" />} />
            <Route path="/tournament/:tid/admins" element={<TournamentProfile childRoute="admins" />} />
            <Route path="/tournament/:tid/tournament-groups" element={<TournamentProfile childRoute="tournament-groups" />} />
            <Route path="/tournament/:tid/stats-groups" element={<TournamentProfile childRoute="stats-groups" />} />
            <Route path="/tournament/:tid/room-monitor" element={<TournamentProfile childRoute="room-monitor" />} />
            <Route path="/tournament-group/:tgid" element={<TournamentGroupProfile />} />
            <Route path="/tournament-group/:tgid/overview" element={<TournamentGroupProfile childRoute="overview" />} />
            <Route path="/tournament-group/:tgid/tournaments" element={<TournamentGroupProfile childRoute="tournaments" />} />
            <Route path="/tournament-group/:tgid/stats-groups" element={<TournamentGroupProfile childRoute="stats-groups" />} />
            <Route path="/round/:roundid" element={<RoundProfile />} />
            <Route path="/round/:roundid/overview" element={<RoundProfile childRoute="overview" />} />
            <Route path="/round/:roundid/games" element={<RoundProfile childRoute="games" />} />
            <Route path="/division/:did" element={<DivisionProfile />} />
            <Route path="/division/:did/overview" element={<DivisionProfile childRoute="overview" />} />
            <Route path="/division/:did/teams" element={<DivisionProfile childRoute="teams" />} />
            <Route path="/division/:did/quizzers" element={<DivisionProfile childRoute="quizzers" />} />
            <Route path="/division/:did/rounds" element={<DivisionProfile childRoute="rounds" />} />
            <Route path="/division/:did/games" element={<DivisionProfile childRoute="games" />} />
            <Route path="/division/:did/stats-groups" element={<DivisionProfile childRoute="stats-groups" />} />
            <Route path="/room/:roomid" element={<RoomProfile />} />
            <Route path="/room/:roomid/overview" element={<RoomProfile childRoute="overview" />} />
            <Route path="/room/:roomid/games" element={<RoomProfile childRoute="games" />} />
            <Route path="/game/:gid" element={<GameProfile />} />
            <Route path="/game/:gid/overview" element={<GameProfile childRoute="overview" />} />
            <Route path="/tdeditor" element={<TDEditor />} />
            <Route path="/rounds-in-progress" element={<RoundsInProgress />} />
            <Route path="/files" element={<Files />} />
            <Route path="/swagger" element={<Swagger />} />
            <Route path="/dev" element={<DevPage />} />
            <Route path="/404" element={<NotFound />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
    )
}
