/**
 * @generated SignedSource<<8d7ebc197b81c6ce8ea8843bff8ee386>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ScanFragment$data = {
  readonly created_at: string;
  readonly schedule: string | null | undefined;
  readonly status: string;
  readonly target: string;
  readonly " $fragmentSpreads": FragmentRefs<"ScanRunListFragment">;
  readonly " $fragmentType": "ScanFragment";
};
export type ScanFragment$key = {
  readonly " $data"?: ScanFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ScanFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ScanFragment",
  "selections": [
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
      "name": "created_at",
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
    },
    {
      "args": null,
      "kind": "FragmentSpread",
      "name": "ScanRunListFragment"
    }
  ],
  "type": "Scan",
  "abstractKey": null
};

(node as any).hash = "6b81f16ab1e76fddaed780a3f85ed2ef";

export default node;
