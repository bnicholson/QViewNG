import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

interface UserProfile {
  id: string
  username: string | null
  email: string
  fname: string
  mname: string
  lname: string
  created_at: string
  updated_at: string
}

const formatDate = (iso: string) =>
  new Date(iso).toLocaleString(undefined, {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit',
  })

export const UserProfileOverviewPage = ({ userId }: { userId: string }) => {
  const auth = useAuth()
  const navigate = useNavigate()

  const [profile, setProfile] = useState<UserProfile | null>(null)
  const [loadError, setLoadError] = useState<string | null>(null)

  const [fname, setFname] = useState('')
  const [mname, setMname] = useState('')
  const [lname, setLname] = useState('')
  const [email, setEmail] = useState('')

  const [saving, setSaving] = useState(false)
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saveSuccess, setSaveSuccess] = useState<string | null>(null)

  useEffect(() => {
    if (!auth.isAuthenticated) return

    fetch(`/api/users/${userId}`, {
      headers: { Authorization: `Bearer ${auth.accessToken}` },
    })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load profile (${r.status})`)
        return r.json()
      })
      .then((data: UserProfile) => {
        setProfile(data)
        setFname(data.fname)
        setMname(data.mname)
        setLname(data.lname)
        setEmail(data.email)
      })
      .catch((e) => setLoadError(e.message))
  }, [auth.isAuthenticated, userId])

  const save = async () => {
    setSaving(true)
    setSaveError(null)
    setSaveSuccess(null)

    const response = await fetch(`/api/users/${userId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify({ fname, mname, lname, email }),
    })

    const body = await response.json().catch(() => ({}))
    setSaving(false)

    if (response.ok) {
      const updated: UserProfile = body.item ?? body
      setProfile(updated)
      setSaveSuccess('Profile updated successfully.')
    } else {
      setSaveError(body.error ?? body.message ?? 'Failed to save profile.')
    }
  }

  if (!auth.isAuthenticated) {
    return (
      <div>
        <a href="#" onClick={() => navigate('/login')}>
          Login to view account details
        </a>
      </div>
    )
  }

  if (loadError) return <div style={{ color: 'red' }}>{loadError}</div>
  if (!profile) return <div>Loading...</div>

  return (
    <div id="overview" style={{ textAlign: 'left' }}>
      <h2>Overview</h2>
      <div className="Form" style={{ display: 'flex', flexFlow: 'column', gap: '12px', maxWidth: '400px' }}>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Username</label>
          <input type="text" value={profile.username ?? ''} disabled style={{ opacity: 0.6 }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>First Name</label>
          <input type="text" value={fname} onChange={(e) => { setFname(e.target.value); setSaveSuccess(null) }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Middle Name</label>
          <input type="text" value={mname} onChange={(e) => { setMname(e.target.value); setSaveSuccess(null) }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Last Name</label>
          <input type="text" value={lname} onChange={(e) => { setLname(e.target.value); setSaveSuccess(null) }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Email</label>
          <input type="email" value={email} onChange={(e) => { setEmail(e.target.value); setSaveSuccess(null) }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Date Created</label>
          <input type="text" value={formatDate(profile.created_at)} disabled style={{ opacity: 0.6 }} />
        </div>
        <div style={{ display: 'flex', flexFlow: 'column' }}>
          <label>Last Modified</label>
          <input type="text" value={formatDate(profile.updated_at)} disabled style={{ opacity: 0.6 }} />
        </div>
        {saveError && <span style={{ color: 'red', fontSize: '0.85em' }}>{saveError}</span>}
        {saveSuccess && <span style={{ color: 'green', fontSize: '0.85em' }}>{saveSuccess}</span>}
        <div>
          <button disabled={saving} onClick={save}>
            {saving ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  )
}
