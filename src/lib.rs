use rmp_serde;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
pub enum MsgPackError {
    SerializeError(rmp_serde::encode::Error),
    DeserializeError(rmp_serde::decode::Error),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum TransactionType {
    #[serde(rename = "pay")]
    Payment,

    #[serde(rename = "axfer")]
    AssetTransfer,

    #[serde(rename = "afrz")]
    AssetFreeze,

    #[serde(rename = "acfg")]
    AssetConfig,

    #[serde(rename = "keyreg")]
    KeyRegistration,

    #[serde(rename = "appl")]
    ApplicationCall,
}

type Byte32 = [u8; 32];
type Pubkey = Byte32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TransactionHeader {
    #[serde(rename = "type")]
    transaction_type: TransactionType,

    #[serde(rename = "snd")]
    sender: Pubkey,

    fee: u64,

    #[serde(rename = "fv")]
    first_valid: u64,

    #[serde(rename = "lv")]
    last_valid: u64,

    #[serde(rename = "gen")]
    genesis_hash: Option<String>,

    note: Option<Vec<u8>>,

    #[serde(rename = "rekey")]
    rekey_to: Option<Pubkey>,

    #[serde(rename = "lx")]
    lease: Option<Byte32>,

    #[serde(rename = "grp")]
    group: Option<Byte32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PayTransactionFields {
    #[serde(flatten)]
    header: TransactionHeader,

    #[serde(rename = "rcv")]
    receiver: Pubkey,

    #[serde(rename = "amt")]
    amount: u64,

    #[serde(rename = "close")]
    close_remainder_to: Option<Pubkey>,
}

#[derive(Debug)]
pub enum Transaction {
    Payment(PayTransactionFields),
}

impl Transaction {
    pub fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        // We use serde_json because it's going to be sorted, which is required for TXID/signing
        let value = match self {
            Transaction::Payment(tx) => serde_json::to_value(tx),
        }
        .map_err(|e| {
            MsgPackError::SerializeError(rmp_serde::encode::Error::Syntax(e.to_string()))
        })?;

        let mut buf = Vec::new();
        let mut serializer = rmp_serde::Serializer::new(&mut buf).with_struct_map();

        value
            .serialize(&mut serializer)
            .map_err(MsgPackError::SerializeError)?;

        Ok(buf)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        // First decode just the header to check the type
        let header: TransactionHeader =
            rmp_serde::from_slice(bytes).map_err(MsgPackError::DeserializeError)?;

        // Then decode the full transaction based on the type
        match header.transaction_type {
            TransactionType::Payment => Ok(Transaction::Payment(
                rmp_serde::from_slice(bytes).map_err(MsgPackError::DeserializeError)?,
            )),
            _ => unimplemented!("Encoding not implemented for this transaction type"),
        }
    }
}

#[test]
fn test_pay_transaction() {
    let transaction = PayTransactionFields {
        header: TransactionHeader {
            transaction_type: TransactionType::Payment,
            sender: [0; 32],
            fee: 1000,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        receiver: [0; 32],
        amount: 1000,
        close_remainder_to: None,
    };

    let tx = Transaction::Payment(transaction.clone());
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();

    match decoded {
        Transaction::Payment(tx) => assert_eq!(tx, transaction),
    }
}
