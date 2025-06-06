/**
 * Algod REST API.
 * API endpoint for algod operations.
 *
 * OpenAPI spec version: 0.0.1
 * Contact: contact@algorand.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { SimulateInitialStates } from '../models/SimulateInitialStates';
import { SimulateTraceConfig } from '../models/SimulateTraceConfig';
import { SimulateTransactionGroupResult } from '../models/SimulateTransactionGroupResult';
import { SimulationEvalOverrides } from '../models/SimulationEvalOverrides';
import { HttpFile } from '../http/http';

export class SimulateTransaction200Response {
    /**
    * The version of this response object.
    */
    'version': number;
    /**
    * The round immediately preceding this simulation. State changes through this round were used to run this simulation.
    */
    'lastRound': number;
    /**
    * A result object for each transaction group that was simulated.
    */
    'txnGroups': Array<SimulateTransactionGroupResult>;
    'evalOverrides'?: SimulationEvalOverrides;
    'execTraceConfig'?: SimulateTraceConfig;
    'initialStates'?: SimulateInitialStates;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "version",
            "baseName": "version",
            "type": "number",
            "format": ""
        },
        {
            "name": "lastRound",
            "baseName": "last-round",
            "type": "number",
            "format": ""
        },
        {
            "name": "txnGroups",
            "baseName": "txn-groups",
            "type": "Array<SimulateTransactionGroupResult>",
            "format": ""
        },
        {
            "name": "evalOverrides",
            "baseName": "eval-overrides",
            "type": "SimulationEvalOverrides",
            "format": ""
        },
        {
            "name": "execTraceConfig",
            "baseName": "exec-trace-config",
            "type": "SimulateTraceConfig",
            "format": ""
        },
        {
            "name": "initialStates",
            "baseName": "initial-states",
            "type": "SimulateInitialStates",
            "format": ""
        }    ];

    static getAttributeTypeMap() {
        return SimulateTransaction200Response.attributeTypeMap;
    }

    public constructor() {
    }
}
