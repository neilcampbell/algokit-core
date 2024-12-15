import init, {
  type PayTransactionFields,
  encodePayment,
  attachSignature,
} from "./pkg/algo_models_rs";

let { AlgorandTransactionCrafter } = require("@algorandfoundation/algo-models");
import * as ed from "@noble/ed25519";

import * as msgpack from "algo-msgpack-with-bigint";

async function main() {
  await init();

  // sample vars
  const genId = "testnet-v1.0";
  const genesisHash = "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=";
  const amount = 1000000;
  const from = "TIQ4WPFJQYSP2SKLSCDWTK2IIQQ6FOS6BHYIYDGRUZSSROJC5P3HBCZ67Y";
  const to = "66LKPOMVQJL2YVMTAVULQVZMZZCD5M2YVWA7KRHEOHYOJU5KLH2PB7HRRY";

  const algoCrafter = new AlgorandTransactionCrafter(genId, genesisHash);

  const tx = algoCrafter
    .pay(amount, from, to)
    .addFirstValidRound(1000)
    .addLastValidRound(1500)
    .get();

  // The encoding algorithm is a fork of the actual msgpack (https://github.com/EvanJRichard/msgpack-javascript)
  // After msgpack encoding a TX TAG is added as a prefix to the result.
  const encodedTs: Uint8Array = tx.encode(); // encoded msg ready - to be signed with EdDSA

  // Now for the Rust version...

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
  const privKey = ed.utils.randomPrivateKey();
  const sig = await ed.signAsync(btyesForSigning, privKey);
  const signedTx = attachSignature(btyesForSigning, sig);

  console.log("signedTx", msgpack.decode(signedTx));

  if (encodedTs.toString() !== btyesForSigning.toString()) {
    throw new Error("Encoded transactions are not equal");
  } else {
    console.log("Encoded transactions are equal");
  }
}

main();
