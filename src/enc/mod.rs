//! Encoder-based structs and traits.

mod encoder;
mod impl_tuples;
mod impls;

use crate::{config::Config, error::EncodeError};

pub mod write;

pub use self::encoder::EncoderImpl;
use self::write::Writer;

/// Any source that can encode types. This type is most notably implemented for [Encoder].
///
/// [Encoder]: ../struct.Encoder.html
pub trait Encode {
    /// Encode a given type.
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError>;
}

/// Helper trait to encode basic types into.
pub trait Encoder: sealed::Sealed {
    /// The concrete [Writer] type
    type W: Writer;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the writer
    fn writer(&mut self) -> &mut Self::W;

    /// Returns a reference to the config
    fn config(&self) -> &Self::C;
}

impl<'a, T> Encoder for &'a mut T
where
    T: Encoder,
{
    type W = T::W;

    type C = T::C;

    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }
}

pub(crate) mod sealed {
    pub trait Sealed {}

    impl<'a, T> Sealed for &'a mut T where T: Sealed {}
}
