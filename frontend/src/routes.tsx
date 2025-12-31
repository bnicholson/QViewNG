import { Route, Routes } from "react-router-dom";
import { TournamentFinder } from "./pages/TournamentFinder";
import { LoginPage } from "./pages/LoginPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { ResetPage } from "./containers/ResetPage";
import { ActivationPage } from "./pages/ActivationPage";
import { RegistrationPage } from "./pages/RegistrationPage";
import { AccountPage } from "./pages/AccountPage";
import { TournamentsPage } from "./pages/TournamentsPage";
import TournamentRedirect from "./pages/TournamentRedirect";
import { TournamentProfile } from "./pages/TournamentProfile";
import { TDEditor } from "./containers/TDEditor";
import { RoundsInProgress } from "./containers/RoundsInProgress";
import { Files } from "./containers/Files";
import { Swagger } from "./containers/Swagger";
import NotFound from "./pages/NotFound";

// NOTE: This component was created for ease-of-access and modularity, not reusability.
//       ONLY the App.tsx file should use this component. 

export default function QViewRoutes() {
    return (
        <Routes>
            <Route path="/" element={<TournamentFinder />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/recovery" element={<RecoveryPage />} />
            <Route path="/reset" element={<ResetPage />} />
            <Route path="/activate" element={<ActivationPage />} />
            <Route path="/register" element={<RegistrationPage />} />
            <Route path="/account" element={<AccountPage />} />
            {/* <Route path="/tournament" element={<Tournaments />} /> */}
            <Route path="/tournaments" element={<TournamentsPage />} />
            <Route path="/tournament/:tid" element={<TournamentRedirect />} />
            <Route path="/tournament/:tid/divisions" element={<TournamentProfile tab="divisions" />} />
            {/* <Route path="/tournament/:tid/division/:did_str" element={<DivisionProfile tab="" />} />  // <- future */}
            <Route path="/tournament/:tid/rooms" element={<TournamentProfile tab="rooms" />} />
            <Route path="/tournament/:tid/teams" element={<TournamentProfile tab="teams" />} />
            <Route path="/tournament/:tid/rounds" element={<TournamentProfile tab="rounds" />} />
            <Route path="/tournament/:tid/quizzers" element={<TournamentProfile tab="quizzers" />} />
            <Route path="/tournament/:tid/games" element={<TournamentProfile tab="games" />} />
            <Route path="/tournament/:tid/admins" element={<TournamentProfile tab="admins" />} />
            <Route path="/tournament/:tid/stats-groups" element={<TournamentProfile tab="stats-groups" />} />
            {/* <Route path="/division" element={<Divisions />} /> */}
            <Route path="/tdeditor" element={<TDEditor />} />
            <Route path="/roundsinprogress" element={<RoundsInProgress />} />
            <Route path="/files" element={<Files />} />
            <Route path="/swagger" element={<Swagger />} />
            <Route path="/404" element={<NotFound />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
    )
}
