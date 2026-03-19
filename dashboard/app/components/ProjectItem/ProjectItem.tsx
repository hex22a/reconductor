import { useFragment } from 'react-relay';
import { fragment } from './ProjectItemFragment';
import type { ProjectItemFragment$key } from '~/__generated__/ProjectItemFragment.graphql';

type ProjectItemProps = {
  projectRef: ProjectItemFragment$key;
};

export function ProjectItem({ projectRef }: ProjectItemProps) {
  const data = useFragment(fragment, projectRef);

  return (
    <div>
      <span>{data.name}</span>
      <span>{data.created_at}</span>
    </div>
  );
}
