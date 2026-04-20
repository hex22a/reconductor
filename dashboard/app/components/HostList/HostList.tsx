import { usePaginationFragment } from 'react-relay';
import type { HostListFragment$key } from '~/__generated__/HostListFragment.graphql';
import { fragment } from './HostList.fragment';
import { HostItem } from '../HostItem/HostItem';

type HostListProps = {
  fragmentRef: HostListFragment$key;
};

export function HostList({ fragmentRef }: HostListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Hosts</h1>
      {data.hosts.edges.map((edge) => (
        <HostItem key={edge.node.id} hostRef={edge.node} />
      ))}
    </div>
  );
}
