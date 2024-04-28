//! Type definitions for a default set.

use core::borrow::Borrow;
use core::hash::Hash;
use core::iter::FusedIterator;

#[cfg(not(feature = "no-hash-maps"))]
use crate::collections::hash;

#[cfg(not(feature = "no-hash-maps"))]
type SetImpl<T> = hashbrown::HashSet<T, hash::RandomState>;

#[cfg(feature = "no-hash-maps")]
type SetImpl<T> = alloc::collections::BTreeSet<T>;

#[cfg(not(feature = "no-hash-maps"))]
type IterImpl<'a, T> = hashbrown::hash_set::Iter<'a, T>;

#[cfg(feature = "no-hash-maps")]
type IterImpl<'a, T> = alloc::collections::btree_set::Iter<'a, T>;

/// A default set of values.
///
/// Provides a unified API between a hash-set and a btree-set.
#[derive(Debug, Clone)]
pub struct Set<T> {
    /// The underlying hash-set or btree-set data structure used.
    inner: SetImpl<T>,
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self {
            inner: SetImpl::default(),
        }
    }
}

impl<T> Set<T> {
    /// Clears the set, removing all elements.
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the number of elements in the [`Set`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the [`Set`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator that yields the items in the [`Set`].
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

impl<T> Set<T>
where
    T: Eq + Hash,
{
    /// Reserves capacity for at least `additional` more elements to be inserted in the [`Set`].
    pub fn reserve(&mut self, additional: usize) {
        #[cfg(not(feature = "no-hash-maps"))]
        self.inner.reserve(additional);
        #[cfg(feature = "no-hash-maps")]
        let _ = additional;
    }

    /// Returns true if the [`Set`] contains an element equal to the `value`.
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.contains(value)
    }

    /// Returns a reference to the element in the [`Set`], if any, that is equal to the `value`.
    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.get(value)
    }

    /// Adds `value` to the [`Set`].
    ///
    /// Returns whether the value was newly inserted:
    ///
    /// - Returns `true` if the set did not previously contain an equal value.
    /// - Returns `false` otherwise and the entry is not updated.
    pub fn insert(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        self.inner.insert(value)
    }

    /// If the set contains an element equal to the value, removes it from the [`Set`] and drops it.
    ///
    /// Returns `true` if such an element was present, otherwise `false`.
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.remove(value)
    }
}

impl<'a, T> IntoIterator for &'a Set<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`Set`].
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    inner: IterImpl<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}
