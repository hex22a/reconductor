/**
 * @generated SignedSource<<e1a68d09cd54034fa7ea27113c8eeea3>>
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

(node as any).hash = "575aad60f7766d1831c2321fcf93ea93";

export default node;
