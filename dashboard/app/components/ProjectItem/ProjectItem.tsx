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
    <tr>
      <td className="p-1">{createdAt.toUTCString()}</td>
      <td className="p-1">{data.name}</td>
      <td className="p-1">
        <NavLink to={`/project/${data.id}`}>details</NavLink>
      </td>
    </tr>
  );
}
