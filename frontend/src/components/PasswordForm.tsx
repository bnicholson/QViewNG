import { useState } from 'react'

/** Shared password-strength rules for setting a new password. */
export const validatePassword = (password: string): string | null => {
  if (password.length < 8) return 'Password must be at least 8 characters long.'
  if (!/[A-Z]/.test(password)) return 'Password must contain at least one uppercase letter.'
  if (!/[a-z]/.test(password)) return 'Password must contain at least one lowercase letter.'
  if (!/[0-9]/.test(password)) return 'Password must contain at least one number.'
  if (!/[^A-Za-z0-9]/.test(password)) return 'Password must contain at least one special character.'
  return null
}

/** Result the caller's onSubmit returns so the form can render field-level errors / success. */
export type PasswordSubmitResult =
  | { ok: true; successMessage?: string }
  | { ok: false; field?: 'current' | 'new' | 'general'; message: string }

interface PasswordFormProps {
  title: string
  submitLabel: string
  /** When true, shows a "Current Password" field (used by the authenticated Change Password flow). */
  requireCurrentPassword?: boolean
  /** Disable the form (e.g. demo mode). */
  disabled?: boolean
  /** Optional notice rendered under the title (e.g. a demo-mode banner). */
  notice?: React.ReactNode
  /**
   * Performs the backend call. Only invoked after local validation passes
   * (strength rules + the two new-password fields matching).
   */
  onSubmit: (values: { currentPassword: string; newPassword: string }) => Promise<PasswordSubmitResult>
}

const fieldStyle: React.CSSProperties = { display: 'flex', flexFlow: 'column' }
const errorStyle: React.CSSProperties = { color: 'red', fontSize: '0.85em', marginTop: '4px' }
const successStyle: React.CSSProperties = { color: 'green', fontSize: '0.85em', marginTop: '4px' }

/**
 * The single, reusable "set a new password" form. Used by both the authenticated
 * Change Password page (with the current-password field) and the password-recovery
 * (reset) page, so the same UI, validation, and match-check are shared between them.
 */
export const PasswordForm = ({
  title,
  submitLabel,
  requireCurrentPassword = false,
  disabled = false,
  notice,
  onSubmit,
}: PasswordFormProps) => {
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [processing, setProcessing] = useState(false)

  const [currentError, setCurrentError] = useState<string | null>(null)
  const [newError, setNewError] = useState<string | null>(null)
  const [confirmError, setConfirmError] = useState<string | null>(null)
  const [generalError, setGeneralError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)

  const submit = async () => {
    if (disabled) return
    setCurrentError(null)
    setNewError(null)
    setConfirmError(null)
    setGeneralError(null)
    setSuccess(null)

    // Local validation runs BEFORE any backend call.
    const strengthError = validatePassword(newPassword)
    if (strengthError) {
      setNewError(strengthError)
      return
    }
    if (newPassword !== confirmPassword) {
      setConfirmError('Passwords do not match.')
      return
    }

    setProcessing(true)
    let result: PasswordSubmitResult
    try {
      result = await onSubmit({ currentPassword, newPassword })
    } catch {
      result = { ok: false, field: 'general', message: 'Something went wrong. Please try again.' }
    }
    setProcessing(false)

    if (result.ok) {
      setCurrentPassword('')
      setNewPassword('')
      setConfirmPassword('')
      if (result.successMessage) setSuccess(result.successMessage)
    } else if (result.field === 'current') {
      setCurrentError(result.message)
    } else if (result.field === 'new') {
      setNewError(result.message)
    } else {
      setGeneralError(result.message)
    }
  }

  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    {/* Match the Login page's form width so the fields line up across auth pages. */}
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h2>{title}</h2>
      {notice}
      <br />

      {requireCurrentPassword && (
        <div style={fieldStyle}>
          <label>Current Password</label>
          <input
            type="password"
            value={currentPassword}
            onChange={(e) => { setCurrentPassword(e.target.value); setCurrentError(null) }}
          />
          {currentError && <span style={errorStyle}>{currentError}</span>}
        </div>
      )}

      <div style={fieldStyle}>
        <label>New Password</label>
        <input
          type="password"
          value={newPassword}
          onChange={(e) => { setNewPassword(e.target.value); setNewError(null) }}
          onBlur={() => setNewError(newPassword ? validatePassword(newPassword) : null)}
        />
        {newError && <span style={errorStyle}>{newError}</span>}
      </div>

      <div style={fieldStyle}>
        <label>Confirm New Password</label>
        <input
          type="password"
          value={confirmPassword}
          onChange={(e) => { setConfirmPassword(e.target.value); setConfirmError(null) }}
          onBlur={() => setConfirmError(confirmPassword && newPassword !== confirmPassword ? 'Passwords do not match.' : null)}
        />
        {confirmError && <span style={errorStyle}>{confirmError}</span>}
      </div>

      {generalError && <span style={errorStyle}>{generalError}</span>}
      {success && <span style={successStyle}>{success}</span>}

      <div style={fieldStyle}>
        <button disabled={processing || disabled} onClick={submit}>
          {submitLabel}
        </button>
      </div>
    </div>
    </div>
  )
}
