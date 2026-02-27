import { Form } from 'react-router';

export function SignIn() {
  return (
    <Form className="w-150 flex flex-col gap-3 border border-white rounded-xl p-3">
      <h1 className="font-special">Sign in</h1>
      <label htmlFor="username">Username:</label>
      <input className="border-b border-white p-2" id="username" name="username" type="text" />
      <label htmlFor="password">Password</label>
      <input className="border-b border-white p-2" id="password" name="password" type="password" />
      <button
        className="border border-white rounded-lg p-2 cursor-pointer hover:bg-white hover:text-black"
        type="submit"
      >
        Sign in
      </button>
    </Form>
  );
}
