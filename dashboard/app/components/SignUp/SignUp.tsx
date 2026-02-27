import type React from 'react';

export function SignUp() {
  async function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const username = formData.get('username');
    const password = formData.get('password');
    const res = await fetch('http://localhost:4000/api/v1/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ username, password }),
    });
    console.log(res);
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="w-150 flex flex-col gap-3 border border-white rounded-xl p-3"
    >
      <h1 className="font-special">Signup</h1>
      <label htmlFor="username">Username:</label>
      <input className="border-b border-white p-2" id="username" name="username" type="text" />
      <label htmlFor="password">Password</label>
      <input className="border-b border-white p-2" id="password" name="password" type="password" />
      <button
        className="border border-white rounded-lg p-2 cursor-pointer hover:bg-white hover:text-black"
        type="submit"
      >
        Register
      </button>
    </form>
  );
}
