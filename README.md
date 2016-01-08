# Murmur3 (32 bit)

[MurmurHash3](https://en.wikipedia.org/wiki/MurmurHash) is a non-cryptographic hash function suitable for general hash-based lookup.
This crate supports one-shot or progressive hashing of any primitive or custom type (which implements the `Hash` trait) on Big- and Small-Endian systems.

### Documentation

[Module documentation with examples](https://schnupperboy.github.io/murmur3/murmur3/).

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
murmur3 = { git = "https://github.com/schnupperboy/murmur3.git" }
```

and this to your crate root:

```rust
extern crate murmur3;
```

Here's a simple example that calculates a 32 bit MurmurHash3:

```rust
extern crate murmur3;

use std::hash::{Hash, Hasher};
use murmur3::Murmur3Hasher;

let data = 1234;
let mut hasher = Murmur3Hasher::new(0); // Seeded with 0
data.hash(&mut hasher);
let hash = hasher.finish();
```

For details especially on how to use for `str` and `[T]` data see the documentation.

# License

`murmur3` is distributed under the terms of MIT license. See LICENSE.md for details.
