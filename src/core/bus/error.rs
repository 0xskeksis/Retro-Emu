use thiserror::Error;

#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressError
{
    #[error("Address {0:#X} is out of bounds")]
    OutOfBounds(u16),
    #[error("Address {0:#X} is invalid")]
    Invalid(u16),
    #[error("Address {0:#X} is read-only")]
    ReadOnly(u16),
    #[error("An unknown error occurred at address {0:#X}")]
    Other(u16),
}
pub type BusResult<T> = std::result::Result<T, AddressError>;


