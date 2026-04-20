import { useFragment } from 'react-relay';
import { fragment } from './Scan.fragmemt';
import type { ScanFragment$key } from '~/__generated__/ScanFragment.graphql';
import { ScanRunList } from '../ScanRunList/ScanRunList';

type ScanProps = {
  fragmentRef: ScanFragment$key;
};

export function Scan({ fragmentRef }: ScanProps) {
  const data = useFragment(fragment, fragmentRef);
  const date = new Date(parseInt(data.created_at)).toISOString();
  return (
    <>
      <div className="font-special">Scan Details</div>
      <div>Target: {data.target}</div>
      <div>Schedle: {data.schedule}</div>
      <div>Created At: {date}</div>
      <ScanRunList fragmentRef={data} />
    </>
  );
}
