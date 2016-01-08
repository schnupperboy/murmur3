#![feature(clone_from_slice)]
/*! This is an implementation of the `Hasher` trait for the Murmur3 algorithm (32 bit output hashes, Big- and Small-Endian systems supported).

# Examples
Hash data of any primitive or custom type (which implements the `Hash` trait):

```
use std::hash::{Hash, Hasher};
use murmur3::Murmur3Hasher;

let data = 1234;
let mut hasher = Murmur3Hasher::new(0); // Seeded with 0
data.hash(&mut hasher);
let hash = hasher.finish();
```

# Notes for use with `[T]` and `str`
In case of `[T]` or `str` Rust adds a little magic to the data before hashing it in
order to prevent hash collisions in some edge cases.

For details see https://github.com/rust-lang/rust/issues/5257
and https://github.com/rust-lang/rust/issues/27108.

Thus to get the expected hash result (without the extra magic added to your data before hashing) from a `str` or `[T]`:

```
use std::hash::{Hash, Hasher};
use murmur3::Murmur3Hasher;

let mut hasher = Murmur3Hasher::new(0);
Hash::hash_slice("hello world".as_bytes(), &mut hasher);
let hash = hasher.finish();

assert_eq!(hash, 1586663183);
```

If interoperability with other Murmur3 implementations is not important
for you when hashing `str` or `[T]`, just use Murmur3Hasher as explained
above instead of passing a reference to a byte slice to `hash_slice()`:
*/
extern crate byteorder;

pub use self::murmur::Murmur3Hasher;

pub mod murmur;
