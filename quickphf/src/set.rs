//! An immutable set constructed at compile time with perfect hashing.
use core::{borrow::Borrow, hash::Hash};

// TODO: Debug impls

use crate::RawPhfMap;

/// An immutable set constructed at compile time with perfect hashing.
#[derive(Debug)]
pub struct PhfSet<K: 'static> {
    raw_map: RawPhfMap<K, K>,
}

impl<K> PhfSet<K> {
    #[doc(hidden)]
    /// This function is public because it is used by `quickphf_codegen` to
    /// instantiate the set—users should never directly write calls to it.
    pub const fn new(
        seed: u64,
        pilots_table: &'static [u16],
        elements: &'static [K],
        free: &'static [u32],
    ) -> PhfSet<K> {
        PhfSet {
            raw_map: RawPhfMap::new(seed, pilots_table, elements, free),
        }
    }
}

impl<K> PhfSet<K> {
    /// Returns the number of elements in the set.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(EVEN_DIGITS.len(), 5);
    /// assert_eq!(EMPTY_SET.len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        self.raw_map.len()
    }

    /// Returns `true` if the set does not contain any elements.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(!DIGITS.is_empty());
    /// assert!(EMPTY_SET.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the elements of the set in no particular order.
    ///   
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut items = EVEN_DIGITS.iter().copied().collect::<Vec<_>>();
    /// items.sort();
    ///
    /// assert_eq!(&items, &[0, 2, 4, 6, 8]);
    /// ```
    pub fn iter(&self) -> Iter<'_, K> {
        Iter {
            iter: self.raw_map.iter(),
        }
    }
}

impl<K: Eq + Hash> PhfSet<K> {
    /// Returns `true` if the set contains the given element.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(PRIME_DIGITS.contains(&2));
    /// assert!(!PRIME_DIGITS.contains(&1));
    /// ```
    pub fn contains<Q>(&self, element: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(element).is_some()
    }

    /// Returns a reference to the copy of the element stored in the set, if
    /// present.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert_eq!(EVEN_DIGITS.get(&2), Some(&2));
    /// assert_eq!(EVEN_DIGITS.get(&3), None);
    /// ```
    pub fn get<Q>(&self, element: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.is_empty() {
            return None;
        }

        let result = self.raw_map.get(element);
        if result.borrow() == element {
            Some(result)
        } else {
            None
        }
    }

    /// Returns an iterator over the set difference in no particular order.
    ///
    /// The iterator yields all items that are in `self` but not in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut difference = EVEN_DIGITS
    ///     .difference(&PRIME_DIGITS)
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// difference.sort();
    ///
    /// assert_eq!(&difference, &[0, 4, 6, 8]);
    /// ```
    pub fn difference<'a>(&'a self, other: &'a PhfSet<K>) -> Difference<'a, K> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    /// Returns an iterator over the intersection in no particular order.
    ///
    /// The iterator yields all items that are in both `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut intersection = PRIME_DIGITS.intersection(&EVEN_DIGITS);
    ///
    /// assert_eq!(intersection.next(), Some(&2));
    /// assert!(intersection.next().is_none());
    /// ```
    pub fn intersection<'a>(&'a self, other: &'a PhfSet<K>) -> Intersection<'a, K> {
        Intersection {
            iter: self.iter(),
            other,
        }
    }

