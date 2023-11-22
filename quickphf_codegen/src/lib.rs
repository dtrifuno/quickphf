#![forbid(unsafe_code)]

//! `quickphf_codegen` is a Rust crate that allows you to generate static hash
//! maps and hash sets at compile-time using
//! [PTHash perfect hash functions](https://arxiv.org/abs/2104.10402).
//!
//! For the runtime code necessary to use these data structures check out the
//! [`quickphf` crate](quickphf) instead.
//!
//! The minimum supported Rust version is 1.56. This crate uses
//! `#![forbid(unsafe_code)]`.
//!
//! **WARNING**: `quickphf` and `quickphf_codegen` currently use the standard
//! library [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)
//! trait which is not portable between platforms with different endianness.
//!
//! ## Example
//!
//! Currently, the only way to generate data structures for use with `quickphf`
//! is by running one of [`build_raw_map`], [`build_map`], or [`build_set`],
//! displaying the result as a string, and then importing the resulting Rust
//! code at the desired location.
//!
//! For example, you can write a
//! [`build.rs` script](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
//! such as:
//!
//! ```ignore
//! use std::env;
//! use std::fs::File;
//! use std::io::{BufWriter, Write};
//! use std::path::Path;
//!
//! fn main() {
//!     let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
//!     let mut file = BufWriter::new(fs::File::create(&path).unwrap());
//!
//!     let keys = ["jpg", "png", "svg"];
//!     let values = ["image/jpeg", "image/png", "image/svg+xml"];
//!     let code = quickphf_codegen::build_map(&keys, &values);
//!
//!     write!(&mut file, code).unwrap();
//! }
//! ```
//!
//! and then import the result in your `lib.rs` by:
//!
//! ```ignore
//! static MIME_TYPES: quickphf::PhfMap<&'static str, &'static str> =
//!     include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
//! ```
//!
//! ## Advanced Usage
//!
//! ### Using QuickPHF with custom types
//!
//! To be usable as a key in a `PhfMap` or `PhfSet`, or as value in a `RawPhfMap`
//! or a `PhfMap`, a type must implement the trait [`ConstInstantiable`], which
//! provides a way to generate code which instantiates any value of that type in
//! a `const` context. This trait is already implemented for many built-in types,
//! but users can also implement it for their own custom types, by one of two ways:
//!
//! 1. If the code required to instantiate a value of a type is identical to its
//! `Debug` representation, for example, like the following enum:
//!
//! ```ignore
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! enum PositionType {
//!     Contract { pub hours_per_week: u32 },
//!     Salaried,
//!     Managerial,
//! }
//! ```
//!
//! then it suffices to write
//!
//! ```ignore
//! impl quickphf_codegen::DebugInstantiable for PositionType {}
//! ```
//!
//! 2. Otherwise, the user has to provide a custom implementation. For example,
//!    the following struct has private fields and thus its values cannot be
//!    instantiated using the `{}` syntax, but provides a `new` constructor
//!    that is a `const fn`. Thus, given
//!
//! ```ignore
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! struct EmploymentRules {
//!     overtime_eligible: bool,
//!     bonus_eligible: bool,
//! }
//!
//! impl EmploymentRules {
//!     pub const fn new(overtime_eligible: bool, bonus_eligible: bool) -> EmploymentRules {
//!         EmploymentRules {
//!             overtime_eligible,
//!             bonus_eligible,
//!         }
//!     }
//! }
//! ```
//!
//! we can provide a custom `ConstInstantiable` implementation by
//!
//! ```ignore
//! use core::fmt;
//! use quickphf_codegen::*;
//!
//! impl ConstInstantiable for EmploymentRules {
//!     fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(
//!             f,
//!             "EmploymentRules::new({}, {})",
//!             self.overtime_eligible, self.bonus_eligible
//!         )
//!     }
//! }
//! ```

use core::fmt;
use core::hash::Hash;

use phf::{generate_phf, Phf};

mod const_instantiable;
pub mod phf;

pub use const_instantiable::ConstInstantiable;
pub use const_instantiable::DebugInstantiable;

/// Generate code for a static [`quickphf::RawPhfMap`].
///
/// # Examples
///
/// ```
/// use quickphf_codegen::*;
///
/// let months = [
///     "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
/// ];
/// let holidays = [2, 1, 0, 0, 0, 1, 1, 0, 1, 1, 2, 1];
/// let holidays_per_month = build_raw_map(&months, &holidays);
/// ```
pub fn build_raw_map<'a, K: Eq + Hash, V: ConstInstantiable>(
    keys: &'a [K],
    values: &'a [V],
) -> CodeWriter<'a, K, V> {
    let phf = generate_phf::<K>(keys);
    CodeWriter {
        kind: Kind::RawMap,
        phf,
        keys: &[],
        values,
    }
}

