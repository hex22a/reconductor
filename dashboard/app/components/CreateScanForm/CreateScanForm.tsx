import { useRef } from 'react';
import { ConnectionHandler, useMutation } from 'react-relay';
import { useParams } from 'react-router';
import { CreateScanMutation } from '~/graphql/mutations/CreateScanMutation';

export function CreateScanForm() {
  const { id: projectId } = useParams();
  const [commit, isInFlight] = useMutation(CreateScanMutation);
  const targetRef = useRef<HTMLInputElement>(null);
  const scheduleRef = useRef<HTMLInputElement>(null);
  function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    const form = e.currentTarget as HTMLFormElement;
    const formData = new FormData(form);
    const connectionId = ConnectionHandler.getConnectionID(projectId!, 'ScanList_scans');
    const target = formData.get('target');
    const schedule = formData.get('schedule');
    commit({
      variables: {
        input: { projectId, target, schedule },
        connections: [connectionId],
      },

      onCompleted() {
        if (targetRef.current) targetRef.current.value = '';
        if (scheduleRef.current) scheduleRef.current.value = '';
      },

      onError(err) {
        console.error(err);
      },
    });
  }
  return (
    <form onSubmit={handleSubmit}>
      <h1 className="font-special">Create scan</h1>
      <fieldset>
        <label htmlFor="target">Target:</label>
        <input
          className="border-b border-white p-2"
          id="target"
          name="target"
          typeof="text"
          placeholder="192.168.50.0/16"
          ref={targetRef}
        />
        <label htmlFor="schedule">Schedule:</label>
        <input
          className="border-b border-white p-2"
          id="schedule"
          name="schedule"
          typeof="text"
          placeholder="10 5 * * *"
          ref={scheduleRef}
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
