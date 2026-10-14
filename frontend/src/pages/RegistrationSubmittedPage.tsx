import { useLocation, useNavigate } from 'react-router-dom'

/** Shown after a successful registration: the account waits on the link in the activation email. */
export const RegistrationSubmittedPage = () => {
  const navigate = useNavigate()
  // The registration page passes the address along; a direct visit has none.
  const email = (useLocation().state as { email?: string } | null)?.email

  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h1>Check Your Email</h1>
      <p>
        Thanks for registering! We sent an activation link to {email ? <strong>{email}</strong> : 'your email address'}.
        Click the link in that email to activate your account, then log in.
      </p>
      <p>
        The link expires in 24 hours. If you don't see the email, check your spam or junk folder.
      </p>
      <a
        style={{ marginTop: '30px' }}
        href="#"
        onClick={() => navigate('/login')}
      >
        Already activated? Click here to login.
      </a>
    </div>
    </div>
  )
}
