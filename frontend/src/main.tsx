import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
// import './index.css'
import App from './App.tsx'
import { BrowserRouter } from 'react-router-dom'
import { store } from './store.tsx'
import { Provider } from 'react-redux'
import { ThemeProvider } from '@mui/material'
import theme from './theme.ts'
import { AuthProvider } from './hooks/useAuth.tsx'

export const IS_DEMO = false

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Provider store={store}>
      <ThemeProvider theme={theme}>
        <BrowserRouter>
          <AuthProvider>
            <App />
          </AuthProvider>
        </BrowserRouter>
      </ThemeProvider>
    </Provider>
  </StrictMode>,
)
