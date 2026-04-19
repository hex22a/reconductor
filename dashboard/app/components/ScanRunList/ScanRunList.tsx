import { usePaginationFragment } from 'react-relay';
import type { ScanRunListFragment$key } from '~/__generated__/ScanRunListFragment.graphql';
import { fragment } from './ScanRunList.fragment';
import { ScanRunItem } from '../ScanRunItem/ScanRunItem';

type ScanRunListProps = {
  fragmentRef: ScanRunListFragment$key;
};

export function ScanRunList({ fragmentRef }: ScanRunListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Runs</h1>
      {data.runs.edges.map((edge) => (
        <ScanRunItem key={edge.node.id} scanRunRef={edge.node} />
      ))}
    </div>
  );
}
