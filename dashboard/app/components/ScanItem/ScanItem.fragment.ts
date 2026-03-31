import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment ScanItemFragment on Scan {
        created_at
        target
        status
        schedule
    }
`;