    /// Returns an iterator over the symmetric difference of the two sets in no particular order.
    ///
    /// The iterator yields all items that are in `self` but not in `other`, or vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut symmetric_difference = PRIME_DIGITS
    ///     .symmetric_difference(&EVEN_DIGITS)
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// symmetric_difference.sort();
    ///
    /// assert_eq!(&symmetric_difference, &[0, 3, 4, 5, 6, 7, 8]);
    /// ```
    pub fn symmetric_difference<'a>(&'a self, other: &'a PhfSet<K>) -> SymmetricDifference<'a, K> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    /// Returns an iterator over the union in no particular order.
    ///
    /// The iterator yields all items that are in `self` or `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// let mut union = PRIME_DIGITS
    ///     .union(&EVEN_DIGITS)
    ///     .copied()
    ///     .collect::<Vec<_>>();
    /// union.sort();
    ///
    /// assert_eq!(&union, &[0, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a PhfSet<K>) -> Union<'a, K> {
        Union {
            iter: self.iter().chain(other.difference(self)),
        }
    }

    /// Returns `true` if `self` and `other` have no elements in common.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(!EVEN_DIGITS.is_disjoint(&PRIME_DIGITS));
    /// ```
    pub fn is_disjoint(&self, other: &PhfSet<K>) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns `true` if there are no elements in `self` that are not in `other` as well.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(EVEN_DIGITS.is_subset(&DIGITS));
    /// ```
    pub fn is_subset(&self, other: &PhfSet<K>) -> bool {
        self.difference(other).next().is_none()
    }

    /// Returns `true` if there are no elements of `other` that are not in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use quickphf::examples::*;
    ///
    /// assert!(DIGITS.is_superset(&EVEN_DIGITS));
    /// ```
    pub fn is_superset(&self, other: &PhfSet<K>) -> bool {
        other.is_subset(self)
    }
}

impl<'a, K> IntoIterator for &'a PhfSet<K> {
    type Item = &'a K;
    type IntoIter = Iter<'a, K>;

    fn into_iter(self) -> Iter<'a, K> {
        self.iter()
    }
}

impl<K: Eq + Hash> PartialEq for PhfSet<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|k| other.contains(k))
    }
}

impl<K: Eq + Hash> Eq for PhfSet<K> {}

#[derive(Clone)]
/// An iterator over the elements of a `PhfSet`.
pub struct Iter<'a, K: 'a> {
    iter: crate::raw_map::Iter<'a, K>,
}

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K> ExactSizeIterator for Iter<'a, K> {}

impl<'a, K> core::iter::FusedIterator for Iter<'a, K> {}

#[derive(Clone)]
/// A lazy iterator producing elements from the difference of two `PhfSets`s.
pub struct Difference<'a, K: 'static> {
    iter: Iter<'a, K>,
    other: &'a PhfSet<K>,
}

impl<'a, K> Iterator for Difference<'a, K>
where
    K: Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&k| !self.other.contains(k))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let self_size = self.iter.size_hint().0;
        let other_size = self.other.len();
        let min_size = self_size.saturating_sub(other_size);

        (min_size, Some(self_size))
    }
}

impl<'a, K> core::iter::FusedIterator for Difference<'a, K> where K: Eq + Hash {}

#[derive(Clone)]
/// A lazy iterator producing elements from the intersection of two `PhfSet`s.
pub struct Intersection<'a, K: 'static> {
    iter: Iter<'a, K>,
    other: &'a PhfSet<K>,
}

impl<'a, K> Iterator for Intersection<'a, K>
where
    K: Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&k| self.other.contains(k))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let self_size = self.iter.size_hint().0;
        let other_size = self.other.len();
        let max_size = usize::min(self_size, other_size);

        (0, Some(max_size))
    }
}

impl<'a, K> core::iter::FusedIterator for Intersection<'a, K> where K: Eq + Hash {}

#[derive(Clone)]
/// A lazy iterator producing elements from the symmetric difference of two `PhfSet`s.
pub struct SymmetricDifference<'a, K: 'static> {
    iter: core::iter::Chain<Difference<'a, K>, Difference<'a, K>>,
}

impl<'a, K> Iterator for SymmetricDifference<'a, K>
where
    K: Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K> core::iter::FusedIterator for SymmetricDifference<'a, K> where K: Eq + Hash {}

#[derive(Clone)]
/// A lazy iterator producing elements from the union of two `PhfSet`s.
pub struct Union<'a, K: 'static> {
    iter: core::iter::Chain<Iter<'a, K>, Difference<'a, K>>,
}

impl<'a, K> Iterator for Union<'a, K>
where
    K: Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K> core::iter::FusedIterator for Union<'a, K> where K: Eq + Hash {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::EMPTY_SET;

    #[test]
    fn test_empty() {
        assert_eq!(EMPTY_SET.get(&17), None);
        assert!(!EMPTY_SET.contains(&620));
        assert!(EMPTY_SET.iter().next().is_none())
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<PhfSet<u64>>();
    }
}
