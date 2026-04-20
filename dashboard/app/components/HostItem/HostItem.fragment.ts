import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment HostItemFragment on Host {
        ip
        mac
        hostname
        vendor
        os_match
        os_accuracy
    }
`;
