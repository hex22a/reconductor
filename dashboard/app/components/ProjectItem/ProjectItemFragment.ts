import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment ProjectItemFragment on Project {
        name
        created_at
    }
`;
