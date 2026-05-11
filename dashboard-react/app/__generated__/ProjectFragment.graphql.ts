/**
 * @generated SignedSource<<6c92245067e3d0ba7ad9f427a11dc668>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ProjectFragment$data = {
  readonly created_at: string;
  readonly name: string;
  readonly " $fragmentSpreads": FragmentRefs<"ScanListFragment">;
  readonly " $fragmentType": "ProjectFragment";
};
export type ProjectFragment$key = {
  readonly " $data"?: ProjectFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ProjectFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ProjectFragment",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "name",
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
      "args": null,
      "kind": "FragmentSpread",
      "name": "ScanListFragment"
    }
  ],
  "type": "Project",
  "abstractKey": null
};

(node as any).hash = "159d5cef30c4a57444a6de22b7b4ec40";

export default node;
