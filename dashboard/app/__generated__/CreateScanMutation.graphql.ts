/**
 * @generated SignedSource<<f46c5a522e0449040619b58e2b9c5c17>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type CreateScanInput = {
  projectId: string;
  schedule?: string | null | undefined;
  target: string;
};
export type CreateScanMutation$variables = {
  connections: ReadonlyArray<string>;
  input: CreateScanInput;
};
export type CreateScanMutation$data = {
  readonly createScan: {
    readonly cursor: string;
    readonly node: {
      readonly created_at: string;
      readonly id: string;
      readonly schedule: string | null | undefined;
      readonly status: string;
      readonly target: string;
    };
  };
};
export type CreateScanMutation = {
  response: CreateScanMutation$data;
  variables: CreateScanMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = {
  "defaultValue": null,
  "kind": "LocalArgument",
  "name": "connections"
},
v1 = {
  "defaultValue": null,
  "kind": "LocalArgument",
  "name": "input"
},
v2 = [
  {
    "kind": "Variable",
    "name": "input",
    "variableName": "input"
  }
],
v3 = {
  "alias": null,
  "args": (v2/*: any*/),
  "concreteType": "ScanEdge",
  "kind": "LinkedField",
  "name": "createScan",
  "plural": false,
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "cursor",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "Scan",
      "kind": "LinkedField",
      "name": "node",
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
          "name": "target",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "schedule",
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
          "name": "status",
          "storageKey": null
        }
      ],
      "storageKey": null
    }
  ],
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": [
      (v0/*: any*/),
      (v1/*: any*/)
    ],
    "kind": "Fragment",
    "metadata": null,
    "name": "CreateScanMutation",
    "selections": [
      (v3/*: any*/)
    ],
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [
      (v1/*: any*/),
      (v0/*: any*/)
    ],
    "kind": "Operation",
    "name": "CreateScanMutation",
    "selections": [
      (v3/*: any*/),
      {
        "alias": null,
        "args": (v2/*: any*/),
        "filters": null,
        "handle": "appendEdge",
        "key": "",
        "kind": "LinkedHandle",
        "name": "createScan",
        "handleArgs": [
          {
            "kind": "Variable",
            "name": "connections",
            "variableName": "connections"
          }
        ]
      }
    ]
  },
  "params": {
    "cacheID": "0217b18a59d6ba4017eb50b99fa78dee",
    "id": null,
    "metadata": {},
    "name": "CreateScanMutation",
    "operationKind": "mutation",
    "text": "mutation CreateScanMutation(\n  $input: CreateScanInput!\n) {\n  createScan(input: $input) {\n    cursor\n    node {\n      id\n      target\n      schedule\n      created_at\n      status\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "3545a33b77de5b2ba05bc7179c01d54f";

export default node;
