//! An immutable hash table constructed at compile time with perfect hashing which does not store its keys.

use core::borrow::Borrow;
use core::hash::Hash;
use core::marker::PhantomData;

use quickdiv::DivisorU64;

use crate::shared::*;

/// An immutable hash table constructed at compile time with perfect hashing which does not store its keys.
#[derive(Debug)]
pub struct RawPhfMap<K, V: 'static> {
    codomain_len: DivisorU64,
    buckets: DivisorU64,
    seed: u64,

    pilots_table: &'static [u16],
    values: &'static [V],
    free: &'static [u32],

    key_marker: PhantomData<K>,
}

impl<K, V> RawPhfMap<K, V> {
    #[doc(hidden)]
    /// This function is public because it is used by `quickphf_codegen` to
    /// instantiate the map—users should never directly write calls to it.
    pub const fn new(
        seed: u64,
        pilots_table: &'static [u16],
        values: &'static [V],
        free: &'static [u32],
    ) -> RawPhfMap<K, V> {
        let codomain_len = DivisorU64::new((values.len() + free.len()) as u64);
        let buckets = DivisorU64::new(pilots_table.len() as u64);

        RawPhfMap {
            codomain_len,
            buckets,
            seed,

            pilots_table,
            values,
            free,

            key_marker: PhantomData,
        }
    }

    /// Returns a reference to the value matching the given key.
    ///
    /// If `key` is not one of the keys that was used when constructing the map,
    /// `get` will silently return an arbitrary value. If robustness to invalid
    /// keys is needed, use a [PhfMap][crate::PhfMap] instead.
    ///
    /// # Panics
    ///
    /// Panics if the `RawPhfMap` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(*HOLIDAYS_PER_MONTH.get("jan"), 2);
    ///
    /// // Looking up an invalid key silently returns an incorrect value.
    /// let result = HOLIDAYS_PER_MONTH.get("purple");
    /// ```
    pub fn get<Q>(&self, key: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let key_hash = hash_key(key.borrow(), self.seed);

        let bucket = get_bucket(key_hash, self.buckets);
        let pilot_hash = hash_pilot_value(self.pilots_table[bucket]);
        let idx = get_index(key_hash, pilot_hash, self.codomain_len);

        if idx < self.len() {
            &self.values[idx]
        } else {
            &self.values[self.free[idx - self.len()] as usize]
        }
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(HOLIDAYS_PER_MONTH.len(), 12);
    /// assert_eq!(EMPTY_RAW_MAP.len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the map does not contain any elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(!HOLIDAYS_PER_MONTH.is_empty());
    /// assert!(EMPTY_RAW_MAP.is_empty())
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// An iterator visiting all the values stored in the map in an arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut values = HOLIDAYS_PER_MONTH
    ///     .iter()
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// values.sort();
    ///
    /// assert_eq!(&values, &[0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2]);
    /// ```
    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            iter: self.values.iter(),
        }
    }
}

#[derive(Clone)]
/// An iterator over the values of a `RawPhfMap`.
pub struct Iter<'a, V: 'a> {
    iter: core::slice::Iter<'a, V>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use crate::examples::EMPTY_RAW_MAP;

    use super::*;

    #[test]
    #[should_panic]
    fn test_get_from_empty() {
        EMPTY_RAW_MAP.get("Lenar");
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RawPhfMap<u64, u64>>();
    }
}
