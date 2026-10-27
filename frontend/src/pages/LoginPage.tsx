import { useState } from 'react'
import { PasswordInput } from '../components/PasswordField'
import { Navigate, Link, useLocation } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export const LoginPage = () => {
  const auth = useAuth()
  // Where to go after logging in, when a page sent the user here (e.g. a members-only Help article).
  const from = (useLocation().state as { from?: string } | null)?.from
  const [email, setEmail] = useState<string>('')
  const [password, setPassword] = useState<string>('')
  const [processing, setProcessing] = useState<boolean>(false)
  const [loginError, setLoginError] = useState<string | null>(null)

  const login = async () => {
    setProcessing(true)
    setLoginError(null)
    const error = await auth.login(email, password)
    if (error == null) {
      // On success the guard below redirects (back to `from`, else to the user's own profile)
      // once the session (which carries the userId) has been set on the auth context.
      return
    }
    setLoginError(error)
    setProcessing(false)
  }

  if (auth.isAuthenticated) {
    // Only same-app paths are honoured as a return destination.
    const to = from?.startsWith('/') && !from.startsWith('//')
      ? from
      : auth.session?.userId ? `/user/${auth.session.userId}/overview` : '/'
    return <Navigate to={to} replace />
  }

  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h1>Login</h1>
      <br />
      <form onSubmit={(e) => { e.preventDefault(); login() }}>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Email or Username</label>
        <input autoFocus value={email} onChange={(e) => setEmail(e.target.value)} />
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Password</label>
        <PasswordInput
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>
      {loginError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{loginError}</span>}
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <button type="submit" disabled={processing}>
          Login
        </button>
      </div>
      </form>
      <Link
        style={{ marginTop: '30px' }}
        to="/register"
      >
        <b>Register here</b> to create a new account.
      </Link>
      <Link
        style={{ marginTop: '30px' }}
        to="/recovery"
      >
        <b>Forgot password?</b> Click here to recover your account.
      </Link>
    </div>
    </div>
  )
}
