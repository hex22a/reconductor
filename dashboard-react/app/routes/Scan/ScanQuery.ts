import { graphql } from 'react-relay';

export const query = graphql`
    query ScanQuery($id: ID!) {
        scan(id: $id) {
            ...ScanFragment
        }
    }
`;
