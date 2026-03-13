import { NavLink, useNavigate } from 'react-router';
import { useAuth } from '~/providers/AuthProvider';
export function Header() {
  const navigate = useNavigate();
  const { user, logout } = useAuth();

  async function handleLogout() {
    await logout();
    navigate('/');
  }

  return (
    <header className="sticky top-0">
      {user?.username ? (
        <nav>
          <span>$: {user?.username}</span>
          <span>
            <button type="button" onClick={handleLogout}>
              (Logout)
            </button>
          </span>
        </nav>
      ) : (
        <nav>
          <NavLink to="/signup">Sign up</NavLink>/<NavLink to="/signin">Sign in</NavLink>
        </nav>
      )}
    </header>
  );
}
