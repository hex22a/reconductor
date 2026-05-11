import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ScanFragment on Scan {
        target
        created_at
        status
        schedule
        ...ScanRunListFragment
    }
`;
