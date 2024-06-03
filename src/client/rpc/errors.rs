use core::fmt;

use miden_objects::{accounts::AccountId, utils::DeserializationError, NoteError};

// RPC ERROR
// ================================================================================================

#[derive(Debug)]
pub enum RpcError {
    ConnectionError(String),
    DeserializationError(String),
    ExpectedFieldMissing(String),
    AccountUpdateForPrivateAccountReceived(AccountId),
    RequestError(String, String),
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcError::ConnectionError(err) => {
                write!(f, "failed to connect to the API server: {err}")
            },
            RpcError::DeserializationError(err) => {
                write!(f, "failed to deserialize RPC data: {err}")
            },
            RpcError::ExpectedFieldMissing(err) => {
                write!(f, "rpc API response missing an expected field: {err}")
            },
            RpcError::AccountUpdateForPrivateAccountReceived(account_id) => {
                write!(
                    f,
                    "rpc API response contained an update for a private account: {}",
                    account_id.to_hex()
                )
            },
            RpcError::RequestError(endpoint, err) => {
                write!(f, "rpc request failed for {endpoint}: {err}")
            },
        }
    }
}

impl From<DeserializationError> for RpcError {
    fn from(err: DeserializationError) -> Self {
        Self::DeserializationError(err.to_string())
    }
}

impl From<NoteError> for RpcError {
    fn from(err: NoteError) -> Self {
        Self::DeserializationError(err.to_string())
    }
}
