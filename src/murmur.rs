use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const R1: u32 = 15;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe6546b64;


/// MurmurHash3 (32 bit, seeded)
///
/// # Examples
///
/// ```
/// use murmur3::seeded;
///
/// let murmur32_hash = seeded("abcd".as_bytes(), 1234);
/// assert_eq!(murmur32_hash, 893017187);
/// ```
pub fn seeded(data: &[u8], seed: u32) -> u32 {
    let mut hash = seed;
    let len = data.len();

    for byte_index in (0..len).step_by(4) {
        let remaining_bytes = len - byte_index;
        if remaining_bytes >= 4 {
            let buf = &data[byte_index..byte_index + 4];

            hash = _process_chunk(hash, &buf);

            hash = (hash << R2) | (hash >> (32 - R2));
            hash = (hash.wrapping_mul(M)).wrapping_add(N);

        } else {
            let mut buf = [0u8; 4];
            buf.clone_from_slice(&data[byte_index..len]);

            hash = _process_chunk(hash, &buf);
        }
    }

    hash = hash ^ (len as u32);
    hash = hash ^ (hash >> 16);
    hash = hash.wrapping_mul(0x85ebca6b);
    hash = hash ^ (hash >> 13);
    hash = hash.wrapping_mul(0xC2b2ae35);
    hash = hash ^ (hash >> 16);

    return hash;
}

/// MurmurHash3 (32 bit, unseeded)
///
/// # Examples
///
/// ```
/// use murmur3::unseeded;
///
/// let murmur32_hash = unseeded("abcd".as_bytes());
/// assert_eq!(murmur32_hash, 1139631978);
/// ```
pub fn unseeded(data: &[u8]) -> u32 {
    seeded(data, 0)
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
fn seeded_test() {
    let mut hash = seeded("abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh".as_bytes(),
                          1234);
    assert_eq!(hash, 1618858773);

    hash = seeded("abcdefgh".as_bytes(), 1234);
    assert_eq!(hash, 4037461305);

    hash = seeded("abcdefg".as_bytes(), 1234);
    assert_eq!(hash, 1113025207);

    hash = seeded("abcdefg".as_bytes(), 5678);
    assert_eq!(hash, 1081589395);

    hash = seeded("abcd".as_bytes(), 1234);
    assert_eq!(hash, 893017187);

    hash = seeded("ab1".as_bytes(), 1234);
    assert_eq!(hash, 672282314);

    hash = seeded("a b".as_bytes(), 1234);
    assert_eq!(hash, 629887092);

    hash = seeded("a".as_bytes(), 1234);
    assert_eq!(hash, 1374314456);

    hash = seeded("".as_bytes(), 1234);
    assert_eq!(hash, 254590987);
}

#[test]
fn unseeded_test() {
    let mut hash = unseeded("abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh".as_bytes());
    assert_eq!(hash, 3788670109);

    hash = unseeded("abcdefgh".as_bytes());
    assert_eq!(hash, 1239272644);

    hash = unseeded("abcdefg".as_bytes());
    assert_eq!(hash, 2285673222);

    hash = unseeded("abcd".as_bytes());
    assert_eq!(hash, 1139631978);

    hash = unseeded("ab1".as_bytes());
    assert_eq!(hash, 1110413313);

    hash = unseeded("a b".as_bytes());
    assert_eq!(hash, 1033158525);

    hash = unseeded("a".as_bytes());
    assert_eq!(hash, 1009084850);

    hash = unseeded("".as_bytes());
    assert_eq!(hash, 0);
}
