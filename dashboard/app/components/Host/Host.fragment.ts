import { graphql } from 'relay-runtime';

export const fragment = graphql`
    fragment HostFragment on Host {
        ip
        hostname
        ...PortListFragment
    }
`;
