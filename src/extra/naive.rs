use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Key {
    idx: u32,
    ver: u16,
}

#[derive(Clone)]
struct Slot {
    outer: u32,
    version: u16,
}

#[derive(Clone)]
pub struct NaiveSlotMap<T> {
    slots: Vec<Slot>,
    data: Vec<T>,
    inner: Vec<u32>,
}

impl<T> NaiveSlotMap<T> {
    pub fn new() -> NaiveSlotMap<T> {
        NaiveSlotMap {
            slots: Vec::new(),
            data: Vec::new(),
            inner: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> NaiveSlotMap<T> {
        NaiveSlotMap {
            slots: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        for slot in &mut self.slots {
            slot.version += 1;
        }
        let mut counter = 0;
        for slot in &mut self.inner {
            *slot = counter;
            counter += 1;
        }
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> Key {
        let index = self.data.len() as u32;
        self.data.push(value);
        if index as usize == self.slots.len() {
            self.slots.push(Slot {
                outer: index,
                version: 0,
            });
            self.inner.push(index);
            Key { idx: index, ver: 0 }
        } else {
            unsafe {
                let key_index = *self.inner.get_unchecked(index as usize);
                let slot = self.slots.get_unchecked_mut(key_index as usize);
                let version = slot.version;
                slot.outer = index;
                Key {
                    idx: key_index,
                    ver: version,
                }
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = self.slots.get_mut(key.idx as usize)?;
        if slot.version != key.ver {
            return None;
        }
        let remove_index = slot.outer;
        slot.version += 1;
        let removed = self.data.swap_remove(remove_index as usize);
        unsafe {
            let slot = self.inner.get_unchecked_mut(self.data.len());
            let update_index = *slot;
            *slot = key.idx;
            *self.inner.get_unchecked_mut(remove_index as usize) = update_index;
            self.slots.get_unchecked_mut(update_index as usize).outer = remove_index;
            Some(removed)
        }
    }

    #[inline]
    pub fn get(&self, key: Key) -> Option<&T> {
        let slot = self.slots.get(key.idx as usize)?;
        if slot.version != key.ver {
            return None;
        }
        unsafe { Some(self.data.get_unchecked(slot.outer as usize)) }
    }

    #[inline]
    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = self.slots.get(key.idx as usize)?;
        if slot.version != key.ver {
            return None;
        }
        unsafe { Some(self.data.get_unchecked_mut(slot.outer as usize)) }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, key: Key) -> Option<&T> {
        let slot = self.slots.get_unchecked(key.idx as usize);
        if slot.version != key.ver {
            return None;
        }
        Some(self.data.get_unchecked(slot.outer as usize))
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = self.slots.get_unchecked(key.idx as usize);
        if slot.version != key.ver {
            return None;
        }
        Some(self.data.get_unchecked_mut(slot.outer as usize))
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T> Index<Key> for NaiveSlotMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, key: Key) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl<T> IndexMut<Key> for NaiveSlotMap<T> {
    #[inline]
    fn index_mut(&mut self, key: Key) -> &mut Self::Output {
        self.get_mut(key).unwrap()
    }
}

impl<'a, T> IntoIterator for &'a NaiveSlotMap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NaiveSlotMap<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

// ////////////////////////////////////////////////////////////////////////////
// Tests
// ////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn add_get() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);

        assert_eq!(map.get(first), Some(&0));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn add_clear() {
        let mut map = NaiveSlotMap::<usize>::new();
        let _first = map.insert(0);
        map.clear();

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn add_twice_get_second() {
        let mut map = NaiveSlotMap::<usize>::new();
        let _first = map.insert(0);
        let second = map.insert(1);

        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn add_twice_get_first() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let _second = map.insert(1);

        assert_eq!(map.get(first), Some(&0));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn add_remove() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let index = map.remove(first);

        assert_eq!(index, Some(0));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn add_remove_old_key() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        map.remove(first);

        assert_eq!(map.len(), 0);
        assert_eq!(map.remove(first), None);
    }

    #[test]
    fn add_get_old_key() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        map.remove(first);

        assert_eq!(map.len(), 0);
        assert_eq!(map.get(first), None);
    }

    #[test]
    fn add_twice_remove_second() {
        let mut map = NaiveSlotMap::<usize>::new();
        let _first = map.insert(0);
        let second = map.insert(1);
        let index = map.remove(second);

        assert_eq!(index, Some(1));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn add_twice_remove_first() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let _second = map.insert(1);
        let index = map.remove(first);

        assert_eq!(index, Some(0));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn add_twice_remove_first_swaps() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        map.remove(first);

        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn add_thrice_remove_first_swaps_ignores_second() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        let _third = map.insert(2);
        map.remove(first);

        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn complex() {
        let mut map = NaiveSlotMap::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);

        assert_eq!(map.get(first), Some(&0));
        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.len(), 2);

        map.remove(first);

        assert_eq!(map.get(first), None);
        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.len(), 1);

        let third = map.insert(2);

        assert_eq!(map.get(third), Some(&2));
        assert_eq!(map.len(), 2);

        let forth = map.insert(3);

        assert_eq!(map.get(forth), Some(&3));
        assert_eq!(map.len(), 3);

        map.remove(second);
        map.remove(third);

        assert_eq!(map.get(second), None);
        assert_eq!(map.get(third), None);
        assert_eq!(map.get(forth), Some(&3));
        assert_eq!(map.len(), 1);

        let first = map.insert(0);
        let second = map.insert(1);
        let third = map.insert(2);
        let forth = map.insert(3);

        assert_eq!(map.get(first), Some(&0));
        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.get(third), Some(&2));
        assert_eq!(map.get(forth), Some(&3));
        assert_eq!(map.len(), 5);

        map.clear();
        let first = map.insert(0);
        let second = map.insert(1);
        let third = map.insert(2);
        let forth = map.insert(3);

        assert_eq!(map.get(first), Some(&0));
        assert_eq!(map.get(second), Some(&1));
        assert_eq!(map.get(third), Some(&2));
        assert_eq!(map.get(forth), Some(&3));
        assert_eq!(map.len(), 4);
    }
}
