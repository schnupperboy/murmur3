# Murmur3

[MurmurHash3](https://en.wikipedia.org/wiki/MurmurHash) is a non-cryptographic hash function suitable for general hash-based lookup. Currently only 32-bit hash values are supported.

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

Here's a simple example that calculates the 32-bit MurmurHash3 for a given &str:

```rust
extern crate murmur3;

use murmur3::{seeded, unseeded};

fn main() {
	let hash_seeded = seeded("abcd", 1234);
	assert_eq!(hash_seeded, 893017187);

	let hash_unseeded = unseeded("abcd");
	assert_eq!(hash_unseeded, 1139631978);
}
```

# License

`murmur3` is distributed under the terms of MIT license. See LICENSE.md for details.
