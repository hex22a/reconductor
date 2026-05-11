import { usePaginationFragment } from 'react-relay';
import { CreateProjectForm } from '../CreateProjectForm/CreateProjectForm';
import type { ProjectListFragment$key } from '~/__generated__/ProjectListFragment.graphql';
import { fragment } from './ProjectList.fragment';
import { Table } from '../Table/Table';
import { NavLink } from 'react-router';

type ProjectsProps = {
  fragmentRef: ProjectListFragment$key;
};

export function ProjectList({ fragmentRef }: ProjectsProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <CreateProjectForm />
      <h1 className="font-special">Projects</h1>
      <Table
        columns={[
          {
            key: 'name',
            label: 'Name',
            render: (value: string, id: string) => <NavLink to={`/project/${id}`}>{value}</NavLink>,
          },
          {
            key: 'created_at',
            label: 'Created At',
            render: (value: string) => new Date(parseInt(value)).toUTCString(),
          },
        ]}
        edges={data.projects.edges}
      />
    </div>
  );
}
