/**
 * @generated SignedSource<<bac23be2641d9f0ea18505f6f10079e1>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type HostItemFragment$data = {
  readonly hostname: string | null | undefined;
  readonly ip: string | null | undefined;
  readonly mac: string | null | undefined;
  readonly os_accuracy: number | null | undefined;
  readonly os_match: string | null | undefined;
  readonly vendor: string | null | undefined;
  readonly " $fragmentType": "HostItemFragment";
};
export type HostItemFragment$key = {
  readonly " $data"?: HostItemFragment$data;
  readonly " $fragmentSpreads": FragmentRefs<"HostItemFragment">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "HostItemFragment",
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
      "name": "mac",
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
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "vendor",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "os_match",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "os_accuracy",
      "storageKey": null
    }
  ],
  "type": "Host",
  "abstractKey": null
};

(node as any).hash = "22dfa5c773191d5d1c0ef4279c46d4f2";

export default node;
