import { useState } from 'react'
import { Navigate, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export const LoginPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const [email, setEmail] = useState<string>('')
  const [password, setPassword] = useState<string>('')
  const [processing, setProcessing] = useState<boolean>(false)
  const [loginError, setLoginError] = useState<string | null>(null)

  const login = async () => {
    setProcessing(true)
    setLoginError(null)
    const error = await auth.login(email, password)
    if (error == null) {
      navigate('/')
      return
    }
    setLoginError(error)
    setProcessing(false)
  }

  if (auth.isAuthenticated) {
    return <Navigate to="/" replace />
  }

  return (
    <div className="Form" style={{ textAlign: 'left' }}>
      <h1>Login</h1>
      <br />
      <form onSubmit={(e) => { e.preventDefault(); login() }}>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Email or Username</label>
        <input value={email} onChange={(e) => setEmail(e.target.value)} />
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Password</label>
        <input
          type="password"
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
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/register')}
      >
        Don't have an account? Click here to register.
      </a>
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/recovery')}
      >
        Forgot your password? Click here to recover your account.
      </a>
    </div>
  )
}
