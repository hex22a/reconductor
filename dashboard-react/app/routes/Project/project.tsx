import { Project } from '~/components/Project/Project';
import type { Route } from './+types/project';
import { useLazyLoadQuery } from 'react-relay';
import type { ProjectQuery } from '~/__generated__/ProjectQuery.graphql';
import { query } from './ProjectQuery';
import { useParams } from 'react-router';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Project details' }, { name: 'description', content: 'Project' }];
}

export default function ProjectRoute() {
  const { id } = useParams();
  const data = useLazyLoadQuery<ProjectQuery>(query, { id: id! });
  if (!data.project) return <div>Project not found</div>;
  return <Project fragmentRef={data.project} />;
}
