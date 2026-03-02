import { Projects } from '~/components/Projects/Projects';
import type { Route } from './+types/projects';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Signin' }, { name: 'description', content: 'Signin to Reconductor' }];
}

export default function ProjectsRoute() {
  return <Projects />;
}
