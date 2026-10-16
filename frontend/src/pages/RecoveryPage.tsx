import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export const RecoveryPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const [username, setUsername] = useState<string>('')
  const [email, setEmail] = useState<string>('')
  const [processing, setProcessing] = useState<boolean>(false)
  const [error, setError] = useState<string | null>(null)

  const recover = async () => {
    if (!username.trim() || !email.trim()) {
      setError('Please enter both the username and the email address of the account.')
      return
    }
    setProcessing(true)
    setError(null)
    // The server answers the same way whether or not the pair matches an account (and only emails
    // on a match), so every successful request goes to the same confirmation page.
    const response = await fetch('/api/auth/forgot', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ username, email }),
    }).catch(() => null)
    setProcessing(false)
    if (response?.ok) {
      navigate('/recovery-sent')
    } else {
      setError('Something went wrong sending the recovery email. Please try again.')
    }
  }

  if (auth.isAuthenticated) {
    navigate('/')
    return <div>Already logged in. Redirecting you to the home page...</div>
  }

  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    {/* Match the Login page's form width so the fields line up across auth pages. */}
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h1>Account Recovery</h1>
      <br />
      <form onSubmit={(e) => { e.preventDefault(); recover() }}>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Username</label>
          <input value={username} onChange={(e) => { setUsername(e.target.value); setError(null) }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Email (of the account)</label>
          <input value={email} onChange={(e) => { setEmail(e.target.value); setError(null) }} />
        </div>
        {error && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{error}</span>}
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <button type="submit" disabled={processing}>
            Send Recovery Email
          </button>
        </div>
      </form>
    </div>
    </div>
  )
}
