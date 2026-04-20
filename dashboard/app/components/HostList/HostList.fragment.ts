import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment HostListFragment on ScanRun
    @argumentDefinitions(first: { type: "Int", defaultValue: 15 }, after: { type: "String" })
    @refetchable(queryName: "HostPaginationQuery") {
        hosts(first: $first, after: $after) @connection(key: "HostList_hosts") {
            edges {
                node {
                    id
                    ...HostItemFragment
                }
            }
        }
    }
`;
