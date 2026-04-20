import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment ScanItemFragment on Scan {
        id
        created_at
        target
        status
        schedule
    }
`;
