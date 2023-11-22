//! QuickPHF is a Rust crate that allows you to use static compile-time
//! generated hash maps and hash sets using
//! [PTHash perfect hash functions](https://arxiv.org/abs/2104.10402).
//!
//! This crate only contains the runtime code required for using such structures.
//! To generate them, look at the [`quickphf_codegen` crate](https://docs.rs/quickphf_codegen/latest/quickphf_codegen)
//! instead.
//!
//! The minimum supported Rust version is 1.56. This crate is `#![no_std]` and
//! `#![forbid(unsafe_code)]`.
//!
//! ## Features
//!
//! - Provides 3 perfect hash function-backed data structures: [`PhfMap`] and [`PhfSet`],
//!   which, for ease of use, mimic the immutable part of the interface of the
//!   standard library `HashMap` and `HashSet`; and [`RawPhfMap`], which is a hash map
//!   that does not store its keys.
//! - About twice as fast as [`phf`](https://docs.rs/phf/latest/phf/) at lookup, and
//!   more than 10 times faster at construction.
//! - Uses a [Rust implementation](https://docs.rs/wyhash/latest/wyhash/) of
//!   Wang Yi's `wyhash` algorithm for hashing.
//! - Uses the [`quickdiv`](https://docs.rs/quickdiv/latest/quickdiv/) crate to speed up
//!   modulo computations.
//! - Very low memory usage: no unused capacity and less than a byte of overhead
//!   per entry.
//!
//! ## Example
//!
//! ```rust
//! use quickphf::examples::*;
//!
//! // You can use `PhfMap` or `PhfSet` just like `HashMap` or `HashSet`.
//! assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&4096), Some(&8));
//! assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&17), None);
//!
//! assert!(PRIME_DIGITS.contains(&3));
//! assert_eq!(PRIME_DIGITS.len(), 4);
//!
//! // With a `RawPhfMap` you would mostly use the `get` method. Note
//! // that it directly returns a &T instead of an Option<&T>.
//! assert_eq!(HOLIDAYS_PER_MONTH.get("jul"), &1);
//!
//! // If you query for an invalid key, it will silently return an
//! // arbitrary answer.
//! let valid_reference = HOLIDAYS_PER_MONTH.get("purple");
//! ```

#![no_std]
#![allow(clippy::unreadable_literal)]

#[doc(hidden)]
pub mod examples;

pub mod raw_map;

#[doc(inline)]
pub use crate::raw_map::RawPhfMap;

pub mod map;

#[doc(inline)]
pub use crate::map::PhfMap;

pub mod set;

#[doc(inline)]
pub use crate::set::PhfSet;

#[doc(hidden)]
pub mod shared;
