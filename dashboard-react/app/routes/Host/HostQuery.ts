import { graphql } from 'react-relay';

export const query = graphql`
    query HostQuery($id: ID!) {
        host(id: $id) {
            ...HostFragment
        }
    }
`;
