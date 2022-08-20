use crate::Crate;
use beach_map::BeachMap;
use criterion::{black_box, BatchSize, Bencher};

pub struct CrateBeachMap();
impl Crate for CrateBeachMap {
    fn name(&self) -> &'static str {
        "BeachMap"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        b.iter_batched(
            || BeachMap::new(),
            |mut i: BeachMap<usize, usize>| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        b.iter_batched(
            || {
                let mut map: BeachMap<usize, usize> = BeachMap::new();
                let mut keys = Vec::new();
                for a in 0..size {
                    keys.push(map.insert(a));
                }
                for a in 0..size {
                    map.remove(keys[a]);
                }
                map
            },
            |mut i| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, size: usize) {
        b.iter_batched(
            || {
                let mut map: BeachMap<usize, usize> = BeachMap::new();
                let mut keys = Vec::new();
                for a in 0..size {
                    keys.push(map.insert(a));
                }
                (map, keys)
            },
            |(mut i, k)| {
                for a in 0..size {
                    i.remove(k[a]);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        b.iter_batched(
            || {
                let mut map: BeachMap<usize, usize> = BeachMap::new();
                let mut keys = Vec::new();
                for a in 0..size {
                    keys.push(map.insert(a));
                }
                (map, keys)
            },
            |(i, k)| {
                for a in 0..size {
                    black_box(i.get(k[lookup[a]]));
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn iterate(&self, b: &mut Bencher, size: usize) {
        b.iter_batched(
            || {
                let mut map: BeachMap<usize, usize> = BeachMap::new();
                let mut keys = Vec::new();
                for a in 0..size {
                    keys.push(map.insert(a));
                }
                map
            },
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reiterate(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        b.iter_batched(
            || {
                let mut map: BeachMap<usize, usize> = BeachMap::new();
                let mut keys = Vec::new();
                for a in 0..size {
                    keys.push(map.insert(a));
                }
                for i in lookup {
                    map.remove(keys[*i]);
                }
                map
            },
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }
}
