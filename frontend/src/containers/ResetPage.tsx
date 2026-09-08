import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { useQueryParam } from '../hooks/useQueryParam'
import { PasswordForm } from '../components/PasswordForm'

export const ResetPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const resetToken = useQueryParam('token')

  if (auth.isAuthenticated) {
    navigate('/')
    return <div>Already logged in. Redirecting you to the home page...</div>
  }

  return (
    <PasswordForm
      title="Account Recovery"
      submitLabel="Recover"
      onSubmit={async ({ newPassword }) => {
        const response = await fetch('/api/auth/reset', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            reset_token: resetToken,
            new_password: newPassword,
          }),
        })
        if (response.ok) {
          navigate('/login')
          return { ok: true }
        }
        const body = await response.json().catch(() => ({}))
        return {
          ok: false,
          field: 'general',
          message: body.error ?? 'Failed to reset password. Your recovery link may have expired.',
        }
      }}
    />
  )
}
