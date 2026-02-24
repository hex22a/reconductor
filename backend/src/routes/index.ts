import v1 from './v1';

import { createGraphQlRoutes } from './graphql';

const graphQlRoutes = createGraphQlRoutes();

export default { ...v1, ...graphQlRoutes };
