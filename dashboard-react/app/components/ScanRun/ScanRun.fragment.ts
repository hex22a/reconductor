import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ScanRunFragment on ScanRun {
        created_at
        ...HostListFragment
    }
`;
