import { ProjectList } from '~/components/ProjectList/ProjectList';
import { useLazyLoadQuery } from 'react-relay';
import { query } from './ProjectsQuery';
import type { Route } from './+types/projects';
import type { ProjectsQuery } from '~/__generated__/ProjectsQuery.graphql';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Projects list' }, { name: 'description', content: 'Your projects' }];
}

export default function ProjectsRoute() {
  const data = useLazyLoadQuery<ProjectsQuery>(query, {});
  return <ProjectList fragmentRef={data} />;
}
