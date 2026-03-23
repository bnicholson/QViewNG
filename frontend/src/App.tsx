import { useAuthCheck } from './hooks/useAuth';
import './App.css';
import '@mui/material/colors';
import Toolbar from '@mui/material/Toolbar';
import CssBaseline from '@mui/material/CssBaseline';
import Container from '@mui/material/Container';
import QViewRoutes from './routes';
import FooterGlobal from './components/FooterGlobal';
import GlobalNavBar from './components/GlobalNavBar';

if (import.meta.env.NODE_ENV === 'development') import('./setupDevelopment')

export default function App() {
  useAuthCheck()

  return (
    <>
      <CssBaseline />
      <GlobalNavBar />
      <Toolbar />
      <div className="App">
        <Container maxWidth="xl" sx={{ px: { xs: 2, sm: 3, md: 4 }, py: { xs: 2, sm: 3 } }}>
          <QViewRoutes />
        </Container>
        <FooterGlobal />
      </div>
    </>
  )
}
