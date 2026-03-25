/**
 * @generated SignedSource<<c346616a068101ae5a09c7f0151460d7>>
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
    }
  ],
  "type": "Project",
  "abstractKey": null
};

(node as any).hash = "8ab702d1bd0c30166c88fcca8c406509";

export default node;
