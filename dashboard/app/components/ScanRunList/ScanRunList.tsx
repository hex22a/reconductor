import { usePaginationFragment } from 'react-relay';
import type { ScanRunListFragment$key } from '~/__generated__/ScanRunListFragment.graphql';
import { fragment } from './ScanRunList.fragment';
import { Table } from '../Table/Table';
import { NavLink } from 'react-router';

type ScanRunListProps = {
  fragmentRef: ScanRunListFragment$key;
};

export function ScanRunList({ fragmentRef }: ScanRunListProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <h1 className="font-special">Runs</h1>
      <Table
        columns={[
          {
            key: 'created_at',
            label: 'Created At',
            render: (value: string, id: string) => (
              <NavLink to={`/run/${id}`}>{new Date(parseInt(value)).toUTCString()}</NavLink>
            ),
          },
        ]}
        edges={data.runs.edges}
      />
    </div>
  );
}
