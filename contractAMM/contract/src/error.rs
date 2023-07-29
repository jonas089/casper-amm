//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
/// Such a user error should be in the range `[0..(u16::MAX - 4)]` (i.e. [0, 65532]) to avoid
/// conflicting with the other `Error` variants.
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    InvalidContext,
    /// Operation would cause an integer overflow.
    Overflow,
    /// Missing key for installer account.
    MissingKey,
    ZeroAmount,
    InvalidToken,
    RatioMismatch,
    /// User error.
    User(u16),
}

const ERROR_INVALID_CONTEXT: u16 = u16::MAX;
const ERROR_OVERFLOW: u16 = u16::MAX - 3;
const ERROR_MISSING_KEY: u16 = u16::MAX - 4;
const ERROR_ZERO_AMOUNT_TRANSFERRED: u16 = u16::MAX - 5;
const ERROR_INVALID_TOKEN_HASH: u16 = u16::MAX - 6;
const ERROR_RATIO_MISMATCH: u16 = u16::MAX - 7;
impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        let user_error = match error {
            Error::InvalidContext => ERROR_INVALID_CONTEXT,
            Error::Overflow => ERROR_OVERFLOW,
            Error::MissingKey => ERROR_MISSING_KEY,
            Error::ZeroAmount => ERROR_ZERO_AMOUNT_TRANSFERRED,
            Error::InvalidToken => ERROR_INVALID_TOKEN_HASH,
            Error::RatioMismatch => ERROR_RATIO_MISMATCH,
            Error::User(user_error) => user_error,
        };
        ApiError::User(user_error)
    }
}
