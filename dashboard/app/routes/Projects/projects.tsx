import { Projects } from '~/components/Projects/Projects';
import { useLazyLoadQuery } from 'react-relay';
import { query } from './ProjectsQuery';
import type { Route } from './+types/projects';
import type { ProjectsQuery } from '~/__generated__/ProjectsQuery.graphql';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Signin' }, { name: 'description', content: 'Signin to Reconductor' }];
}

export default function ProjectsRoute() {
  const data = useLazyLoadQuery<ProjectsQuery>(query, {});
  return <Projects fragmentRef={data} />;
}
