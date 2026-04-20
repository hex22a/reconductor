import { useParams } from 'react-router';
import type { Route } from './+types/scan';
import { useLazyLoadQuery } from 'react-relay';
import { query } from './ScanQuery';
import type { ScanQuery } from '~/__generated__/ScanQuery.graphql';
import { Scan } from '~/components/Scan/Scan';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Scan details' }, { name: 'description', content: 'Scan' }];
}

export default function ScanRoute() {
  const { id } = useParams();
  const data = useLazyLoadQuery<ScanQuery>(query, { id: id! });
  if (!data.scan) return <div>Scan not found</div>;
  return <Scan fragmentRef={data.scan} />;
}
