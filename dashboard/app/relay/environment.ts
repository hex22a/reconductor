import { Environment, Network, RecordSource, Store } from 'relay-runtime';
import { GRAPHQL_URL } from '~/constants';

async function fetchGraphQL(params: any, variables: any) {
    const res = await fetch(GRAPHQL_URL, {
        method: 'POST',
        headers: {
            'content-type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({
            query: params.text,
            variables,
        }),
    });

    return await res.json();
}

const network = Network.create(fetchGraphQL);

export const environment = new Environment({
    network,
    store: new Store(new RecordSource()),
});
