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

import { Box } from '../models/Box';
import { HttpFile } from '../http/http';

export class GetApplicationBoxes200Response {
    /**
    * The round for which this information is relevant.
    */
    'round': number;
    /**
    * Used for pagination, when making another request provide this token with the next parameter.
    */
    'nextToken'?: string;
    'boxes': Array<Box>;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "round",
            "baseName": "round",
            "type": "number",
            "format": ""
        },
        {
            "name": "nextToken",
            "baseName": "next-token",
            "type": "string",
            "format": ""
        },
        {
            "name": "boxes",
            "baseName": "boxes",
            "type": "Array<Box>",
            "format": ""
        }    ];

    static getAttributeTypeMap() {
        return GetApplicationBoxes200Response.attributeTypeMap;
    }

    public constructor() {
    }
}
