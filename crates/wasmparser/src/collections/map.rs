//! Type definitions for a default map.

use core::borrow::Borrow;
use core::hash::Hash;
use core::iter::FusedIterator;

#[cfg(not(feature = "no-hash-maps"))]
mod detail {
    use crate::collections::hash;
    use hashbrown::{hash_map, HashMap};

    pub type MapImpl<K, V> = HashMap<K, V, hash::RandomState>;
    pub type EntryImpl<'a, K, V> = hash_map::Entry<'a, K, V, hash::RandomState>;
    pub type OccupiedEntryImpl<'a, K, V> = hash_map::OccupiedEntry<'a, K, V, hash::RandomState>;
    pub type VacantEntryImpl<'a, K, V> = hash_map::VacantEntry<'a, K, V, hash::RandomState>;
    pub type IterImpl<'a, K, V> = hash_map::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = hash_map::IterMut<'a, K, V>;
}

#[cfg(feature = "no-hash-maps")]
mod detail {
    use alloc::collections::{btree_map, BTreeMap};

    pub type MapImpl<K, V> = BTreeMap<K, V>;
    pub type EntryImpl<'a, K, V> = btree_map::Entry<'a, K, V>;
    pub type OccupiedEntryImpl<'a, K, V> = btree_map::OccupiedEntry<'a, K, V>;
    pub type VacantEntryImpl<'a, K, V> = btree_map::VacantEntry<'a, K, V>;
    pub type IterImpl<'a, K, V> = btree_map::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = btree_map::IterMut<'a, K, V>;
}

/// A default key-value mapping.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    inner: detail::MapImpl<K, V>,
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self {
            inner: detail::MapImpl::default(),
        }
    }
}

impl<K, V> Map<K, V> {
    /// Clears the map, removing all elements.
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the number of elements in the [`Map`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the [`Map`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator that yields the items in the [`Map`].
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// Returns an iterator that yields the mutable items in the [`Map`].
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.inner.iter_mut(),
        }
    }
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    /// Returns a reference to the value corresponding to the `key`.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.get(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.get_mut(key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        self.inner.insert(key, value)
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.remove(key)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: Ord,
    {
        match self.inner.entry(key) {
            detail::EntryImpl::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            detail::EntryImpl::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This enum is constructed from the entry method on [`Map`].
#[derive(Debug)]
pub enum Entry<'a, K: Ord, V> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Ord,
{
    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &K {
        match *self {
            Self::Occupied(ref entry) => entry.key(),
            Self::Vacant(ref entry) => entry.key(),
        }
    }
}

/// A view into an occupied entry in a [`Map`].
///
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, K: Ord, V> {
    inner: detail::OccupiedEntryImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V>
where
    K: Ord,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        self.inner.get()
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    /// Sets the value of the entry with the [`OccupiedEntry`]'s key, and returns the entry's old value.
    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    /// Converts the [`OccupiedEntry`] into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    /// Takes the value out of the entry, and returns it.
    pub fn remove(self) -> V {
        self.inner.remove()
    }
}

/// A view into a vacant entry in a [`Map`].
///
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct VacantEntry<'a, K: Ord, V> {
    inner: detail::VacantEntryImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V>
where
    K: Ord,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Take ownership of the key.
    pub fn into_key(self) -> K {
        self.inner.into_key()
    }

    /// Sets the value of the entry with the [`VacantEntry`]'s key, and returns a mutable reference to it.
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
    {
        self.inner.insert(value)
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`Map`].
#[derive(Debug, Clone)]
pub struct Iter<'a, K, V> {
    inner: detail::IterImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

impl<'a, K, V> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An iterator over the mutable items of a [`Map`].
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    inner: detail::IterMutImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> {}
