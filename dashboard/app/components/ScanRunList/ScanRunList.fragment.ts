import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ScanRunListFragment on Scan
    @argumentDefinitions(first: { type: "Int", defaultValue: 15 }, after: { type: "String" })
    @refetchable(queryName: "ScanRunPaginationQuery") {
        runs(first: $first, after: $after) @connection(key: "ScanRunList_runs") {
            edges {
                node {
                    id
                    ...ScanRunItemFragment
                }
            }
        }
    }
`;
