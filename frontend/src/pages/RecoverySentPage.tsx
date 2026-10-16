import { Link } from 'react-router-dom'

/**
 * Shown after every recovery request, whether or not the username and email matched an account,
 * so the page never reveals which accounts exist. The server only emails a reset link on a match.
 */
export const RecoverySentPage = () => {
  return (
    <div style={{ display: 'flex', justifyContent: 'center' }}>
    <div className="Form" style={{ textAlign: 'left', width: 650, maxWidth: '100%', marginLeft: '20px', marginRight: '20px' }}>
      <h1>Check Your Email</h1>
      <p>
        If the username and email address you entered belong to the same account, we've sent a link
        to reset your password to that email address.
      </p>
      <p>
        The link expires in 24 hours. If you don't see the email, check your spam or junk folder, or
        make sure you entered the username and email address exactly as they are on your account.
      </p>
      <Link
        style={{ marginTop: '30px' }}
        to="/recovery"
      >
        Didn't get the email? Try again.
      </Link>
      <Link
        style={{ marginTop: '30px' }}
        to="/login"
      >
        Back to login.
      </Link>
    </div>
    </div>
  )
}
