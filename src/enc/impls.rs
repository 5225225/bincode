use super::{Encode, Encodeable};
use crate::error::EncodeError;

impl Encodeable for u8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u8(*self)
    }
}

impl Encodeable for u16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u16(*self)
    }
}

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u32(*self)
    }
}

impl Encodeable for u64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u64(*self)
    }
}

impl Encodeable for u128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u128(*self)
    }
}

impl Encodeable for usize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_usize(*self)
    }
}

impl Encodeable for i8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i8(*self)
    }
}

impl Encodeable for i16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i16(*self)
    }
}

impl Encodeable for i32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i32(*self)
    }
}

impl Encodeable for i64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i64(*self)
    }
}

impl Encodeable for i128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i128(*self)
    }
}

impl Encodeable for isize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_isize(*self)
    }
}

impl Encodeable for f32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_f32(*self)
    }
}

impl Encodeable for f64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_f64(*self)
    }
}

impl Encodeable for char {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_char(*self)
    }
}

impl Encodeable for &'_ [u8] {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_slice(*self)
    }
}

impl Encodeable for &'_ str {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_slice(self.as_bytes())
    }
}

impl<const N: usize> Encodeable for [u8; N] {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_array(*self)
    }
}

impl<'a, T> Encode for &'a mut T
where
    T: Encode,
{
    fn encode_u8(&mut self, val: u8) -> Result<(), EncodeError> {
        T::encode_u8(self, val)
    }
    fn encode_u16(&mut self, val: u16) -> Result<(), EncodeError> {
        T::encode_u16(self, val)
    }
    fn encode_u32(&mut self, val: u32) -> Result<(), EncodeError> {
        T::encode_u32(self, val)
    }
    fn encode_u64(&mut self, val: u64) -> Result<(), EncodeError> {
        T::encode_u64(self, val)
    }
    fn encode_u128(&mut self, val: u128) -> Result<(), EncodeError> {
        T::encode_u128(self, val)
    }
    fn encode_usize(&mut self, val: usize) -> Result<(), EncodeError> {
        T::encode_usize(self, val)
    }

    fn encode_i8(&mut self, val: i8) -> Result<(), EncodeError> {
        T::encode_i8(self, val)
    }
    fn encode_i16(&mut self, val: i16) -> Result<(), EncodeError> {
        T::encode_i16(self, val)
    }
    fn encode_i32(&mut self, val: i32) -> Result<(), EncodeError> {
        T::encode_i32(self, val)
    }
    fn encode_i64(&mut self, val: i64) -> Result<(), EncodeError> {
        T::encode_i64(self, val)
    }
    fn encode_i128(&mut self, val: i128) -> Result<(), EncodeError> {
        T::encode_i128(self, val)
    }
    fn encode_isize(&mut self, val: isize) -> Result<(), EncodeError> {
        T::encode_isize(self, val)
    }

    fn encode_f32(&mut self, val: f32) -> Result<(), EncodeError> {
        T::encode_f32(self, val)
    }
    fn encode_f64(&mut self, val: f64) -> Result<(), EncodeError> {
        T::encode_f64(self, val)
    }
    fn encode_slice(&mut self, val: &[u8]) -> Result<(), EncodeError> {
        T::encode_slice(self, val)
    }
    fn encode_array<const N: usize>(&mut self, val: [u8; N]) -> Result<(), EncodeError> {
        T::encode_array(self, val)
    }

    fn encode_char(&mut self, val: char) -> Result<(), EncodeError> {
        T::encode_char(self, val)
    }
}
