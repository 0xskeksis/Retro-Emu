use thiserror::Error;

#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressError
{
    #[error("Address {0:#X} is out of bounds")]
    OutOfBounds(u16),
    #[error("An unknown error occurred at address {0:#X}")]
    Other(u16),
}
pub type Result<T> = std::result::Result<T, AddressError>;


