use crate::Crate;
use criterion::{black_box, BatchSize, Bencher};
use id_vec::IdVec;

pub struct CrateIdVec();
impl Crate for CrateIdVec {
    fn name(&self) -> &'static str {
        "IdVec"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let map: IdVec<usize> = IdVec::new();
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut map: IdVec<usize> = IdVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.insert(a));
        }
        for a in 0..size {
            map.remove(keys[a]);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: IdVec<usize> = IdVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.insert(a));
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in lookup {
                    i.remove(keys[*a]);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: IdVec<usize> = IdVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.insert(a));
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
        let mut map: IdVec<usize> = IdVec::new();
        for a in 0..size {
            map.insert(a);
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
        let mut map: IdVec<usize> = IdVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.insert(a));
        }
        for i in lookup {
            map.remove(keys[*i]);
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
