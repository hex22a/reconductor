import { useFragment } from 'react-relay';
import type { ScanRunFragment$key } from '~/__generated__/ScanRunFragment.graphql';
import { fragment } from './ScanRun.fragment';
import { HostList } from '../HostList/HostList';

type ScanRunProps = {
  fragmentRef: ScanRunFragment$key;
};

export function ScanRun({ fragmentRef }: ScanRunProps) {
  const data = useFragment(fragment, fragmentRef);
  const createdAt = new Date(parseInt(data.created_at));
  return (
    <>
      <div className="font-special">Scan Run Details</div>
      <div>Created At: {createdAt.toISOString()}</div>
      <HostList fragmentRef={data} />
    </>
  );
}
