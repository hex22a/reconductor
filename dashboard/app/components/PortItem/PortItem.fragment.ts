import { graphql } from 'react-relay';

export const fragment = graphql`
    fragment PortItemFragment on Port {
        port
        protocol
        state
        service
        product
        version
    }
`;
