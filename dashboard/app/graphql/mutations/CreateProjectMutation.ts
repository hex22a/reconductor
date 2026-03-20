import { graphql } from 'react-relay';

export const CreateProjectMutation = graphql`
    mutation CreateProjectMutation($input: CreateProjectInput!, $connections: [ID!]!) {
        createProject(input: $input) @appendEdge(connections: $connections) {
            cursor
            node {
                id
                name
                created_at
            }
        }
    }
`;
