import { useFragment } from 'react-relay';
import type { ProjectFragment$key } from '~/__generated__/ProjectFragment.graphql';
import { fragment } from './Project.fragment';
import { CreateScanForm } from '../CreateScanForm/CreateScanForm';
import { ScanList } from '../ScanList/ScanList';

type ProjectProps = {
  fragmentRef: ProjectFragment$key;
};

export function Project({ fragmentRef }: ProjectProps) {
  const data = useFragment(fragment, fragmentRef);
  const date = new Date(parseInt(data.created_at)).toISOString();
  return (
    <>
      <div className="font-special">Project details</div>
      <div>Name: {data.name}</div>
      <div>Created At: {date}</div>
      <CreateScanForm />
      <ScanList fragmentRef={data} />
    </>
  );
}
