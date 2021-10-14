use super::{BorrowDecodable, BorrowDecode, Decodable, Decode};
use crate::error::DecodeError;

impl<'de> Decodable for u8 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u8()
    }
}

impl<'de> Decodable for u16 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u16()
    }
}

impl<'de> Decodable for u32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u32()
    }
}

impl<'de> Decodable for u64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u64()
    }
}

impl<'de> Decodable for u128 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u128()
    }
}

impl<'de> Decodable for usize {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_usize()
    }
}

impl<'de> Decodable for i8 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i8()
    }
}

impl<'de> Decodable for i16 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i16()
    }
}

impl<'de> Decodable for i32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i32()
    }
}

impl<'de> Decodable for i64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i64()
    }
}

impl<'de> Decodable for i128 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i128()
    }
}

impl<'de> Decodable for isize {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_isize()
    }
}

impl<'de> Decodable for f32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_f32()
    }
}

impl<'de> Decodable for f64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_f64()
    }
}

impl<'de> Decodable for char {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_char()
    }
}

impl<'a, 'de: 'a> BorrowDecodable<'de> for &'a [u8] {
    fn borrow_decode<D: BorrowDecode<'de>>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        decoder.decode_slice(len)
    }
}

impl<'a, 'de: 'a> BorrowDecodable<'de> for &'a str {
    fn borrow_decode<D: BorrowDecode<'de>>(decoder: D) -> Result<Self, DecodeError> {
        let slice: &[u8] = BorrowDecodable::borrow_decode(decoder)?;
        core::str::from_utf8(slice).map_err(DecodeError::Utf8)
    }
}

impl<'de, const N: usize> Decodable for [u8; N] {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_array()
    }
}

impl<'de, T> Decodable for core::marker::PhantomData<T> {
    fn decode<D: Decode>(_: D) -> Result<Self, DecodeError> {
        Ok(core::marker::PhantomData)
    }
}

impl<'a, 'de, T> Decode for &'a mut T
where
    T: Decode,
{
    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        T::decode_u8(self)
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        T::decode_u16(self)
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        T::decode_u32(self)
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        T::decode_u64(self)
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        T::decode_u128(self)
    }

    fn decode_usize(&mut self) -> Result<usize, DecodeError> {
        T::decode_usize(self)
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        T::decode_i8(self)
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        T::decode_i16(self)
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        T::decode_i32(self)
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        T::decode_i64(self)
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        T::decode_i128(self)
    }

    fn decode_isize(&mut self) -> Result<isize, DecodeError> {
        T::decode_isize(self)
    }

    fn decode_f32(&mut self) -> Result<f32, DecodeError> {
        T::decode_f32(self)
    }

    fn decode_f64(&mut self) -> Result<f64, DecodeError> {
        T::decode_f64(self)
    }

    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        T::decode_array::<N>(self)
    }

    fn decode_char(&mut self) -> Result<char, DecodeError> {
        T::decode_char(self)
    }
}

impl<'a, 'de, T> BorrowDecode<'de> for &'a mut T
where
    T: BorrowDecode<'de>,
{
    fn decode_slice(&mut self, len: usize) -> Result<&'de [u8], DecodeError> {
        T::decode_slice(self, len)
    }
}
