import { useLazyLoadQuery } from 'react-relay';
import { CreateProjectForm } from '../CreateProjectForm/CreateProjectForm';
import { query } from '~/routes/Projects/ProjectsQuery';
import { ProjectItem } from '../ProjectItem/ProjectItem';
import type { ProjectsQuery } from '~/__generated__/ProjectsQuery.graphql';

export function Projects() {
  const data = useLazyLoadQuery<ProjectsQuery>(query, {});
  return (
    <>
      <h1 className="font-special">Projects</h1>
      {data.projects.map((project) => (
        <ProjectItem key={project.id} projectRef={project} />
      ))}
      <CreateProjectForm />
    </>
  );
}
