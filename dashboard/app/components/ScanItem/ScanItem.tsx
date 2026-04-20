import { useFragment } from 'react-relay';
import type { ScanItemFragment$key } from '~/__generated__/ScanItemFragment.graphql';
import { fragment } from './ScanItem.fragment';
import { NavLink } from 'react-router';

type ScanItemProps = {
  scanRef: ScanItemFragment$key;
};
export function ScanItem({ scanRef }: ScanItemProps) {
  const data = useFragment(fragment, scanRef);
  const createdAt = new Date(parseInt(data.created_at));

  return (
    <div className="my-3 flex gap-3">
      <span>{createdAt.toUTCString()}</span>
      <span>{data.target}</span>
      <span>{data.status}</span>
      <span>{data.schedule}</span>
      <NavLink to={`/scan/${data.id}`}>details</NavLink>
    </div>
  );
}
