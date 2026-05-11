import { usePaginationFragment } from 'react-relay';
import type { ScanListFragment$key } from '~/__generated__/ScanListFragment.graphql';
import { fragment } from './ScanList.fragment';
import { Table } from '../Table/Table';
import { NavLink } from 'react-router';

type ScanListProps = {
  fragmentRef: ScanListFragment$key;
};
export function ScanList({ fragmentRef }: ScanListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Scans</h1>
      <Table
        columns={[
          {
            key: 'target',
            label: 'Target',
            render: (value: string | null | undefined, id: string) => (
              <NavLink to={`/scan/${id}`}>{value}</NavLink>
            ),
          },
          {
            key: 'status',
            label: 'Status',
          },
          { key: 'schedule', label: 'Schedule' },
          {
            key: 'created_at',
            label: 'Created At',
            render: (value: string | null | undefined) => new Date(parseInt(value!)).toUTCString(),
          },
        ]}
        edges={data.scans.edges}
      />
    </div>
  );
}
