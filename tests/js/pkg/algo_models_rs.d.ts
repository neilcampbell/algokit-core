/* tslint:disable */
/* eslint-disable */
export function get_encoded_transaction_type(bytes: Uint8Array): TransactionType;
export function encode_payment(tx: PayTransactionFields): Uint8Array;
export function decode_payment(bytes: Uint8Array): PayTransactionFields;
export function encode_asset_transfer(tx: AssetTransferTransactionFields): Uint8Array;
export function decode_asset_transfer(bytes: Uint8Array): AssetTransferTransactionFields;
export type TransactionType = "Payment" | "AssetTransfer" | "AssetFreeze" | "AssetConfig" | "KeyRegistration" | "ApplicationCall";

export interface TransactionHeader {
    transaction_type: TransactionType;
    sender: Uint8Array;
    fee: bigint;
    first_valid: bigint;
    last_valid: bigint;
    genesis_hash?: Uint8Array;
    genesis_id?: string;
    note?: Uint8Array;
    rekey_to?: Uint8Array;
    lease?: Uint8Array;
    group?: Uint8Array;
}

export interface PayTransactionFields extends TransactionHeader {
    receiver: Uint8Array;
    amount: bigint;
    close_remainder_to?: Uint8Array;
}

export interface AssetTransferTransactionFields extends TransactionHeader {
    asset_id: bigint;
    amount: bigint;
    receiver: Uint8Array;
    asset_sender?: Uint8Array;
    close_remainder_to?: Uint8Array;
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
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
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
