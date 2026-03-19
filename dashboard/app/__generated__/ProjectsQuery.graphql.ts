/**
 * @generated SignedSource<<e6bde822b23845507871828a8062b81b>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type ProjectsQuery$variables = Record<PropertyKey, never>;
export type ProjectsQuery$data = {
  readonly projects: ReadonlyArray<{
    readonly id: string;
    readonly " $fragmentSpreads": FragmentRefs<"ProjectItemFragment">;
  }>;
};
export type ProjectsQuery = {
  response: ProjectsQuery$data;
  variables: ProjectsQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "ProjectsQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Project",
        "kind": "LinkedField",
        "name": "projects",
        "plural": true,
        "selections": [
          (v0/*: any*/),
          {
            "args": null,
            "kind": "FragmentSpread",
            "name": "ProjectItemFragment"
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
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "ProjectsQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Project",
        "kind": "LinkedField",
        "name": "projects",
        "plural": true,
        "selections": [
          (v0/*: any*/),
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
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "72e87bde45fdbc310f59f5b8b5f3e37c",
    "id": null,
    "metadata": {},
    "name": "ProjectsQuery",
    "operationKind": "query",
    "text": "query ProjectsQuery {\n  projects {\n    id\n    ...ProjectItemFragment\n  }\n}\n\nfragment ProjectItemFragment on Project {\n  name\n  created_at\n}\n"
  }
};
})();

(node as any).hash = "d77295b315e236b3f576b0de13ca7809";

export default node;
