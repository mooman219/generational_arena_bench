use core::marker::PhantomData;
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
    inner: u32,
}

#[derive(Clone)]
pub struct UniqueNaiveSlotMap<K, T> {
    slots: Vec<Slot>,
    data: Vec<T>,
    key_type: PhantomData<K>,
}

impl<K, T> UniqueNaiveSlotMap<K, T> {
    pub fn new() -> UniqueNaiveSlotMap<K, T> {
        UniqueNaiveSlotMap {
            slots: Vec::new(),
            data: Vec::new(),
            key_type: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> UniqueNaiveSlotMap<K, T> {
        UniqueNaiveSlotMap {
            slots: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            key_type: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        let mut counter = 0;
        for slot in &mut self.slots {
            slot.version = slot.version.wrapping_add(1);
            slot.inner = counter;
            counter += 1;
        }
    }

    pub fn insert(&mut self, value: T) -> Key {
        let index = self.data.len() as u32;
        self.data.push(value);
        if index as usize == self.slots.len() {
            self.slots.push(Slot {
                outer: index,
                version: 0,
                inner: index,
            });
            Key { idx: index, ver: 0 }
        } else {
            unsafe {
                let key_index = self.slots.get_unchecked(index as usize).inner;
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

    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = self.slots.get_mut(key.idx as usize)?;
        if slot.version != key.ver {
            return None;
        }
        let remove_index = slot.outer;
        slot.version += slot.version.wrapping_add(1);
        let removed = self.data.swap_remove(remove_index as usize);
        unsafe {
            let slot = self.slots.get_unchecked_mut(self.data.len());
            let update_index = slot.inner;
            slot.inner = key.idx;
            self.slots.get_unchecked_mut(remove_index as usize).inner = update_index;
            self.slots.get_unchecked_mut(update_index as usize).outer = remove_index;
            Some(removed)
        }
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        let slot = unsafe { self.slots.get_unchecked(key.idx as usize) };
        if slot.version != key.ver {
            return None;
        }
        unsafe { Some(self.data.get_unchecked(slot.outer as usize)) }
    }

    // pub fn get(&self, key: Key) -> Option<&T> {
    //     let slot = self.slots.get(key.idx as usize)?;
    //     if slot.version != key.ver {
    //         return None;
    //     }
    //     unsafe { Some(self.data.get_unchecked(slot.outer as usize)) }
    // }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = self.slots.get(key.idx as usize)?;
        if slot.version != key.ver {
            return None;
        }
        unsafe { Some(self.data.get_unchecked_mut(slot.outer as usize)) }
    }

    pub unsafe fn get_unchecked(&self, key: Key) -> Option<&T> {
        let slot = self.slots.get_unchecked(key.idx as usize);
        if slot.version != key.ver {
            return None;
        }
        Some(self.data.get_unchecked(slot.outer as usize))
    }

    pub unsafe fn get_unchecked_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = self.slots.get_unchecked(key.idx as usize);
        if slot.version != key.ver {
            return None;
        }
        Some(self.data.get_unchecked_mut(slot.outer as usize))
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }
}

impl<K, T> Index<Key> for UniqueNaiveSlotMap<K, T> {
    type Output = T;

    fn index(&self, key: Key) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl<K, T> IndexMut<Key> for UniqueNaiveSlotMap<K, T> {
    fn index_mut(&mut self, key: Key) -> &mut Self::Output {
        self.get_mut(key).unwrap()
    }
}

impl<'a, K, T> IntoIterator for &'a UniqueNaiveSlotMap<K, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, K, T> IntoIterator for &'a mut UniqueNaiveSlotMap<K, T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
