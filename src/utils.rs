use byteorder::{BigEndian, ByteOrder};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn extend_vec(vec: &mut Vec<u8>, len: usize) -> usize {
    let vlen = vec.len();
    vec.resize(vlen + len, 0);
    vec.len()
}

pub trait Encodable {
    fn serialize(&self, vec: &mut Vec<u8>);
}

macro_rules! impl_ints {
    ($ty:ident, $write_fn:ident, $read_fn:ident, $bytes:expr) => {
        impl Encodable for $ty {
            fn serialize(&self, mut vec: &mut Vec<u8>) {
                let vlen = extend_vec(&mut vec, $bytes);
                BigEndian::$write_fn(&mut vec[(vlen - $bytes)..], *self);
            }
        }
    };
}

impl_ints!(u64, write_u64, read_u64, 8);
impl_ints!(u32, write_u32, read_u32, 4);
impl_ints!(u16, write_u16, read_u16, 2);

macro_rules! impl_arrays {
    ($bytes:expr) => {
        impl Encodable for [u8; $bytes] {
            fn serialize(&self, vec: &mut Vec<u8>) {
                vec.extend_from_slice(&self[..]);
            }
        }
    };
}

impl_arrays!(2);
impl_arrays!(4);
impl_arrays!(8);
impl_arrays!(16);
impl_arrays!(32);

/// Calculate sha256 of vector of bytes
pub(crate) fn sha256_bytes(b: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.input(b);
    let mut res = [0; 32];
    hash.result(&mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_ints() {
        let mut v = vec![];
        let number = 0xff0fu16;
        number.serialize(&mut v);
        assert_eq!(v, vec![0xff, 0x0f]);

        v = vec![];
        let number = 0x0f0fff00u32;
        number.serialize(&mut v);
        assert_eq!(v, vec![0x0f, 0x0f, 0xff, 0x00]);

        v = vec![];
        let number = 0xff00ff00ff00ff00u64;
        number.serialize(&mut v);
        assert_eq!(v, vec![0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00]);
    }

    #[test]
    fn test_encode_arrays() {
        let mut v = vec![];
        let numbers = [0xcc, 0xaa];
        numbers.serialize(&mut v);
        assert_eq!(v, vec![0xcc, 0xaa]);
    }

    #[test]
    fn test_encode_mixedup() {
        let mut v = vec![];
        let numbers = [0xcc, 0xaa];
        let number = 0xeeddu16;
        number.serialize(&mut v);
        numbers.serialize(&mut v);
        assert_eq!(v, vec![0xee, 0xdd, 0xcc, 0xaa]);
    }
}
