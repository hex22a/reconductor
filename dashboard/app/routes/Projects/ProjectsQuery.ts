import { graphql } from 'react-relay';

export const query = graphql`
    query ProjectsQuery {
        projects {
            id
            ...ProjectItemFragment
        }
    }
`;
