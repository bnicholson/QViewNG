import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

const validatePassword = (password: string): string | null => {
  if (password.length < 8) return 'Password must be at least 8 characters long.'
  if (!/[A-Z]/.test(password)) return 'Password must contain at least one uppercase letter.'
  if (!/[a-z]/.test(password)) return 'Password must contain at least one lowercase letter.'
  if (!/[0-9]/.test(password)) return 'Password must contain at least one number.'
  if (!/[^A-Za-z0-9]/.test(password)) return 'Password must contain at least one special character.'
  return null
}

export const MyProfileChangePasswordPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  const [processing, setProcessing] = useState(false)
  const [originalPassword, setOriginalPassword] = useState('')
  const [password, setPassword] = useState('')
  const [oldPasswordError, setOldPasswordError] = useState<string | null>(null)
  const [newPasswordError, setNewPasswordError] = useState<string | null>(null)
  const [passwordSuccess, setPasswordSuccess] = useState<string | null>(null)

  const changePassword = async () => {
    setOldPasswordError(null)
    setNewPasswordError(null)
    setPasswordSuccess(null)

    const pwdErr = validatePassword(password)
    if (pwdErr) {
      setNewPasswordError(pwdErr)
      return
    }

    setProcessing(true)
    const response = await fetch('/api/auth/change', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify({
        old_password: originalPassword,
        new_password: password,
      }),
    })
    const body = await response.json().catch(() => ({}))
    setProcessing(false)

    if (response.ok) {
      setOriginalPassword('')
      setPassword('')
      setPasswordSuccess('Password changed successfully.')
    } else if (response.status === 401) {
      setOldPasswordError(body.error ?? 'Current password is incorrect.')
    } else {
      setNewPasswordError(body.error ?? 'Failed to change password.')
    }
  }

  if (!auth.isAuthenticated) {
    return (
      <div>
        <a href="#" onClick={() => navigate('/login')}>Login to change your password</a>
      </div>
    )
  }

  return (
    <div className="Form" style={{ textAlign: 'left' }}>
      <h2>Change Password</h2>
      <br />
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>Current Password</label>
        <input
          type="password"
          value={originalPassword}
          onChange={(e) => { setOriginalPassword(e.target.value); setOldPasswordError(null) }}
        />
        {oldPasswordError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{oldPasswordError}</span>}
      </div>
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <label>New Password</label>
        <input
          type="password"
          value={password}
          onChange={(e) => { setPassword(e.target.value); setNewPasswordError(null) }}
          onBlur={() => setNewPasswordError(password ? validatePassword(password) : null)}
        />
        {newPasswordError && <span style={{ color: 'red', fontSize: '0.85em', marginTop: '4px' }}>{newPasswordError}</span>}
      </div>
      {passwordSuccess && <span style={{ color: 'green', fontSize: '0.85em', marginTop: '4px' }}>{passwordSuccess}</span>}
      <div style={{ display: 'flex', flexFlow: 'column' }}>
        <button disabled={processing} onClick={changePassword}>
          Change Password
        </button>
      </div>
    </div>
  )
}
