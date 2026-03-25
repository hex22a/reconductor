import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment ProjectItemFragment on Project {
        id
        name
        created_at
    }
`;
