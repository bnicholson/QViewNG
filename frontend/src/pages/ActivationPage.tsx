import { useState } from 'react'
import { useNavigate } from 'react-router'
import { useQueryParam } from '../hooks/useQueryParam'

export const ActivationPage = () => {
  const navigate = useNavigate()
  const token = useQueryParam('token') || ''
  const [activationToken, setActivationToken] = useState<string>(token)
  const [processing, setProcessing] = useState<boolean>(false)
  const [error, setError] = useState<string | null>(null)

  const activate = async () => {
    if (!activationToken.trim()) {
      setError('Please enter your activation token.')
      return
    }
    setProcessing(true)
    setError(null)
    const response = await fetch(
      `/api/auth/activate?activation_token=${encodeURIComponent(activationToken)}`,
    )
    if (response.ok) {
      navigate('/login')
    } else {
      const body = await response.json().catch(() => ({}))
      setError(body.error ?? 'Invalid or expired activation token.')
    }
    setProcessing(false)
  }

  return (
    <div className="Form" style={{ textAlign: 'left' }}>
      <h1>Activate Account</h1>
      <p style={{ marginBottom: '16px' }}>
        Enter the activation token from the email sent to you after registration.
      </p>
      <form onSubmit={(e) => { e.preventDefault(); activate() }}>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Activation Token</label>
          <input
            value={activationToken}
            onChange={(e) => { setActivationToken(e.target.value); setError(null) }}
          />
          {error && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{error}</span>}
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <button type="submit" disabled={processing}>
            Activate
          </button>
        </div>
      </form>
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/login')}
      >
        Already activated? Click here to login.
      </a>
    </div>
  )
}
