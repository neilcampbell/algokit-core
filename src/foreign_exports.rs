#[cfg(not(target_arch = "wasm32"))]
use crate::UniffiCustomTypeConverter;
#[cfg(not(target_arch = "wasm32"))]
use uniffi::{self};

use crate::{MsgPackError, Transaction};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl UniffiCustomTypeConverter for ByteBuf {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(ByteBuf::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec()
    }
}

#[cfg(not(target_arch = "wasm32"))]
uniffi::custom_type!(ByteBuf, Vec<u8>);

#[cfg(target_arch = "wasm32")]
impl From<MsgPackError> for JsValue {
    fn from(e: MsgPackError) -> Self {
        JsValue::from(e.to_string())
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Enum))]
pub enum TransactionType {
    Payment,
    AssetTransfer,
    AssetFreeze,
    AssetConfig,
    KeyRegistration,
    ApplicationCall,
}

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Record))]
/// The transaction header contains the fields that can be present in any transaction.
/// "Header" only indicates that these are common fields, NOT that they are the first fields in the transaction.
pub struct TransactionHeader {
    /// The type of transaction
    transaction_type: TransactionType,

    /// The sender of the transaction
    sender: ByteBuf,

    fee: u64,

    first_valid: u64,

    last_valid: u64,

    #[tsify(optional)]
    genesis_hash: Option<ByteBuf>,

    #[tsify(optional)]
    genesis_id: Option<String>,

    #[tsify(optional)]
    note: Option<ByteBuf>,

    #[tsify(optional)]
    rekey_to: Option<ByteBuf>,

    #[tsify(optional)]
    lease: Option<ByteBuf>,

    #[tsify(optional)]
    group: Option<ByteBuf>,
}

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Record))]
pub struct PayTransactionFields {
    header: TransactionHeader,

    receiver: ByteBuf,

    amount: u64,

    #[tsify(optional)]
    close_remainder_to: Option<ByteBuf>,
}

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(target_arch = "wasm32"), derive(uniffi::Record))]
pub struct AssetTransferTransactionFields {
    header: TransactionHeader,

    asset_id: u64,

    amount: u64,

    receiver: ByteBuf,

    #[tsify(optional)]
    asset_sender: Option<ByteBuf>,

    #[tsify(optional)]
    close_remainder_to: Option<ByteBuf>,
}

// Go from wasm to rust
impl From<TransactionType> for crate::TransactionType {
    fn from(tx: TransactionType) -> Self {
        match tx {
            TransactionType::Payment => crate::TransactionType::Payment,
            TransactionType::AssetTransfer => crate::TransactionType::AssetTransfer,
            TransactionType::AssetFreeze => crate::TransactionType::AssetFreeze,
            TransactionType::AssetConfig => crate::TransactionType::AssetConfig,
            TransactionType::KeyRegistration => crate::TransactionType::KeyRegistration,
            TransactionType::ApplicationCall => crate::TransactionType::ApplicationCall,
        }
    }
}

impl From<TransactionHeader> for crate::TransactionHeader {
    fn from(tx: TransactionHeader) -> Self {
        Self {
            transaction_type: tx.transaction_type.into(),
            sender: tx.sender,
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash,
            note: tx.note,
            rekey_to: tx.rekey_to,
            lease: tx.lease,
            group: tx.group,
        }
    }
}

impl From<PayTransactionFields> for crate::PayTransactionFields {
    fn from(tx: PayTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            receiver: tx.receiver,
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<AssetTransferTransactionFields> for crate::AssetTransferTransactionFields {
    fn from(tx: AssetTransferTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: tx.receiver,
            asset_sender: tx.asset_sender,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<crate::TransactionHeader> for TransactionHeader {
    fn from(tx: crate::TransactionHeader) -> Self {
        Self {
            transaction_type: tx.transaction_type.into(),
            sender: ByteBuf::from(tx.sender.to_vec()),
            fee: tx.fee,
            first_valid: tx.first_valid,
            last_valid: tx.last_valid,
            genesis_id: tx.genesis_id,
            genesis_hash: tx.genesis_hash,
            note: tx.note,
            rekey_to: tx.rekey_to,
            lease: tx.lease,
            group: tx.group,
        }
    }
}

impl From<crate::PayTransactionFields> for PayTransactionFields {
    fn from(tx: crate::PayTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            receiver: ByteBuf::from(tx.receiver.to_vec()),
            amount: tx.amount,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

impl From<crate::AssetTransferTransactionFields> for AssetTransferTransactionFields {
    fn from(tx: crate::AssetTransferTransactionFields) -> Self {
        Self {
            header: tx.header.into(),
            asset_id: tx.asset_id,
            amount: tx.amount,
            receiver: ByteBuf::from(tx.receiver.to_vec()),
            asset_sender: tx.asset_sender,
            close_remainder_to: tx.close_remainder_to,
        }
    }
}

// Go from rust to wasm
impl From<crate::TransactionType> for TransactionType {
    fn from(tx: crate::TransactionType) -> Self {
        match tx {
            crate::TransactionType::Payment => TransactionType::Payment,
            crate::TransactionType::AssetTransfer => TransactionType::AssetTransfer,
            crate::TransactionType::AssetFreeze => TransactionType::AssetFreeze,
            crate::TransactionType::AssetConfig => TransactionType::AssetConfig,
            crate::TransactionType::KeyRegistration => TransactionType::KeyRegistration,
            crate::TransactionType::ApplicationCall => TransactionType::ApplicationCall,
        }
    }
}

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen(js_name = "getEncodedTransactionType")
)]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]

/// Get the transaction type from the encoded transaction.
/// This is particularly useful when decoding a transaction that has a unknow type
pub fn get_encoded_transaction_type(bytes: &[u8]) -> Result<TransactionType, MsgPackError> {
    let header: TransactionHeader =
        rmp_serde::from_slice(bytes).map_err(|_| MsgPackError::DeserializeError)?;
    Ok(header.transaction_type)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "encodePayment"))]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn encode_payment(tx: PayTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    Transaction::Payment(tx.clone().into()).encode()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "decodePayment"))]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn decode_payment(bytes: &[u8]) -> Result<PayTransactionFields, MsgPackError> {
    Transaction::decode(bytes).map(|tx| match tx {
        Transaction::Payment(tx) => tx.into(),
        _ => panic!("Decoded transaction is not a payment"),
    })
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "encodeAssetTransfer"))]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn encode_asset_transfer(tx: AssetTransferTransactionFields) -> Result<Vec<u8>, MsgPackError> {
    Transaction::AssetTransfer(tx.clone().into()).encode()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "decodeAssetTransfer"))]
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn decode_asset_transfer(bytes: &[u8]) -> Result<AssetTransferTransactionFields, MsgPackError> {
    Transaction::decode(bytes).map(|tx| match tx {
        Transaction::AssetTransfer(tx) => tx.into(),
        _ => panic!("Decoded transaction is not an asset transfer"),
    })
}
