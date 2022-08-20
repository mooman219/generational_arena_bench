use crate::Crate;
use criterion::{black_box, BatchSize, Bencher};
use stash::{Stash, UniqueStash};

pub struct CrateStash();
impl Crate for CrateStash {
    fn name(&self) -> &'static str {
        "Stash"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let map: Stash<usize, usize> = Stash::new();
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut map: Stash<usize, usize> = Stash::new();
        for a in 0..size {
            map.put(a);
        }
        for a in 0..size {
            map.take(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, size: usize) {
        let mut map: Stash<usize, usize> = Stash::new();
        for a in 0..size {
            map.put(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.take(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: Stash<usize, usize> = Stash::new();
        for a in 0..size {
            map.put(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    black_box(i.get(lookup[a]));
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn iterate(&self, b: &mut Bencher, size: usize) {
        let mut map: Stash<usize, usize> = Stash::new();
        for a in 0..size {
            map.put(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reiterate(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: Stash<usize, usize> = Stash::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.put(a));
        }
        for i in lookup {
            map.take(keys[*i]);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }
}

pub struct CrateUniqueStash();
impl Crate for CrateUniqueStash {
    fn name(&self) -> &'static str {
        "UniqueStash"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let map: UniqueStash<usize> = UniqueStash::new();
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut map: UniqueStash<usize> = UniqueStash::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.put(a));
        }
        for a in 0..size {
            map.take(keys[a]);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, size: usize) {
        let mut map: UniqueStash<usize> = UniqueStash::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.put(a));
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.take(keys[a]);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: UniqueStash<usize> = UniqueStash::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.put(a));
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    black_box(i.get(keys[lookup[a]]));
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn iterate(&self, b: &mut Bencher, size: usize) {
        let mut map: UniqueStash<usize> = UniqueStash::new();
        for a in 0..size {
            map.put(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reiterate(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: UniqueStash<usize> = UniqueStash::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.put(a));
        }
        for i in lookup {
            map.take(keys[*i]);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }
}
