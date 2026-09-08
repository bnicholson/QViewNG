import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { IS_DEMO } from '../main'
import { PasswordForm } from '../components/PasswordForm'

export const UserProfileChangePasswordPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  if (!auth.isAuthenticated) {
    return (
      <div>
        <a href="#" onClick={() => navigate('/login')}>Login to change your password</a>
      </div>
    )
  }

  return (
    <PasswordForm
      title="Change Password"
      submitLabel="Change Password"
      requireCurrentPassword
      disabled={IS_DEMO}
      notice={IS_DEMO ? (
        <div style={{ background: '#fef9c3', border: '1px solid #ca8a04', borderRadius: 6, padding: '10px 14px', marginBottom: 12, color: '#713f12', fontSize: '0.9em' }}>
          <strong>Demo Mode:</strong> QView is in Demo Mode. Password changes are disabled and will not be persisted.
        </div>
      ) : undefined}
      onSubmit={async ({ currentPassword, newPassword }) => {
        const response = await fetch('/api/auth/change', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${auth.accessToken}`,
          },
          body: JSON.stringify({
            old_password: currentPassword,
            new_password: newPassword,
          }),
        })
        const body = await response.json().catch(() => ({}))
        if (response.ok) {
          return { ok: true, successMessage: 'Password changed successfully.' }
        }
        if (response.status === 401) {
          return { ok: false, field: 'current', message: body.error ?? 'Current password is incorrect.' }
        }
        return { ok: false, field: 'new', message: body.error ?? 'Failed to change password.' }
      }}
    />
  )
}
