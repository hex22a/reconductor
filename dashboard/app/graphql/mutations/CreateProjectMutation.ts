import { graphql } from 'react-relay';

export const CreateProjectMutation = graphql`
    mutation CreateProjectMutation($input: CreateProjectInput!) {
        createProject(input: $input) {
            id
            name
            created_at
        }
    }
`;
