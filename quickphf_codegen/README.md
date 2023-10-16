# QuickPHF-Codegen

[![Latest Release]][crates.io] [![Documentation]][docs.rs] ![Minimum Supported Rust Version 1.56]

[Latest Release]: https://img.shields.io/crates/v/quickphf_codegen.svg
[crates.io]: https://crates.io/crates/quickphf_codegen
[Documentation]: https://docs.rs/quickphf_codegen/badge.svg
[docs.rs]: https://docs.rs/quickphf_codegen/
[Minimum Supported Rust Version 1.56]: https://img.shields.io/badge/MSRV-1.56-blue.svg

`quickphf_codegen` is a Rust crate that allows you to generate static hash maps and
hash sets at compile-time using
[PTHash perfect hash functions](https://arxiv.org/abs/2104.10402).

For the runtime code necessary to use these data structures check out the
[`quickphf` crate](https://crates.io/crates/quickphf) instead.

The minimum supported Rust version is 1.56. This crate uses `#![forbid(unsafe_code)]`.

**WARNING**: `quickphf` and `quickphf_codegen` currently use the standard library
[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)
trait which is not portable between systems with different endianness.

## Example

Currently, the only way to generate data structures for use with `quickphf` is by running
one of
[`build_raw_map`](https://docs.rs/quickphf_codegen/latest/quickphf_codegen/fn.build_raw_map.html),
[`build_map`](https://docs.rs/quickphf_codegen/latest/quickphf_codegen/fn.build_map.html),
or [`build_set`](https://docs.rs/quickphf_codegen/latest/quickphf_codegen/fn.build_set.html),
displaying the result as a string, and then importing the resulting Rust code at the desired location.

For example, you can write a [`build.rs` script](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
such as:

```rust
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(fs::File::create(&path).unwrap());

    let keys = ["jpg", "png", "svg"];
    let values = ["image/jpeg", "image/png", "image/svg+xml"];
    let code = quickphf_codegen::build_map(&keys, &values);

    write!(&mut file, code).unwrap();
}
```

and then import the result in your `lib.rs` by:

```rust
static MIME_TYPES: quickphf::PhfMap<&'static str, &'static str> =
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
```

## Advanced Usage

### Using QuickPHF with custom types

To be usable as a key in a `PhfMap` or `PhfSet`, or as value in a `RawPhfMap` or a `PhfMap`, a type must implement
the trait `ConstInstantiable`, which provides a way to generate code which instantiates any value of that type in
a `const` context. This trait is already implemented for many built-in types, but users can also implement it for
their own custom types, by one of two ways:

1. If the code required to instantiate a value of a type is identical to its `Debug` representation, for example,
   like the following enum:

```rust
#[derive(Debug, Hash, PartialEq, Eq)]
enum PositionType {
    Contract { hours_per_week: u32 },
    Salaried,
    Managerial,
}
```

then it suffices to write

```rust
impl quickphf_codegen::DebugInstantiable for PositionType {}
```

2. The user has to provide a custom implementation. For example, the following struct has
   private fields and thus its values cannot be instantiated using the `{}` syntax, but provides
   a `new` constructor that is `const fn`. Thus, given

```rust
#[derive(Debug, Hash, PartialEq, Eq)]
struct EmploymentRules {
    overtime_eligible: bool,
    bonus_eligible: bool,
}

impl EmploymentRules {
    pub const fn new(overtime_eligible: bool, bonus_eligible: bool) -> EmploymentRules {
        EmploymentRules {
            overtime_eligible,
            bonus_eligible,
        }
    }
}
```

we can provide a custom `ConstInstantiable` implementation by

```rust
use core::fmt;
use quickphf_codegen::*;

impl ConstInstantiable for EmploymentRules {
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmploymentRules::new({}, {})",
            self.overtime_eligible, self.bonus_eligible
        )
    }
}
```

## Performance

Generating a PHF-based data structure with `quickphf_codegen` is about 10 times faster than
with [`phf`](https://crates.io/crates/phf). It can generate a 1,000,000 entry map in about
300 ms.

![](https://github.com/dtrifuno/quickphf/blob/main/benchmarks/generate.png?raw=true)

## License

Licensed under any of:

- Apache License, Version 2.0, ([LICENSE-APACHE](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- zlib License ([LICENSE-ZLIB](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-ZLIB) or <https://opensource.org/license/zlib/>)

by your choice.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be multi-licensed as above, without any additional terms or conditions.
