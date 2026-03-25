import { useFragment } from 'react-relay';
import { fragment } from './ProjectItem.fragment';
import type { ProjectItemFragment$key } from '~/__generated__/ProjectItemFragment.graphql';
import { NavLink } from 'react-router';

type ProjectItemProps = {
  projectRef: ProjectItemFragment$key;
};

export function ProjectItem({ projectRef }: ProjectItemProps) {
  const data = useFragment(fragment, projectRef);
  const createdAt = new Date(parseInt(data.created_at));

  return (
    <div className="my-3 flex gap-3">
      <span>{createdAt.toUTCString()}</span>
      <span>{data.name}</span>
      <NavLink to={`/project/${data.id}`}>details</NavLink>
    </div>
  );
}
