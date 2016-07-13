use std::hash::{Hasher, Hash};
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const R1: u32 = 15;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe6546b64;


pub struct Murmur3Hasher {
    hash: u32,
    len: usize,
    carry: Vec<u8>,
    seed: u32
}

impl Murmur3Hasher {
    /// Creates a new `Murmur3Hasher`.
    pub fn new(seed: u32) -> Murmur3Hasher {
        let mut state = Murmur3Hasher {
            hash: seed,
            len: 0,
            carry: Vec::with_capacity(4),
            seed: seed
        };

        state.reset();
        state
    }

    fn reset(&mut self) {
        self.hash = self.seed;
        self.len = 0;
        self.carry = Vec::with_capacity(4);
    }
}

impl Hasher for Murmur3Hasher {

    fn write(&mut self, new_data: &[u8]) {
        println!("new data: {:?}", new_data);

        self.len = self.len + new_data.len();

        let mut data = self.carry.clone();
        data.append(&mut new_data.to_vec());

        for chunk in data.chunks(4) {
            if chunk.len() == 4 {
                self.hash = _process_chunk(self.hash, chunk);

                self.hash = (self.hash << R2) | (self.hash >> (32 - R2));
                self.hash = (self.hash.wrapping_mul(M)).wrapping_add(N);
            } else {
                self.carry = chunk.to_vec();
            }
        }
    }

    fn finish(&self) -> u64 {
        let mut hash = self.hash;

        let mut buf = [0u8; 4];
        buf.clone_from_slice(&self.carry);
        hash = _process_chunk(hash, &buf);

        hash = hash ^ (self.len as u32);
        hash = hash ^ (hash >> 16);
        hash = hash.wrapping_mul(0x85ebca6b);
        hash = hash ^ (hash >> 13);
        hash = hash.wrapping_mul(0xC2b2ae35);
        hash = hash ^ (hash >> 16);

        hash as u64
    }
}

#[cfg(target_endian="big")]
macro_rules! read_u32 {
    ($buf_reader:expr) => ($buf_reader.read_u32::<BigEndian>().unwrap());
}

#[cfg(target_endian="little")]
macro_rules! read_u32 {
    ($buf_reader:expr) => ($buf_reader.read_u32::<LittleEndian>().unwrap());
}

fn _process_chunk(hash: u32, buf: &[u8]) -> u32 {
    let mut buf_reader = Cursor::new(buf);
    let mut chunk = read_u32!(buf_reader);

    chunk = chunk.wrapping_mul(C1);
    chunk = (chunk << R1) | (chunk >> (32 - R1));
    chunk = chunk.wrapping_mul(C2);

    hash ^ chunk
}

#[test]
fn test_progressive_equals_one_shot() {
    let data1 = "abcdefghabcdefghabcdefghabcdefgh".as_bytes();
    let data2 = "abcdefghabcdefghabcdefghabcdefgh".as_bytes();
    let data12 = "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh".as_bytes();
    let expected_hash = 1618858773;

    let mut progressive_hasher = Murmur3Hasher::new(1234);
    Hash::hash_slice(data1, &mut progressive_hasher);
    Hash::hash_slice(data2, &mut progressive_hasher);
    let progressive_hash = progressive_hasher.finish();

    let mut one_shot_hasher = Murmur3Hasher::new(1234);
    Hash::hash_slice(data12, &mut one_shot_hasher);
    let one_shot_hash = one_shot_hasher.finish();

    assert_eq!(progressive_hash, expected_hash);
    assert_eq!(one_shot_hash, expected_hash);
}

#[test]
fn test_unaligned() {
    let data = "abcde".as_bytes();
    let expected_hash = 3902511862;

    let mut hasher = Murmur3Hasher::new(0);
    Hash::hash_slice(data, &mut hasher);
    let hash = hasher.finish();

    assert_eq!(hash, expected_hash);
}

#[test]
fn test_empty_seeded() {
    let data = "".as_bytes();
    let expected_hash = 254590987;

    let mut hasher = Murmur3Hasher::new(1234);
    Hash::hash_slice(data, &mut hasher);
    let hash = hasher.finish();

    assert_eq!(hash, expected_hash);
}

#[test]
fn test_empty_unseeded() {
    let data = "".as_bytes();
    let expected_hash = 0;

    let mut hasher = Murmur3Hasher::new(0);
    Hash::hash_slice(data, &mut hasher);
    let hash = hasher.finish();

    assert_eq!(hash, expected_hash);
}

#[test]
fn test_reset() {
    let data1 = "abcd".as_bytes();
    let data2 = "efgh".as_bytes();
    let expected_hash1 = 1139631978;
    let expected_hash2 = 635154487;

    let mut hasher = Murmur3Hasher::new(0);
    Hash::hash_slice(data1, &mut hasher);
    let hash1 = hasher.finish();

    hasher.reset();
    Hash::hash_slice(data2, &mut hasher);
    let hash2 = hasher.finish();

    assert_eq!(hash1, expected_hash1);
    assert_eq!(hash2, expected_hash2);
}
