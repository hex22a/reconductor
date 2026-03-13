import type React from 'react';
import { useState } from 'react';
import { API_REGISTER_URL } from '~/constants';
import { FormError } from '../FormError/FormError';
import { useNavigate } from 'react-router';
import {
  VALIDATION_ERROR_CODE,
  SYNTAX_ERROR_CODE,
  DATABASE_ERROR_CODE,
  UNEXPECTED_ERROR_CODE,
} from '$/constants';

export function SignUp() {
  const navigate = useNavigate();
  const [genericError, setGenericError] = useState<string | null>(null);
  const [usernameError, setUsernameError] = useState<string | null>(null);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  function dismissGenericError() {
    setGenericError(null);
  }
  function dismissUsernameError() {
    setUsernameError(null);
  }
  function dismissPasswordError() {
    setPasswordError(null);
  }
  async function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const username = formData.get('username');
    const password = formData.get('password');
    try {
      const res = await fetch(API_REGISTER_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
      });
      const result = await res.json();
      switch (result.code) {
        case VALIDATION_ERROR_CODE:
          if (result.error.fieldErrors.username) {
            setUsernameError(result.error.fieldErrors.username[0]);
          }
          if (result.error.fieldErrors.password) {
            setPasswordError(result.error.fieldErrors.password[0]);
          }
          break;
        case DATABASE_ERROR_CODE:
        case UNEXPECTED_ERROR_CODE:
        case SYNTAX_ERROR_CODE:
          setGenericError(result.error);
          break;
        default:
          navigate('/signin');
      }
    } catch (err) {
      setGenericError('Network error');
    }
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="w-150 flex flex-col gap-3 border border-white rounded-xl p-3"
    >
      <h1 className="font-special">Signup</h1>
      <label htmlFor="username">Username:</label>
      <input className="border-b border-white p-2" id="username" name="username" type="text" />
      {usernameError && <FormError error={usernameError} onClose={dismissUsernameError} />}
      <label htmlFor="password">Password</label>
      <input className="border-b border-white p-2" id="password" name="password" type="password" />
      {passwordError && <FormError error={passwordError} onClose={dismissPasswordError} />}
      <button
        className="border border-white rounded-lg p-2 cursor-pointer hover:bg-white hover:text-black"
        type="submit"
      >
        Register
      </button>
      {genericError && <FormError error={genericError} onClose={dismissGenericError} />}
    </form>
  );
}
