import { graphql } from 'react-relay';

export const query = graphql`
    query RunQuery($id: ID!) {
        run(id: $id) {
            ...ScanRunFragment
        }
    }
`;
