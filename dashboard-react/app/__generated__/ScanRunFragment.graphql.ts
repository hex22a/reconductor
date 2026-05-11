/**
 * @generated SignedSource<<77229bfb56de786e9d2e0a97f12c7bad>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ScanRunFragment$data = {
  readonly created_at: string;
  readonly " $fragmentSpreads": FragmentRefs<"HostListFragment">;
  readonly " $fragmentType": "ScanRunFragment";
};
export type ScanRunFragment$key = {
  readonly " $data"?: ScanRunFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ScanRunFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ScanRunFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "created_at",
      "storageKey": null
    },
    {
      "args": null,
      "kind": "FragmentSpread",
      "name": "HostListFragment"
    }
  ],
  "type": "ScanRun",
  "abstractKey": null
};

(node as any).hash = "b4a40d8fd6685bc08a8fca84ac14594e";

export default node;
