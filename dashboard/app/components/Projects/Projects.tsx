import { usePaginationFragment } from 'react-relay';
import { CreateProjectForm } from '../CreateProjectForm/CreateProjectForm';
import { ProjectItem } from '../ProjectItem/ProjectItem';
import type { ProjectsListFragment$key } from '~/__generated__/ProjectsListFragment.graphql';
import { fragment } from './Projects.fragment';

type ProjectsProps = {
  fragmentRef: ProjectsListFragment$key;
};

export function Projects({ fragmentRef }: ProjectsProps) {
  const { data } = usePaginationFragment(fragment, fragmentRef);
  return (
    <>
      <h1 className="font-special">Projects</h1>
      {data.projects.edges.map((edge) => (
        <ProjectItem key={edge.node.id} projectRef={edge.node} />
      ))}
      <CreateProjectForm />
    </>
  );
}
