import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment PortListFragment on Host
    @argumentDefinitions(first: { type: "Int", defaultValue: 15 }, after: { type: "String" })
    @refetchable(queryName: "PortPaginationQuery") {
        ports(first: $first, after: $after) @connection(key: "PortList_ports") {
            edges {
                node {
                    id
                    ...PortItemFragment
                }
            }
        }
    }
`;
