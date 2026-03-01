import { NavLink, Outlet } from 'react-router';

export default function MainLayout() {
  return (
    <div className="flex flex-col items-center min-h-screen">
      <header className="sticky top-0">
        <nav>
          <NavLink to="/signup">Sign up</NavLink>/<NavLink to="/signin">Sign in</NavLink>
        </nav>
      </header>
      <main className="flex flex-col justify-center flex-1">
        <Outlet />
      </main>
      <footer>
        Crafted.
        <a href="https://github.com/hex22a/reconductor">Reconductor monorepo</a>
      </footer>
    </div>
  );
}
