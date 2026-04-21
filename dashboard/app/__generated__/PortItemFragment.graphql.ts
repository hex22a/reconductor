/**
 * @generated SignedSource<<4d2607984f2b71d118575916455963f0>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type PortItemFragment$data = {
  readonly port: number;
  readonly product: string | null | undefined;
  readonly protocol: string | null | undefined;
  readonly service: string | null | undefined;
  readonly state: string | null | undefined;
  readonly version: string | null | undefined;
  readonly " $fragmentType": "PortItemFragment";
};
export type PortItemFragment$key = {
  readonly " $data"?: PortItemFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"PortItemFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "PortItemFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "port",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "protocol",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "state",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "service",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "product",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "version",
      "storageKey": null
    }
  ],
  "type": "Port",
  "abstractKey": null
};

(node as any).hash = "a57eb363de31ab85fa260199b07f89bc";

export default node;
