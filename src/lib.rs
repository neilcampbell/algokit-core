use rmp_serde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

mod wasm_exports;

#[cfg(not(target_arch = "wasm32"))]
uniffi::setup_scaffolding!();

#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Error))]
#[derive(Debug, Error)]
pub enum MsgPackError {
    #[error("Failed to serialize transaction")]
    SerializeError,

    #[error("Failed to deserialize transaction")]
    DeserializeError,

    #[error("Failed to encode rmpv value")]
    RmpvEncodeError,

    #[error("Failed to decode rmpv value")]
    RmpvDecodeError,

    #[error("Failed to convert rmpv value")]
    RmpvConvertError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionType {
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

type Byte32 = serde_bytes::ByteBuf;
type Pubkey = Byte32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionHeader {
    #[serde(rename = "type")]
    transaction_type: TransactionType,

    #[serde(rename = "snd")]
    sender: Pubkey,

    fee: u64,

    #[serde(rename = "fv")]
    first_valid: u64,

    #[serde(rename = "lv")]
    last_valid: u64,

    #[serde(rename = "gh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    genesis_hash: Option<Byte32>,

    #[serde(rename = "gen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    genesis_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<serde_bytes::ByteBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rekey")]
    rekey_to: Option<Pubkey>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lx")]
    lease: Option<Byte32>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    close_remainder_to: Option<Pubkey>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AssetTransferTransactionFields {
    #[serde(flatten)]
    header: TransactionHeader,

    #[serde(rename = "xaid")]
    asset_id: u64,

    #[serde(rename = "aamt")]
    amount: u64,

    #[serde(rename = "arcv")]
    receiver: Pubkey,

    #[serde(rename = "asnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_sender: Option<Pubkey>,

    #[serde(rename = "aclose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    close_remainder_to: Option<Pubkey>,
}

#[derive(Debug)]
pub enum Transaction {
    Payment(PayTransactionFields),
    AssetTransfer(AssetTransferTransactionFields),
}

impl Transaction {
    /// msgpack encoding of the transaction with keys sorted and empty fields omitted
    pub fn encode(&self) -> Result<Vec<u8>, MsgPackError> {
        // First serialize to a temporary buffer to get the map entries
        let mut temp_buf = Vec::new();
        let mut temp_serializer = rmp_serde::Serializer::new(&mut temp_buf)
            .with_struct_map()
            .with_bytes(rmp_serde::config::BytesMode::ForceAll);

        match self {
            Transaction::Payment(tx) => tx.serialize(&mut temp_serializer),
            Transaction::AssetTransfer(tx) => tx.serialize(&mut temp_serializer),
        }
        .map_err(|_| MsgPackError::SerializeError)?;

        // Deserialize into a BTreeMap to sort
        let sorted_map: BTreeMap<String, rmpv::Value> = rmpv::ext::from_value(
            rmpv::decode::read_value(&mut temp_buf.as_slice())
                .map_err(|_| MsgPackError::RmpvDecodeError)?,
        )
        .map_err(|_| MsgPackError::RmpvConvertError)?;

        // Serialize the sorted map
        let mut final_buf = Vec::new();
        rmpv::encode::write_value(
            &mut final_buf,
            &rmpv::Value::Map(
                sorted_map
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::String(k.into()), v))
                    .collect(),
            ),
        )
        .map_err(|_| MsgPackError::RmpvEncodeError)?;

        Ok(final_buf)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, MsgPackError> {
        // First decode just the header to check the type
        let header: TransactionHeader =
            rmp_serde::from_slice(bytes).map_err(|_| MsgPackError::DeserializeError)?;

        // Then decode the full transaction based on the type
        match header.transaction_type {
            TransactionType::Payment => Ok(Transaction::Payment(
                rmp_serde::from_slice(bytes).map_err(|_| MsgPackError::DeserializeError)?,
            )),
            TransactionType::AssetTransfer => Ok(Transaction::AssetTransfer(
                rmp_serde::from_slice(bytes).map_err(|_| MsgPackError::DeserializeError)?,
            )),
            _ => unimplemented!("Encoding not implemented for this transaction type"),
        }
    }
}

#[test]
fn test_pay_transaction() {
    let transaction = PayTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::Payment,
            sender: serde_bytes::ByteBuf::from([0; 32]),
            fee: 1000,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        receiver: serde_bytes::ByteBuf::from([0; 32]),
        amount: 1000,
        close_remainder_to: None,
    };

    let tx = Transaction::Payment(transaction.clone());
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();

    match decoded {
        Transaction::Payment(tx) => assert_eq!(tx, transaction),
        _ => panic!("Decoded transaction is not a payment"),
    }
}

#[test]
fn test_asset_transfer_transaction() {
    let transaction = AssetTransferTransactionFields {
        header: TransactionHeader {
            genesis_id: None,
            transaction_type: TransactionType::AssetTransfer,
            sender: serde_bytes::ByteBuf::from([0; 32]),
            fee: 1000,
            first_valid: 1000,
            last_valid: 1000,
            genesis_hash: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        },
        asset_id: 1,
        amount: 1000,
        receiver: serde_bytes::ByteBuf::from([0; 32]),
        asset_sender: None,
        close_remainder_to: None,
    };

    let tx = Transaction::AssetTransfer(transaction.clone());
    let encoded = tx.encode().unwrap();
    let decoded = Transaction::decode(&encoded).unwrap();

    match decoded {
        Transaction::AssetTransfer(tx) => assert_eq!(tx, transaction),
        _ => panic!("Decoded transaction is not an asset transfer"),
    }
}
