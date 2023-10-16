# QuickPHF

[![Latest Release]][crates.io] [![Documentation]][docs.rs] ![Minimum Supported Rust Version 1.56]

[Latest Release]: https://img.shields.io/crates/v/quickphf.svg
[crates.io]: https://crates.io/crates/quickphf
[Documentation]: https://docs.rs/quickphf/badge.svg
[docs.rs]: https://docs.rs/quickphf/
[Minimum Supported Rust Version 1.56]: https://img.shields.io/badge/MSRV-1.56-blue.svg

QuickPHF is a Rust crate that allows you to use static compile-time
generated hash maps and hash sets using
[PTHash perfect hash functions](https://arxiv.org/abs/2104.10402).

This crate only contains the runtime code required for using such structures.
To generate them, look at the [`quickphf_codegen` crate](https://crates.io/crates/quickphf_codegen)
instead.

The minimum supported Rust version is 1.56. This crate is `#![no_std]` and
`#![forbid(unsafe_code)]`.

## Features

- Provides 3 perfect hash function-backed data structures: `PhfMap` and `PhfSet`,
  which, for ease of use, mimic the immutable part of the interface of the
  standard library `HashMap` and `HashSet`; and `RawPhfMap`, which is a hash
  map that does not store its keys.
- About twice as fast as [`phf`](https://crates.io/crates/phf) at lookup, and
  more than 10 times faster at construction.
- Uses a [Rust implementation](https://crates.io/crates/wyhash) of
  Wang Yi's `wyhash` algorithm for hashing.
- Uses the [`quickdiv`](https://crates.io/crates/quickdiv) crate to speed up
  modulo computations.
- Very low memory usage: no unused capacity and less than a byte of overhead
  per entry.

## Example

```rust
use quickphf::examples::*;

// You can use `PhfMap` or `PhfSet` just like `HashMap` or `HashSet`.
assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&4096), Some(&8));
assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&17), None);

assert!(PRIME_DIGITS.contains(&3));
assert_eq!(PRIME_DIGITS.len(), 4);

// With a `RawPhfMap` you would mostly use the `get` method. Note
// that it directly returns a &T instead of an Option<&T>.
assert_eq!(HOLIDAYS_PER_MONTH.get("jul"), &1);

// If you query for an invalid key, it will silently return an
// arbitrary answer.
let valid_reference = HOLIDAYS_PER_MONTH.get("purple");
```

## Performance

In general, `quickphf` is about twice as fast as `phf` at lookup, with `RawPhfMap`
being faster than `PhfMap`, especially for larger hash maps.

![](https://github.com/dtrifuno/quickphf/blob/main/benchmarks/lookup.png?raw=true)

## License

Licensed under any of:

- Apache License, Version 2.0, ([LICENSE-APACHE](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- zlib License ([LICENSE-ZLIB](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-ZLIB) or <https://opensource.org/license/zlib/>)

at your choice.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be multi-licensed as above, without any additional terms or conditions.
