import { usePaginationFragment } from 'react-relay';
import { fragment } from './PortList.fragment';
import type { PortListFragment$key } from '~/__generated__/PortListFragment.graphql';
import { PortItem } from '../PortItem/PortItem';

type PortListProps = {
  fragmentRef: PortListFragment$key;
};

export function PortList({ fragmentRef }: PortListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Ports</h1>
      {data.ports.edges.map((edge) => (
        <PortItem key={edge.node.id} portRef={edge.node} />
      ))}
    </div>
  );
}
