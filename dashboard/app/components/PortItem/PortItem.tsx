import { useFragment } from 'react-relay';
import { fragment } from './PortItem.fragment';
import type { PortItemFragment$key } from '~/__generated__/PortItemFragment.graphql';

type PortItemProps = {
  portRef: PortItemFragment$key;
};

export function PortItem({ portRef }: PortItemProps) {
  const data = useFragment(fragment, portRef);

  return (
    <div className="my-3 flex gap-3">
      <span>{data.port}</span>
      <span>{data.service}</span>
      <span>{data.state}</span>
      <span>{data.protocol}</span>
      <span>{data.product}</span>
      <span>{data.version}</span>
    </div>
  );
}
