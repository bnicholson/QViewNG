import { Route, Routes } from "react-router-dom";
import { TournamentFinder } from "./pages/TournamentFinder";
import { LoginPage } from "./pages/LoginPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { ResetPage } from "./containers/ResetPage";
import { ActivationPage } from "./pages/ActivationPage";
import { RegistrationPage } from "./pages/RegistrationPage";
import { MyProfilePage } from "./pages/MyProfilePage";
import { UserProfilePage } from "./pages/UserProfilePage";
import { TournamentsPage } from "./pages/TournamentsPage";
import { TournamentProfile } from "./pages/TournamentProfile";
import { TeamProfile } from "./pages/TeamProfile";
import { TDEditor } from "./containers/TDEditor";
import { RoundsInProgress } from "./containers/RoundsInProgress";
import { Files } from "./containers/Files";
import { Swagger } from "./containers/Swagger";
import NotFound from "./pages/NotFound";
import { ManageUsers } from "./pages/ManageUsers";
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
            <Route path="/register" element={<RegistrationPage />} />
            <Route path="/my-profile" element={<MyProfilePage/>} />
            <Route path="/my-profile/overview" element={<MyProfilePage childRoute="overview" />} />
            <Route path="/my-profile/permissions" element={<MyProfilePage childRoute="permissions" />} />
            <Route path="/my-profile/change-password" element={<MyProfilePage childRoute="change-password" />} />
            <Route path="/my-profile/sessions" element={<MyProfilePage childRoute="sessions" />} />
            <Route path="/user/:user_id" element={<UserProfilePage />} />
            <Route path="/user/:user_id/overview" element={<UserProfilePage childRoute="overview" />} />
            <Route path="/user/:user_id/permissions" element={<UserProfilePage childRoute="permissions" />} />
            <Route path="/user/:user_id/change-password" element={<UserProfilePage childRoute="change-password" />} />
            <Route path="/user/:user_id/sessions" element={<UserProfilePage childRoute="sessions" />} />
            <Route path="/user/:user_id/as-quizzer" element={<UserProfilePage childRoute="as-quizzer" />} />
            <Route path="/user/:user_id/as-coach" element={<UserProfilePage childRoute="as-coach" />} />
            <Route path="/user/:user_id/as-coach-rosters" element={<UserProfilePage childRoute="as-coach-rosters" />} />
            <Route path="/user/:user_id/as-coach-gear" element={<UserProfilePage childRoute="as-coach-gear" />} />
            <Route path="/user/:user_id/as-admin" element={<UserProfilePage childRoute="as-admin" />} />
            <Route path="/user/:user_id/as-quizmaster" element={<UserProfilePage childRoute="as-quizmaster" />} />
            <Route path="/user/:user_id/as-content-judge" element={<UserProfilePage childRoute="as-content-judge" />} />
            <Route path="/crm/users" element={<ManageUsers />} />
            <Route path="/tournaments-page" element={<TournamentsPage />} />
            <Route path="/tournament/:tid" element={<TournamentProfile/>} />
            <Route path="/tournament/:tid/overview" element={<TournamentProfile childRoute="overview" />} />
            <Route path="/tournament/:tid/divisions" element={<TournamentProfile childRoute="divisions" />} />
            {/* <Route path="/tournament/:tid/division/:did_str" element={<DivisionProfile childRoute="" />} />  // <- future */}
            <Route path="/tournament/:tid/rooms" element={<TournamentProfile childRoute="rooms" />} />
            <Route path="/tournament/:tid/teams" element={<TournamentProfile childRoute="teams" />} />
            <Route path="/tournament/:tid/team/:teamid" element={<TeamProfile />} />
            <Route path="/tournament/:tid/team/:teamid/overview" element={<TeamProfile childRoute="overview" />} />
            <Route path="/tournament/:tid/team/:teamid/quizzers" element={<TeamProfile childRoute="quizzers" />} />
            <Route path="/tournament/:tid/rounds" element={<TournamentProfile childRoute="rounds" />} />
            <Route path="/tournament/:tid/quizzers" element={<TournamentProfile childRoute="quizzers" />} />
            <Route path="/tournament/:tid/games" element={<TournamentProfile childRoute="games" />} />
            <Route path="/tournament/:tid/admins" element={<TournamentProfile childRoute="admins" />} />
            <Route path="/tournament/:tid/stats-groups" element={<TournamentProfile childRoute="stats-groups" />} />
            <Route path="/tournament/:tid/room-monitor" element={<TournamentProfile childRoute="room-monitor" />} />
            {/* <Route path="/division" element={<Divisions />} /> */}
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
