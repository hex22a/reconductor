import { usePaginationFragment } from 'react-relay';
import { fragment } from './PortList.fragment';
import type { PortListFragment$key } from '~/__generated__/PortListFragment.graphql';
import { Table } from '../Table/Table';

type PortListProps = {
  fragmentRef: PortListFragment$key;
};

export function PortList({ fragmentRef }: PortListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Ports</h1>
      <Table
        columns={[
          { key: 'port', label: 'Port #' },
          { key: 'protocol', label: 'Protocol' },
          { key: 'state', label: 'State' },
          { key: 'service', label: 'Service' },
          { key: 'product', label: 'Product' },
          { key: 'version', label: 'Version' },
        ]}
        edges={data.ports.edges}
      />
    </div>
  );
}
