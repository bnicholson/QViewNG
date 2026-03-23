import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export const MyProfileOverviewPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  return (
    <div style={{ textAlign: 'left' }}>
      {auth.isAuthenticated && (
        <div id="overview">
          <h2>Overview</h2>
          <p>User # {auth.session?.userId}</p>
        </div>
      )}
      {!auth.isAuthenticated && (
        <div>
          <a href="#" onClick={() => navigate('/login')}>
            Login to view your account details
          </a>
        </div>
      )}
    </div>
  )
}
