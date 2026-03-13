import type React from 'react';
import { useMutation } from 'react-relay';
import { CreateProjectMutation } from '~/graphql/mutations/CreateProjectMutation';

export function CreateProjectForm() {
  const [commit, isInFlight] = useMutation(CreateProjectMutation);
  function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const name = formData.get('name');
    commit({
      variables: {
        input: {
          name,
        },
      },

      onCompleted() {
        console.log('completed');
      },

      onError(err) {
        console.error(err);
      },
    });
  }
  return (
    <form onSubmit={handleSubmit}>
      <h1 className="font-special">Create project</h1>
      <label htmlFor="name">Project name:</label>
      <input className="border-b border-white p-2" id="name" name="name" type="text" />
      <button
        className="border border-white rounded-lg p-2 cursor-pointer hover:bg-white hover:text-black"
        type="submit"
        disabled={isInFlight}
      >
        Create
      </button>
    </form>
  );
}
