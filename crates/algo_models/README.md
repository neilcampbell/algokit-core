# algo_models

Rust library for encoding and decoding Algorand transactions. Also includes WASM and Python bindings provided by [wasm-pack](https://github.com/rustwasm/wasm-pack) and [uniffi-rs](https://github.com/mozilla/uniffi-rs).

## Key Files

- [src/lib.rs](src/lib.rs): Core Rust library
- [src/foreign_exports.rs](src/foreign_exports.rs): Wrappers around the Rust library for the WASM and Python bindings
- [tests/js/pkg/algo_models.js](tests/js/pkg/algo_models.js): auto-generated WASM bindings
- [tests/js/index.ts](tests/js/index.ts): example of using the WASM bindings
- [tests/py/algo_models.py](tests/py/algo_models.py): auto-generated Python bindings
- [tests/py/app.py](tests/py/app.py): example of using the Python bindings
- [build.py](build.py): script to build the WASM and Python bindings

## Example

### TypeScript

Example of using the library in TypeScript:

```ts
const fields = {
  header: {
    sender: tx.snd,
    fee: tx.fee,
    transactionType: "Payment",
    firstValid: tx.fv,
    lastValid: tx.lv,
    genesisHash: tx.gh,
    genesisId: tx.gen,
  },
  receiver: tx.rcv,
  amount: tx.amt,
} as PayTransactionFields;

const btyesForSigning = encodePayment(fields);

// Signing with a ed25519 lib that has no idea about Algorand
const sig = await ed.signAsync(btyesForSigning, privKey);
const signedTx = attachSignature(btyesForSigning, sig);
```

See [tests/js/index.ts](tests/js/index.ts) for the full example.
