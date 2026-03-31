import { usePaginationFragment } from 'react-relay';
import type { ScanListFragment$key } from '~/__generated__/ScanListFragment.graphql';
import { fragment } from './ScanList.fragment';
import { ScanItem } from '../ScanItem/ScanItem';

type ScanListProps = {
  fragmentRef: ScanListFragment$key;
};
export function ScanList({ fragmentRef }: ScanListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Scans</h1>
      {data.scans.edges.map((edge) => (
        <ScanItem key={edge.node.id} scanRef={edge.node} />
      ))}
    </div>
  );
}
