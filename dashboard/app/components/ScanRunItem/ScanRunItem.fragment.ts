import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment ScanRunItemFragment on ScanRun {
        id
        created_at
    }
`;
