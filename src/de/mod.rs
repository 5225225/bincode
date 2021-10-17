//! Decoder-based structs and traits.

use crate::{config::Config, error::DecodeError};

mod decoder;
mod impl_tuples;
mod impls;

pub mod read;
pub use self::decoder::DecoderImpl;
use self::read::{BorrowReader, Reader};

/// Trait that makes a type able to be decoded, akin to serde's `DeserializeOwned` trait.
///
/// This trait should be implemented for types which do not have references to data in the reader. For types that contain e.g. `&str` and `&[u8]`, implement [BorrowDecode] instead.
///
/// Whenever you implement `Decode` for your type, the base trait `BorrowDecode` is automatically implemented.
pub trait Decode: for<'de> BorrowDecode<'de> {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError>;
}

/// Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait.
///
/// This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decode] instead.
pub trait BorrowDecode<'de>: Sized {
    /// Attempt to decode this type with the given [BorrowDecode].
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: D) -> Result<Self, DecodeError>;
}

impl<'de, T: Decode> BorrowDecode<'de> for T {
    fn borrow_decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        Decode::decode(decoder)
    }
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
pub trait Decoder: sealed::Sealed {
    /// The concrete [Reader] type
    type R: Reader;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the reader
    fn reader(&mut self) -> &mut Self::R;

    /// Returns a mutable reference to the config
    fn config(&self) -> &Self::C;
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
///
/// This is an extension of [Decode] that can also return borrowed data.
pub trait BorrowDecoder<'de>: Decoder {
    /// The concrete [BorrowReader] type
    type BR: BorrowReader<'de>;

    /// Rerturns a mutable reference to the borrow reader
    fn borrow_reader(&mut self) -> &mut Self::BR;
}

impl<'a, T> Decoder for &'a mut T
where
    T: Decoder,
{
    type R = T::R;

    type C = T::C;

    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }
}

impl<'a, 'de, T> BorrowDecoder<'de> for &'a mut T
where
    T: BorrowDecoder<'de>,
{
    type BR = T::BR;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        T::borrow_reader(self)
    }
}

pub(crate) mod sealed {
    pub trait Sealed {}

    impl<'a, T> Sealed for &'a mut T where T: Sealed {}
}
