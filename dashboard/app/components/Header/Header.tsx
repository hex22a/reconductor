import { useEffect, useState } from 'react';
import { NavLink } from 'react-router';
import { API_ME_URL } from '~/constants';
export function Header() {
  const [username, setUsername] = useState<string | null>(null);
  useEffect(() => {
    fetch(API_ME_URL, { credentials: 'include' }).then((res) => {
      res.json().then((responseJson) => {
        setUsername(responseJson.username);
      });
    });
  }, []);
  return (
    <header className="sticky top-0">
      {username ? (
        <div>{username}</div>
      ) : (
        <nav>
          <NavLink to="/signup">Sign up</NavLink>/<NavLink to="/signin">Sign in</NavLink>
        </nav>
      )}
    </header>
  );
}
