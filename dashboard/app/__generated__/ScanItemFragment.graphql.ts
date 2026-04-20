/**
 * @generated SignedSource<<bb8a0954af3a30ae7e71bf101edaa9c5>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ScanItemFragment$data = {
  readonly created_at: string;
  readonly id: string;
  readonly schedule: string | null | undefined;
  readonly status: string;
  readonly target: string;
  readonly " $fragmentType": "ScanItemFragment";
};
export type ScanItemFragment$key = {
  readonly " $data"?: ScanItemFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ScanItemFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ScanItemFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "id",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "created_at",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "target",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "status",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "schedule",
      "storageKey": null
    }
  ],
  "type": "Scan",
  "abstractKey": null
};

(node as any).hash = "b68a00f29c176e674f9b55921a7f1ddf";

export default node;