/// Generate code for a static [`quickphf::PhfMap`].
///
/// # Examples
///
/// ```
/// use quickphf_codegen::*;
///
/// let roots = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let fourth_powers = roots.map(|x| x * x * x * x);
/// let powers_to_roots = build_map(&fourth_powers, &roots);
/// ```
pub fn build_map<'a, K: Eq + Hash + ConstInstantiable, V: ConstInstantiable>(
    keys: &'a [K],
    values: &'a [V],
) -> CodeWriter<'a, K, V> {
    let phf = generate_phf(keys);
    CodeWriter {
        kind: Kind::Map,
        phf,
        keys,
        values,
    }
}

/// Generate code for a static [`quickphf::PhfSet`].
///
/// # Examples
///
/// ```
/// use quickphf_codegen::*;
/// let digits_set = build_set(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
pub fn build_set<K: Eq + Hash + ConstInstantiable>(keys: &[K]) -> CodeWriter<'_, K> {
    let phf = generate_phf(keys);
    CodeWriter {
        kind: Kind::Set,
        phf,
        keys,
        values: &[],
    }
}

enum Kind {
    RawMap,
    Map,
    Set,
}

/// Code generator for a PTHash perfect hash function hash table structure.
pub struct CodeWriter<'a, K, V = ()> {
    kind: Kind,
    phf: Phf,
    keys: &'a [K],
    values: &'a [V],
}

impl<'a, K: ConstInstantiable, V: ConstInstantiable> fmt::Display for CodeWriter<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}

impl<'a, K: ConstInstantiable, V: ConstInstantiable> CodeWriter<'a, K, V> {
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = match self.kind {
            Kind::RawMap => "RawPhfMap",
            Kind::Map => "PhfMap",
            Kind::Set => "PhfSet",
        };

        writeln!(f, "::quickphf::{}::new(", type_name)?;
        writeln!(f, "    {},", self.phf.seed)?;

        write!(f, "    &")?;
        self.write_slice(self.phf.pilots_table.iter(), f)?;
        writeln!(f, ",")?;

        //
        let mut prev_entry = false;
        write!(f, "    &")?;
        write!(f, "[")?;

        for &idx in &self.phf.map {
            if prev_entry {
                write!(f, ", ")?;
            } else {
                prev_entry = true;
            }

            match self.kind {
                Kind::Map => {
                    let key = &self.keys[idx as usize];
                    let value = &self.values[idx as usize];

                    write!(f, "(")?;
                    key.fmt_const_new(f)?;
                    write!(f, ", ")?;
                    value.fmt_const_new(f)?;
                    write!(f, ")")?;
                }
                Kind::RawMap => {
                    self.values[idx as usize].fmt_const_new(f)?;
                }
                Kind::Set => {
                    self.keys[idx as usize].fmt_const_new(f)?;
                }
            }
        }
        writeln!(f, "],")?;

        write!(f, "    &")?;
        self.write_slice(self.phf.free.iter(), f)?;
        writeln!(f)?;

        write!(f, ")")
    }

    fn write_slice<T: ConstInstantiable + 'a>(
        &'a self,
        entries: impl Iterator<Item = &'a T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "[")?;

        let mut prev_entry = false;
        for entry in entries {
            if prev_entry {
                write!(f, ", ")?;
            } else {
                prev_entry = true;
            }

            entry.fmt_const_new(f)?;
        }

        write!(f, "]")
    }
}
