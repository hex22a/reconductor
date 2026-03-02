import { NavLink } from 'react-router';
export function Header() {
  return (
    <header className="sticky top-0">
      <nav>
        <NavLink to="/signup">Sign up</NavLink>/<NavLink to="/signin">Sign in</NavLink>
      </nav>
    </header>
  );
}
