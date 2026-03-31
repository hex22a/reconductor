import { usePaginationFragment } from 'react-relay';
import { CreateProjectForm } from '../CreateProjectForm/CreateProjectForm';
import { ProjectItem } from '../ProjectItem/ProjectItem';
import type { ProjectListFragment$key } from '~/__generated__/ProjectListFragment.graphql';
import { fragment } from './ProjectList.fragment';

type ProjectsProps = {
  fragmentRef: ProjectListFragment$key;
};

export function ProjectList({ fragmentRef }: ProjectsProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <div className="w-3xl">
      <CreateProjectForm />
      <div className="my-2">
        <h1 className="font-special">Projects</h1>
        {data.projects.edges.map((edge) => (
          <ProjectItem key={edge.node.id} projectRef={edge.node} />
        ))}
      </div>
    </div>
  );
}
