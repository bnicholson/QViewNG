import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import type { UserSessionResponse } from '../hooks/useAuth'

const validatePassword = (password: string): string | null => {
  if (password.length < 8) return 'Password must be at least 8 characters long.'
  if (!/[A-Z]/.test(password)) return 'Password must contain at least one uppercase letter.'
  if (!/[a-z]/.test(password)) return 'Password must contain at least one lowercase letter.'
  if (!/[0-9]/.test(password)) return 'Password must contain at least one number.'
  if (!/[^A-Za-z0-9]/.test(password)) return 'Password must contain at least one special character.'
  return null
}

export const MyProfilePermissionsPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  const [processing, setProcessing] = useState<boolean>(false)
  const [originalPassword, setOriginalPassword] = useState<string>('')
  const [password, setPassword] = useState<string>('')
  const [oldPasswordError, setOldPasswordError] = useState<string | null>(null)
  const [newPasswordError, setNewPasswordError] = useState<string | null>(null)
  const [passwordSuccess, setPasswordSuccess] = useState<string | null>(null)

  const [page, setPage] = useState<number>(0)
  const pageSize = 10

  const [isFetchingSessions, setFetchingSessions] = useState<boolean>(false)
  const [sessions, setSessions] = useState<UserSessionResponse>({
    sessions: [],
    num_pages: 1,
  })

  const [isDeleting, setDeleting] = useState<boolean>(false)

  const deleteSession = async (id: number) => {
    setDeleting(true)

    const response = await fetch(`/api/auth/sessions/${id}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })

    if (response.ok) {
      if (sessions.sessions.length === 1 && page !== 0) {
        setPage(page - 1)
      }
      await fetchSessions()
    }

    setDeleting(false)
  }

  const deleteAllSessions = async () => {
    setDeleting(true)

    const response = await fetch(`/api/auth/sessions`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })

    if (response.ok) {
      setPage(0)
      await fetchSessions()
    }

    setDeleting(false)
  }

  const fetchSessions = async () => {
    setFetchingSessions(true)

    if (!auth.isAuthenticated) {
      setSessions({ sessions: [], num_pages: 1 })
      setFetchingSessions(false)
      return
    }

    const data = await (
      await fetch(`/api/auth/sessions?page=${page}&page_size=${pageSize}`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${auth.accessToken}`,
        },
      })
    ).json()

    setSessions(data)
    setFetchingSessions(false)
  }

  useEffect(() => {
    fetchSessions()
  }, [auth.isAuthenticated, page])

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

  return (
    <div style={{ textAlign: 'left' }}>
        {auth.isAuthenticated && (
          <div>
            <div id="roles">
              <div className="Form" style={{ textAlign: 'left' }}>
                <h2>Roles</h2>
                <pre>
                  {auth.session?.roles?.length
                    ? auth.session.roles.map((role) => <div key={role}>{role}</div>)
                    : <div>No roles assigned.</div>
                  }
                </pre>
              </div>
            </div>

            <div id="permissions" style={{ marginTop: '32px' }}>
              <div className="Form" style={{ textAlign: 'left' }}>
                <h2>Permissions</h2>
                <pre>
                  {!auth.session && (
                    <div>Error: No auth session present.</div>
                  )}
                  {auth.session?.permissions?.map((perm) => (
                    <div key={perm}>{perm}</div>
                  ))}
                  {auth.session?.permissions?.length === 0 && (
                    <div>No permissions granted.</div>
                  )}
                </pre>
              </div>
            </div>

            <div id="change-password" style={{ marginTop: '32px' }}>
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
            </div>

            <div id="sessions" style={{ marginTop: '32px' }}>
              <h2>Sessions</h2>
              <button disabled={isDeleting} onClick={() => deleteAllSessions()}>
                Delete All
              </button>
              {sessions.sessions.map((session) => (
                <div key={session.id}>
                  {JSON.stringify(session, null, 2)}
                  <button
                    disabled={isDeleting}
                    onClick={() => deleteSession(session.id)}
                  >
                    Delete
                  </button>
                </div>
              ))}
              {isFetchingSessions && <div>Fetching sessions...</div>}
              <div>
                <button
                  disabled={page <= 0}
                  onClick={() => setPage(page - 1)}
                >{`<<`}</button>
                <span>
                  {page + 1} / {sessions.num_pages}
                </span>
                <button
                  disabled={page + 1 >= sessions.num_pages}
                  onClick={() => setPage(page + 1)}
                >{`>>`}</button>
              </div>
            </div>
          </div>
        )}
        {!auth.isAuthenticated && (
          <div>
            <a href="#" onClick={() => navigate('/login')}>
              Login to view your permissions
            </a>
          </div>
        )}
    </div>
  )
}
