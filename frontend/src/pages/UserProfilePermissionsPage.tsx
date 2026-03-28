import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

interface RolesAndPermissions {
  roles: string[]
  permissions: string[]
}

export const UserProfilePermissionsPage = ({ userId }: { userId: string }) => {
  const auth = useAuth()
  const navigate = useNavigate()

  const [data, setData] = useState<RolesAndPermissions | null>(null)
  const [loadError, setLoadError] = useState<string | null>(null)

  useEffect(() => {
    if (!auth.isAuthenticated) return

    fetch(`/api/users/${userId}/roles-and-permissions`, {
      headers: { Authorization: `Bearer ${auth.accessToken}` },
    })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load permissions (${r.status})`)
        return r.json()
      })
      .then((d: RolesAndPermissions) => setData(d))
      .catch((e) => setLoadError(e.message))
  }, [auth.isAuthenticated, userId])

  if (!auth.isAuthenticated) {
    return (
      <div>
        <a href="#" onClick={() => navigate('/login')}>Login to view permissions</a>
      </div>
    )
  }

  if (loadError) return <div style={{ color: 'red' }}>{loadError}</div>
  if (!data) return <div>Loading...</div>

  return (
    <div style={{ textAlign: 'left' }}>
      <div id="roles">
        <div className="Form">
          <h2>Roles</h2>
          <pre>
            {data.roles.length
              ? data.roles.map((role) => <div key={role}>{role}</div>)
              : <div>No roles assigned.</div>
            }
          </pre>
        </div>
      </div>

      <div id="permissions" style={{ marginTop: '32px' }}>
        <div className="Form">
          <h2>Permissions</h2>
          <pre>
            {data.permissions.length
              ? data.permissions.map((perm) => <div key={perm}>{perm}</div>)
              : <div>No permissions granted.</div>
            }
          </pre>
        </div>
      </div>
    </div>
  )
}
