import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ProjectsListFragment on Query
    @argumentDefinitions(first: { type: "Int", defaultValue: 15 }, after: { type: "String" })
    @refetchable(queryName: "ProjectsPaginationQuery") {
        projects(first: $first, after: $after) @connection(key: "ProjectsList_projects") {
            edges {
                node {
                    id
                    ...ProjectItemFragment
                }
            }
        }
    }
`;
