/**
 * @generated SignedSource<<ec0bd0fe5a2609e7c3c42d3f0072a469>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ProjectQuery$variables = {
  id: string;
};
export type ProjectQuery$data = {
  readonly project: {
    readonly " $fragmentSpreads": FragmentRefs<"ProjectFragment">;
  } | null | undefined;
};
export type ProjectQuery = {
  response: ProjectQuery$data;
  variables: ProjectQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "id"
  }
],
v1 = [
  {
    "kind": "Variable",
    "name": "id",
    "variableName": "id"
  }
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "ProjectQuery",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "Project",
        "kind": "LinkedField",
        "name": "project",
        "plural": false,
        "selections": [
          {
            "args": null,
            "kind": "FragmentSpread",
            "name": "ProjectFragment"
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "ProjectQuery",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "Project",
        "kind": "LinkedField",
        "name": "project",
        "plural": false,
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
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "id",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "31dc4a87691ea60f2ef4aadfadff39bc",
    "id": null,
    "metadata": {},
    "name": "ProjectQuery",
    "operationKind": "query",
    "text": "query ProjectQuery(\n  $id: ID!\n) {\n  project(id: $id) {\n    ...ProjectFragment\n    id\n  }\n}\n\nfragment ProjectFragment on Project {\n  name\n  created_at\n}\n"
  }
};
})();

(node as any).hash = "3d4dccc96c5c8629f85b6352574f5d58";

export default node;
