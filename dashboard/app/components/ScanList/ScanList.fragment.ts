import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ScanListFragment on Project
    @argumentDefinitions(first: { type: "Int", defaultValue: 15 }, after: { type: "String" })
    @refetchable(queryName: "ScanPaginationQuery") {
        scans(first: $first, after: $after) @connection(key: "ScanList_scans") {
            edges {
                node {
                    id
                    created_at
                    target
                    status
                    schedule
                }
            }
        }
    }
`;
