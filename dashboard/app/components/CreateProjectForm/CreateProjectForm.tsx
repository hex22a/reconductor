import type React from 'react';
import { useRef } from 'react';
import { ConnectionHandler, useMutation } from 'react-relay';
import { CreateProjectMutation } from '~/graphql/mutations/CreateProjectMutation';

export function CreateProjectForm() {
  const [commit, isInFlight] = useMutation(CreateProjectMutation);
  const nameRef = useRef<HTMLInputElement>(null);
  function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const connectionId = ConnectionHandler.getConnectionID('root', 'ProjectsList_projects');
    const name = formData.get('name');
    commit({
      variables: {
        input: { name },
        connections: [connectionId],
      },

      onCompleted() {
        if (nameRef.current) nameRef.current.value = '';
      },

      onError(err) {
        console.error(err);
      },
    });
  }
  return (
    <form onSubmit={handleSubmit}>
      <h1 className="font-special">Create project</h1>
      <fieldset className="flex gap-3 items-center w-full">
        <label htmlFor="name">Project name:</label>
        <input
          className="border-b border-white p-2 flex-1"
          id="name"
          name="name"
          type="text"
          ref={nameRef}
        />
        <button
          className="border border-white rounded-lg p-2 cursor-pointer hover:bg-white hover:text-black"
          type="submit"
          disabled={isInFlight}
        >
          Create
        </button>
      </fieldset>
    </form>
  );
}
