/**
 * @generated SignedSource<<b460dbce71b8300b57e6a374f82db422>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type CreateProjectInput = {
  name: string;
};
export type CreateProjectMutation$variables = {
  input: CreateProjectInput;
};
export type CreateProjectMutation$data = {
  readonly createProject: {
    readonly created_at: string;
    readonly id: string;
    readonly name: string;
  };
};
export type CreateProjectMutation = {
  response: CreateProjectMutation$data;
  variables: CreateProjectMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "input"
  }
],
v1 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Variable",
        "name": "input",
        "variableName": "input"
      }
    ],
    "concreteType": "Project",
    "kind": "LinkedField",
    "name": "createProject",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "id",
        "storageKey": null
      },
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
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "CreateProjectMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "CreateProjectMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "53b373864643232d466b4c278a93b574",
    "id": null,
    "metadata": {},
    "name": "CreateProjectMutation",
    "operationKind": "mutation",
    "text": "mutation CreateProjectMutation(\n  $input: CreateProjectInput!\n) {\n  createProject(input: $input) {\n    id\n    name\n    created_at\n  }\n}\n"
  }
};
})();

(node as any).hash = "19fb64614138679a1519b403f0b6b613";

export default node;
