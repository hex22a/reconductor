import { Project } from '~/components/Project/Project';
import type { Route } from './+types/project';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Project details' }, { name: 'description', content: 'Project' }];
}

export default function ProjectRoute() {
  return <Project />;
}
