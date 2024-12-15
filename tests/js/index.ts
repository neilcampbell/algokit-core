import init, {
  type PayTransactionFields,
  encodePayment,
} from "./pkg/algo_models_rs";

let { AlgorandTransactionCrafter } = require("@algorandfoundation/algo-models");

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
  const encoded: Uint8Array = tx.encode(); // encoded msg ready - to be signed with EdDSA

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

  const encoded2 = encodePayment(fields);

  console.log("algo-models-ts", encoded);
  console.log("algo-models-rs", encoded2);
  console.log("algo-models-rs", msgpack.decode(encoded2));
  console.log("algo-models-ts", msgpack.decode(encoded.slice(2)));

  console.log("EQUAL?", encoded.slice(2).toString() === encoded2.toString());
}

main();
