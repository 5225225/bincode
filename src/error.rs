//! Errors that can be encounting by Encoding and Decoding.

/// Errors that can be encountered by encoding a type
#[non_exhaustive]
#[derive(Debug)]
pub enum EncodeError {
    /// The writer ran out of storage.
    UnexpectedEnd,

    /// The targetted writer encountered an `std::io::Error`
    #[cfg(feature = "std")]
    Io {
        /// The encountered error
        error: std::io::Error,
        /// The amount of bytes that were written before the error occured
        index: usize,
    },
}

/// Errors that can be encounted by decoding a type
#[non_exhaustive]
#[derive(Debug)]
pub enum DecodeError {
    /// The reader reached its end but more bytes were expected.
    UnexpectedEnd,

    /// Invalid type was found. The decoder tried to read type `expected`, but found type `found` instead.
    InvalidIntegerType {
        /// The type that was being read from the reader
        expected: IntegerType,
        /// The type that was encoded in the data
        found: IntegerType,
    },

    /// Invalid enum variant was found. The decoder tried to decode variant index `found`, but the variant index should be between `min` and `max`.
    UnexpectedVariant {
        /// The type name that was being decoded.
        type_name: &'static str,

        /// The min index of the enum. Usually this is `0`.
        min: u32,

        /// the max index of the enum.
        max: u32,

        /// The index of the enum that the decoder encountered
        found: u32,
    },

    /// The decoder tried to decode a `str`, but an utf8 error was encountered.
    Utf8(core::str::Utf8Error),

    /// The decoder tried to decode a `char` and failed. The given buffer contains the bytes that are read at the moment of failure.
    InvalidCharEncoding([u8; 4]),

    /// The decoder tried to decode a `bool` and failed. The given value is what is actually read.
    InvalidBooleanValue(u8),

    /// The decoder tried to decode an array of length `required`, but the binary data contained an array of length `found`.
    ArrayLengthMismatch {
        /// The length of the array required by the rust type.
        required: usize,
        /// The length of the array found in the binary format.
        found: usize,
    },
}

/// Integer types. Used by [DecodeError]. These types have no purpose other than being shown in errors.
#[non_exhaustive]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum IntegerType {
    U16,
    U32,
    U64,
    U128,
    USize,
}
