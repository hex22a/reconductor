/**
 * @generated SignedSource<<ea8d182a7159ca31351f7ed54b5e61a0>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type HostFragment$data = {
  readonly hostname: string | null | undefined;
  readonly ip: string | null | undefined;
  readonly " $fragmentSpreads": FragmentRefs<"PortListFragment">;
  readonly " $fragmentType": "HostFragment";
};
export type HostFragment$key = {
  readonly " $data"?: HostFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"HostFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "HostFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "ip",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "hostname",
      "storageKey": null
    },
    {
      "args": null,
      "kind": "FragmentSpread",
      "name": "PortListFragment"
    }
  ],
  "type": "Host",
  "abstractKey": null
};

(node as any).hash = "8a0e373a9bebee2b49db43a5bd8dbe3e";

export default node;
