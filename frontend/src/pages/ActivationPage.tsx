import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router'
import { useQueryParam } from '../hooks/useQueryParam'
import { IS_DEMO } from '../main'

/** How long the success message shows before moving on to the login page. */
const REDIRECT_MS = 3000

type Status = 'activating' | 'activated' | 'failed'

/**
 * The page the activation email links to (`/activate?token=…`). It activates the account as soon as
 * it loads, then sends the user on to the login page.
 */
export const ActivationPage = () => {
  const navigate = useNavigate()
  const token = useQueryParam('token') || ''
  const [status, setStatus] = useState<Status>(token && !IS_DEMO ? 'activating' : 'failed')
  const [error, setError] = useState<string | null>(
    IS_DEMO ? null : token ? null : 'This activation link is missing its token.',
  )
  // Tokens are single-use, so a second request (StrictMode runs effects twice) would report failure.
  const requested = useRef(false)

  useEffect(() => {
    if (IS_DEMO || !token || requested.current) return
    requested.current = true

    const activate = async () => {
      const response = await fetch(`/api/auth/activate?activation_token=${encodeURIComponent(token)}`)
        .catch(() => null)
      if (response?.ok) {
        setStatus('activated')
      } else {
        const body = await response?.json().catch(() => ({}))
        setError(body?.error ?? 'Invalid or expired activation link.')
        setStatus('failed')
      }
    }
    activate()
  }, [token])

  useEffect(() => {
    if (status !== 'activated') return
    const timer = setTimeout(() => navigate('/login', { replace: true }), REDIRECT_MS)
    return () => clearTimeout(timer)
  }, [status, navigate])

  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h1>Activate Account</h1>
      {IS_DEMO && (
        <div style={{ background: '#fef9c3', border: '1px solid #ca8a04', borderRadius: 6, padding: '10px 14px', marginBottom: 12, color: '#713f12', fontSize: '0.9em' }}>
          <strong>Demo Mode:</strong> QView is in Demo Mode. Accounts are not activated.
        </div>
      )}
      {status === 'activating' && <p>Activating your account…</p>}
      {status === 'activated' && (
        <p>Your account is activated. Taking you to the login page…</p>
      )}
      {status === 'failed' && error && (
        <>
          <p style={{ color: 'red' }}>{error}</p>
          <p>
            Activation links expire after 24 hours and can only be used once. If you have already
            activated your account, you can log in.
          </p>
        </>
      )}
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/login')}
      >
        Click here to login.
      </a>
    </div>
    </div>
  )
}
