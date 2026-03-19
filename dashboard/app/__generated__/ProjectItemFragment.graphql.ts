/**
 * @generated SignedSource<<b9f83bd9a2bcd948aa49a9831c6fe8f8>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ProjectItemFragment$data = {
  readonly created_at: string;
  readonly name: string;
  readonly " $fragmentType": "ProjectItemFragment";
};
export type ProjectItemFragment$key = {
  readonly " $data"?: ProjectItemFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"ProjectItemFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ProjectItemFragment",
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

(node as any).hash = "de9643ca97ff5449d1a7688ca30509cc";

export default node;
