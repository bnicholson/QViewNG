import { useAuthCheck } from './hooks/useAuth';
import { usePageTitle } from './hooks/usePageTitle';
import './App.css';
import '@mui/material/colors';
import Toolbar from '@mui/material/Toolbar';
import CssBaseline from '@mui/material/CssBaseline';
import Container from '@mui/material/Container';
import QViewRoutes from './routes';
import FooterGlobal from './components/FooterGlobal';
import GlobalNavBar from './components/GlobalNavBar';
import MobileBottomNav from './components/MobileBottomNav';
import { ProfileHistoryProvider } from './contexts/ProfileHistoryContext';
import { MobileMenuProvider } from './contexts/MobileMenuContext';

if (import.meta.env.NODE_ENV === 'development') import('./setupDevelopment')

export default function App() {
  useAuthCheck()
  usePageTitle()

  return (
    <ProfileHistoryProvider>
    <MobileMenuProvider>
      <CssBaseline />
      <GlobalNavBar />
      <Toolbar />
      <div className="App">
        <Container maxWidth="xl" sx={{ flexGrow: 1, px: { xs: 2, sm: 3, md: 4 }, py: { xs: 2, sm: 3 } }}>
          <QViewRoutes />
        </Container>
        <FooterGlobal />
      </div>
      <MobileBottomNav />
    </MobileMenuProvider>
    </ProfileHistoryProvider>
  )
}
