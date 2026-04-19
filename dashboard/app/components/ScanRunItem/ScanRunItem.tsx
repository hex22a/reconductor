import { useFragment } from 'react-relay';
import type { ScanRunItemFragment$key } from '~/__generated__/ScanRunItemFragment.graphql';
import { fragment } from './ScanRunItem.fragment';

type ScanRunItemProps = {
  scanRunRef: ScanRunItemFragment$key;
};

export function ScanRunItem({ scanRunRef }: ScanRunItemProps) {
  const data = useFragment(fragment, scanRunRef);
  const createdAt = new Date(parseInt(data.created_at));

  return (
    <div className="my-3 flex gap-3">
      <span>${createdAt.toUTCString()}</span>
    </div>
  );
}
