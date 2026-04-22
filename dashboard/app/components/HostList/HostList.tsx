import { usePaginationFragment } from 'react-relay';
import type { HostListFragment$key } from '~/__generated__/HostListFragment.graphql';
import { fragment } from './HostList.fragment';
import { Table } from '../Table/Table';
import { NavLink } from 'react-router';

type HostListProps = {
  fragmentRef: HostListFragment$key;
};

export function HostList({ fragmentRef }: HostListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Hosts</h1>
      <Table
        columns={[
          {
            key: 'ip',
            label: 'IP Address',
            render: (value: string | number | null | undefined, id: string) => (
              <NavLink to={`/host/${id}`}>{value}</NavLink>
            ),
          },
          { key: 'mac', label: 'Mac Address' },
          { key: 'hostname', label: 'Hostmane' },
          { key: 'vendor', label: 'Vendor' },
          { key: 'os_match', label: 'OS Match' },
          { key: 'os_accuracy', label: 'OS Accuracy' },
        ]}
        edges={data.hosts.edges}
      />
    </div>
  );
}
