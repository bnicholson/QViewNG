import { useAuth } from '../hooks/useAuth';
import { Link } from 'react-router-dom';

export const AuthStatusPage = () => {
  const auth = useAuth();
  return (
    <div style={{ padding: 24 }}>
      <h1>Auth Status</h1>
      {auth.isAuthenticated ? (
        <div>
          <p style={{ color: 'green' }}>Logged in</p>
          <p>User ID: {auth.session?.userId}</p>
          <p>Roles: {auth.session?.roles?.join(', ') || 'none'}</p>
          <p>Permissions: {auth.session?.permissions?.join(', ') || 'none'}</p>
          <button onClick={() => auth.logout()}>Logout</button>
        </div>
      ) : (
        <div>
          <p style={{ color: 'red' }}>Not logged in</p>
          <Link to="/login">Go to Login</Link>
        </div>
      )}
    </div>
  );
};
