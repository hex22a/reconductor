import { useFragment } from 'react-relay';
import type { HostItemFragment$key } from '~/__generated__/HostItemFragment.graphql';
import { fragment } from './HostItem.fragment';

type HostItemProps = {
  hostRef: HostItemFragment$key;
};

export function HostItem({ hostRef }: HostItemProps) {
  const data = useFragment(fragment, hostRef);

  return (
    <div className="my-3 flex gap-3">
      <span>{data.ip}</span>
      <span>{data.hostname}</span>
      <span>{data.vendor}</span>
      <span>{data.os_match}</span>
      <span>{data.os_accuracy}</span>
      <span>{data.mac}</span>
    </div>
  );
}
