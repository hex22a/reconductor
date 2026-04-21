import { useParams } from 'react-router';
import type { Route } from './+types/host';
import { useLazyLoadQuery } from 'react-relay';
import { query } from './HostQuery';
import type { HostQuery } from '~/__generated__/HostQuery.graphql';
import { Host } from '~/components/Host/Host';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Host details' }, { name: 'description', content: 'Host' }];
}

export default function HostRoute() {
  const { id } = useParams();
  const data = useLazyLoadQuery<HostQuery>(query, { id: id! });
  if (!data.host) return <div>Run not found</div>;
  return <Host fragmentRef={data.host} />;
}
