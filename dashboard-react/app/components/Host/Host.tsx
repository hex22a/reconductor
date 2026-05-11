import { useFragment } from 'react-relay';
import { fragment } from './Host.fragment';
import type { HostFragment$key } from '~/__generated__/HostFragment.graphql';
import { PortList } from '../PortList/PortList';

type HostProps = {
  fragmentRef: HostFragment$key;
};

export function Host({ fragmentRef }: HostProps) {
  const data = useFragment(fragment, fragmentRef);
  return (
    <>
      <div className="font-special">Host Details</div>
      <div>Host: {data.hostname}</div>
      <div>IP: {data.ip}</div>
      <PortList fragmentRef={data} />
    </>
  );
}
