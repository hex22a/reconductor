import { SignUp } from '~/components/SignUp/SignUp';
import type { Route } from './+types/signup';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Signup' }, { name: 'description', content: 'Signup to Reconductor' }];
}

export default function Home() {
  return <SignUp />;
}
