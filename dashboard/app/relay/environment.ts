import { Environment, Network, RecordSource, Store } from 'relay-runtime';
import type { RequestParameters, Variables } from 'relay-runtime';
import { GRAPHQL_URL } from '~/constants';

export function createEnvironment(csrfToken: string | null): Environment {
    async function fetchGraphQL(params: RequestParameters, variables: Variables) {
        const res = await fetch(GRAPHQL_URL, {
            method: 'POST',
            headers: {
                'content-type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
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

    return new Environment({
        network,
        store: new Store(new RecordSource()),
    });
}
