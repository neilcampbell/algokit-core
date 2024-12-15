/* tslint:disable */
/* eslint-disable */
export function get_encoded_transaction_type(bytes: Uint8Array): TransactionType;
export function encode_payment(tx: PayTransactionFields): Uint8Array;
export function decode_payment(bytes: Uint8Array): PayTransactionFields;
export function encode_asset_transfer(tx: AssetTransferTransactionFields): Uint8Array;
export function decode_asset_transfer(bytes: Uint8Array): AssetTransferTransactionFields;
export type TransactionType = "Payment" | "AssetTransfer" | "AssetFreeze" | "AssetConfig" | "KeyRegistration" | "ApplicationCall";

export interface TransactionHeader {
    transactionType: TransactionType;
    sender: Uint8Array;
    fee: bigint;
    firstValid: bigint;
    lastValid: bigint;
    genesisHash?: Uint8Array;
    genesisId?: string;
    note?: Uint8Array;
    rekeyTo?: Uint8Array;
    lease?: Uint8Array;
    group?: Uint8Array;
}

export interface PayTransactionFields {
    header: TransactionHeader;
    receiver: Uint8Array;
    amount: bigint;
    closeRemainderTo?: Uint8Array;
}

export interface AssetTransferTransactionFields {
    header: TransactionHeader;
    assetId: bigint;
    amount: bigint;
    receiver: Uint8Array;
    assetSender?: Uint8Array;
    closeRemainderTo?: Uint8Array;
}


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_encoded_transaction_type: (a: number, b: number) => [number, number, number];
  readonly encode_payment: (a: any) => [number, number, number, number];
  readonly decode_payment: (a: number, b: number) => [number, number, number];
  readonly encode_asset_transfer: (a: any) => [number, number, number, number];
  readonly decode_asset_transfer: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
