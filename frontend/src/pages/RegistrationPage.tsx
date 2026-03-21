import { useState } from 'react'
import { useNavigate } from 'react-router-dom'

const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

const validatePassword = (password: string): string | null => {
  if (password.length < 8) return 'Password must be at least 8 characters long.'
  if (!/[A-Z]/.test(password)) return 'Password must contain at least one uppercase letter.'
  if (!/[a-z]/.test(password)) return 'Password must contain at least one lowercase letter.'
  if (!/[0-9]/.test(password)) return 'Password must contain at least one number.'
  if (!/[^A-Za-z0-9]/.test(password)) return 'Password must contain at least one special character.'
  return null
}

export const RegistrationPage = () => {
  const navigate = useNavigate()
  const [fname, setFname] = useState<string>('')
  const [mname, setMname] = useState<string>('')
  const [lname, setLname] = useState<string>('')
  const [username, setUsername] = useState<string>('')
  const [email, setEmail] = useState<string>('')
  const [password, setPassword] = useState<string>('')
  const [processing, setProcessing] = useState<boolean>(false)
  const [fnameError, setFnameError] = useState<string | null>(null)
  const [lnameError, setLnameError] = useState<string | null>(null)
  const [usernameError, setUsernameError] = useState<string | null>(null)
  const [emailError, setEmailError] = useState<string | null>(null)
  const [passwordError, setPasswordError] = useState<string | null>(null)
  const [serverError, setServerError] = useState<string | null>(null)

  const handleEmailBlur = () => {
    if (email && !EMAIL_REGEX.test(email)) {
      setEmailError('Please enter a valid email address.')
    } else {
      setEmailError(null)
    }
  }

  const handlePasswordBlur = () => {
    setPasswordError(password ? validatePassword(password) : null)
  }

  const register = async () => {
    const fnErr = fname.trim() ? null : 'First name is required.'
    const lnErr = lname.trim() ? null : 'Last name is required.'
    const unameErr = username.trim() ? null : 'Username is required.'
    const emailErr = email && !EMAIL_REGEX.test(email) ? 'Please enter a valid email address.' : null
    const pwdErr = validatePassword(password)
    setFnameError(fnErr)
    setLnameError(lnErr)
    setUsernameError(unameErr)
    setEmailError(emailErr ?? null)
    setPasswordError(pwdErr)
    if (fnErr || lnErr || unameErr || emailErr || pwdErr) return

    setProcessing(true)
    setServerError(null)
    const response = await fetch('/api/auth/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ fname, mname: mname || undefined, lname, username, email, password }),
    })
    setProcessing(false)
    if (response.ok) {
      navigate('/activate')
    } else {
      const body = await response.json().catch(() => ({}))
      setServerError(body.error ?? 'Registration failed. Please try again.')
    }
  }

  return (
    <div className="Form" style={{ textAlign: 'left' }}>
      <h1>Registration</h1>
      <br />
      <form onSubmit={(e) => { e.preventDefault(); register() }}>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>First Name</label>
        <input
          value={fname}
          onChange={(e) => { setFname(e.target.value); setFnameError(null) }}
          onBlur={() => setFnameError(fname.trim() ? null : 'First name is required.')}
        />
        {fnameError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{fnameError}</span>}
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Middle Name <span style={{ color: '#888', fontSize: '0.85em' }}>(optional)</span></label>
        <input value={mname} onChange={(e) => setMname(e.target.value)} />
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Last Name</label>
        <input
          value={lname}
          onChange={(e) => { setLname(e.target.value); setLnameError(null) }}
          onBlur={() => setLnameError(lname.trim() ? null : 'Last name is required.')}
        />
        {lnameError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{lnameError}</span>}
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Username</label>
        <input
          value={username}
          onChange={(e) => { setUsername(e.target.value); setUsernameError(null) }}
          onBlur={() => setUsernameError(username.trim() ? null : 'Username is required.')}
        />
        {usernameError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{usernameError}</span>}
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Email</label>
        <input
          value={email}
          onChange={(e) => { setEmail(e.target.value); setEmailError(null) }}
          onBlur={handleEmailBlur}
        />
        {emailError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{emailError}</span>}
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Password</label>
        <input
          type="password"
          value={password}
          onChange={(e) => { setPassword(e.target.value); setPasswordError(null) }}
          onBlur={handlePasswordBlur}
        />
        {passwordError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{passwordError}</span>}
      </div>
      {serverError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{serverError}</span>}
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <button type="submit" disabled={processing}>
          Register
        </button>
      </div>
      </form>
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/login')}
      >
        Already have an account? Click here to login.
      </a>
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/activate')}
      >
        Need to activate your account? Click here.
      </a>
    </div>
  )
}
