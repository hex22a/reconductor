/**
 * @generated SignedSource<<1b0146998b0e6455b1f9c961b65c256c>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ScanRunItemFragment$data = {
  readonly created_at: string;
  readonly " $fragmentType": "ScanRunItemFragment";
};
export type ScanRunItemFragment$key = {
  readonly " $data"?: ScanRunItemFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ScanRunItemFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ScanRunItemFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "created_at",
      "storageKey": null
    }
  ],
  "type": "ScanRun",
  "abstractKey": null
};

(node as any).hash = "f4dd689108f929e4cfcdc9c2338a13e4";

export default node;
