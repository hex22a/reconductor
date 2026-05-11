import { useParams } from 'react-router';
import type { Route } from './+types/run';
import { useLazyLoadQuery } from 'react-relay';
import { query } from './RunQuery';
import { ScanRun } from '~/components/ScanRun/ScanRun';
import type { RunQuery } from '~/__generated__/RunQuery.graphql';

export function meta({}: Route.MetaArgs) {
  return [{ title: 'Scan run details' }, { name: 'description', content: 'Scan run' }];
}

export default function RunRoute() {
  const { id } = useParams();
  const data = useLazyLoadQuery<RunQuery>(query, { id: id! });
  if (!data.run) return <div>Run not found</div>;
  return <ScanRun fragmentRef={data.run} />;
}
