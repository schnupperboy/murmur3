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
/// let murmur32_hash = seeded("abcd", 1234);
/// assert_eq!(murmur32_hash, 893017187);
/// ```
pub fn seeded(key: &str, seed: u32) -> u32 {
    let mut hash = seed;
    let key_bytes: &[u8] = key.as_bytes();
    let len = key_bytes.len();

    for byte_index in (0..len).step_by(4) {
        let remaining_bytes = len - byte_index;
        if remaining_bytes >= 4 {
            let buf = &key_bytes[byte_index..byte_index + 4];

            hash = _process_chunk(hash, &buf);

            hash = (hash << R2) | (hash >> (32 - R2));
            hash = (hash.wrapping_mul(M)).wrapping_add(N);

        } else {
            let mut buf = [0u8; 4];
            buf.clone_from_slice(&key_bytes[byte_index..len]);

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
/// let murmur32_hash = unseeded("abcd");
/// assert_eq!(murmur32_hash, 1139631978);
/// ```
pub fn unseeded(key: &str) -> u32 {
    seeded(key, 0)
}

fn _process_chunk(hash: u32, buf: &[u8]) -> u32 {
    let mut buf_reader = Cursor::new(buf);
    let mut chunk = buf_reader.read_u32::<LittleEndian>().unwrap();

    chunk = chunk.wrapping_mul(C1);
    chunk = (chunk << R1) | (chunk >> (32 - R1));
    chunk = chunk.wrapping_mul(C2);

    hash ^ chunk
}

#[test]
fn seeded_test() {
    let mut hash = seeded("abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh",
                          1234);
    assert_eq!(hash, 1618858773);

    hash = seeded("abcdefgh", 1234);
    assert_eq!(hash, 4037461305);

    hash = seeded("abcdefg", 1234);
    assert_eq!(hash, 1113025207);

    hash = seeded("abcdefg", 5678);
    assert_eq!(hash, 1081589395);

    hash = seeded("abcd", 1234);
    assert_eq!(hash, 893017187);

    hash = seeded("ab1", 1234);
    assert_eq!(hash, 672282314);

    hash = seeded("a b", 1234);
    assert_eq!(hash, 629887092);

    hash = seeded("a", 1234);
    assert_eq!(hash, 1374314456);

    hash = seeded("", 1234);
    assert_eq!(hash, 254590987);
}

#[test]
fn unseeded_test() {
    let mut hash = unseeded("abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh");
    assert_eq!(hash, 3788670109);

    hash = unseeded("abcdefgh");
    assert_eq!(hash, 1239272644);

    hash = unseeded("abcdefg");
    assert_eq!(hash, 2285673222);

    hash = unseeded("abcd");
    assert_eq!(hash, 1139631978);

    hash = unseeded("ab1");
    assert_eq!(hash, 1110413313);

    hash = unseeded("a b");
    assert_eq!(hash, 1033158525);

    hash = unseeded("a");
    assert_eq!(hash, 1009084850);

    hash = unseeded("");
    assert_eq!(hash, 0);
}
