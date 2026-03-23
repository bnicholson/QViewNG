import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export const MyProfilePermissionsPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  if (!auth.isAuthenticated) {
    return (
      <div>
        <a href="#" onClick={() => navigate('/login')}>Login to view your permissions</a>
      </div>
    )
  }

  return (
    <div style={{ textAlign: 'left' }}>
      <div id="roles">
        <div className="Form">
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
        <div className="Form">
          <h2>Permissions</h2>
          <pre>
            {!auth.session && <div>Error: No auth session present.</div>}
            {auth.session?.permissions?.map((perm) => (
              <div key={perm}>{perm}</div>
            ))}
            {auth.session?.permissions?.length === 0 && (
              <div>No permissions granted.</div>
            )}
          </pre>
        </div>
      </div>
    </div>
  )
}
