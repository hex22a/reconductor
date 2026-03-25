import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment ProjectFragment on Project {
        name
        created_at
    }
`;
