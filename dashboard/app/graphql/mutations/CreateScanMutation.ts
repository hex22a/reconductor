import { graphql } from 'react-relay';

export const CreateScanMutation = graphql`
    mutation CreateScanMutation($input: CreateScanInput!, $connections: [ID!]!) {
        createScan(input: $input) {
            edge @appendEdge(connections: $connections) {
                cursor
                node {
                    id
                    target
                    schedule
                    created_at
                    status
                }
            }
            errors {
                field
                message
            }
        }
    }
`;
