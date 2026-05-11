import { Welcome } from '~/components/welcome/welcome';
import type { Route } from './+types/home';

export function meta({}: Route.MetaArgs) {
  return [
    { title: 'Reconductor' },
    { name: 'description', content: 'Reconductor network scanner' },
  ];
}

export default function Home() {
  return <Welcome />;
}
