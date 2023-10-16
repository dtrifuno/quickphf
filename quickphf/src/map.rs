//! An immutable hash table constructed at compile time with perfect hashing.
use core::{borrow::Borrow, hash::Hash};

use core::fmt::Debug;

use crate::RawPhfMap;

/// An immutable hash table constructed at compile time with perfect hashing.
#[derive(Debug)]
pub struct PhfMap<K: 'static, V: 'static> {
    raw_map: RawPhfMap<K, (K, V)>,
}

impl<K, V> PhfMap<K, V> {
    #[doc(hidden)]
    /// This function is public because it is used by `quickphf_codegen` to
    /// instantiate the map—users should never directly write calls to it.
    pub const fn new(
        seed: u64,
        pilots_table: &'static [u16],
        entries: &'static [(K, V)],
        free: &'static [u32],
    ) -> PhfMap<K, V> {
        Self {
            raw_map: RawPhfMap::new(seed, pilots_table, entries, free),
        }
    }

    /// Returns references to the key and value corresponding to the supplied
    /// key, if present.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get_key_value(&81), Some((&81, &3)));
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get_key_value(&8000), None);
    /// ```
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.is_empty() {
            return None;
        }

        let item = self.raw_map.get(key);
        if item.0.borrow() == key {
            Some((&item.0, &item.1))
        } else {
            None
        }
    }

    /// Returns a reference to the value corresponding to the key, if present.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&2401), Some(&7));
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get(&1729), None);
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_key_value(key).map(|e| e.1)
    }

    /// Returns a reference to the copy of the key stored in the map, if
    /// present.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get_key(&4096), Some(&4096));
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.get_key(&4830), None);
    /// ```
    pub fn get_key<Q>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_key_value(key).map(|e| e.0)
    }

    /// Returns `true` if the map contains the given key.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    /// assert!(FOURTH_POWERS_TO_ROOTS.contains_key(&10_000));
    /// assert!(!FOURTH_POWERS_TO_ROOTS.contains_key(&24));
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_key_value(key).is_some()
    }

    /// Returns the number of elements in the map.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(FOURTH_POWERS_TO_ROOTS.len(), 10);
    /// assert_eq!(EMPTY_MAP.len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        self.raw_map.len()
    }

    /// Returns `true` if the map does not contain any elements.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(!FOURTH_POWERS_TO_ROOTS.is_empty());
    /// assert!(EMPTY_MAP.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.raw_map.is_empty()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut entries = FOURTH_POWERS_TO_ROOTS
    ///     .iter()
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// entries.sort();
    ///
    /// let expected_entries = [
    ///     (1, 1),
    ///     (16, 2),
    ///     (81, 3),
    ///     (256, 4),
    ///     (625, 5),
    ///     (1296, 6),
    ///     (2401, 7),
    ///     (4096, 8),
    ///     (6561, 9),
    ///     (10000, 10),
    /// ];
    ///
    /// assert_eq!(&entries, &expected_entries);
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.raw_map.iter(),
        }
    }

    /// An iterator visiting all stored keys in arbitrary order.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut keys = FOURTH_POWERS_TO_ROOTS
    ///     .keys()
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// keys.sort();
    ///
    /// let expected_keys = [1, 16, 81, 256, 625, 1296, 2401, 4096, 6561, 10000];
    ///
    /// assert_eq!(&keys, &expected_keys);
    /// ```
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    /// An iterator visiting all stored values in arbitrary order.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut values = FOURTH_POWERS_TO_ROOTS
    ///     .values()
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// values.sort();
    ///
    /// let expected_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    ///
    /// assert_eq!(&values, &expected_values);
    /// ```
    pub fn values(&self) -> Values<'_, K, V> {
        Values { iter: self.iter() }
    }
}

impl<K, V> PartialEq for PhfMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(k1, v1)| other.get(k1).map_or(false, |v2| *v1 == *v2))
    }
}

impl<K, V> Eq for PhfMap<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
}

#[derive(Clone)]
/// An iterator over the entries of a `PhfMap`.
pub struct Iter<'a, K: 'a, V: 'a> {
    iter: crate::raw_map::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}

impl<'a, K, V> core::iter::FusedIterator for Iter<'a, K, V> {}

#[derive(Clone)]
/// An iterator over the values of a `PhfMap`.
pub struct Values<'a, K: 'a, V: 'a> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| &t.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {}

impl<'a, K, V> core::iter::FusedIterator for Values<'a, K, V> {}

#[derive(Clone)]
/// An iterator over the keys of a `PhfMap`.
pub struct Keys<'a, K: 'a, V: 'a> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| &t.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {}

impl<'a, K, V> core::iter::FusedIterator for Keys<'a, K, V> {}

#[cfg(test)]
mod tests {
    use crate::examples::EMPTY_MAP;

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(EMPTY_MAP.get("Nosy"), None);
        assert_eq!(EMPTY_MAP.get_key("Smithy"), None);
        assert_eq!(EMPTY_MAP.get_key_value("Nighteyes"), None);

        assert!(EMPTY_MAP.iter().next().is_none());
        assert!(EMPTY_MAP.keys().next().is_none());
        assert!(EMPTY_MAP.values().next().is_none());
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<PhfMap<u64, u64>>();
    }
}
