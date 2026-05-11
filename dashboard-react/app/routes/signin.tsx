import { SignIn } from '~/components/SignIn/SignIn';
import type { Route } from './+types/signin';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Signin' }, { name: 'description', content: 'Signin to Reconductor' }];
}

export default function SigninRoute() {
  return <SignIn />;
}
