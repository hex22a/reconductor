import { graphql } from 'react-relay';

export const query = graphql`
    query ProjectQuery($id: ID!) {
        project(id: $id) {
            ...ProjectFragment
        }
    }
`;
