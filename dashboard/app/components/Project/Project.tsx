import { useFragment } from 'react-relay';
import type { ProjectFragment$key } from '~/__generated__/ProjectFragment.graphql';
import { fragment } from './Project.fragment';

type ProjectProps = {
  fragmentRef: ProjectFragment$key;
};

export function Project({ fragmentRef }: ProjectProps) {
  const { name, created_at } = useFragment(fragment, fragmentRef);
  const date = new Date(parseInt(created_at)).toISOString();
  return (
    <>
      <div>Project details</div>
      <div>Name: {name}</div>
      <div>Created At: {date}</div>
    </>
  );
}
