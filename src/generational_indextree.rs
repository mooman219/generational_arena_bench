use crate::Crate;
use criterion::{black_box, BatchSize, Bencher};
use generational_indextree::Arena;

pub struct CrateGenerationalIndextree();

impl Crate for CrateGenerationalIndextree {
    fn name(&self) -> &'static str {
        "generational-indextree"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let arena = Arena::new();

        b.iter_batched(
            || arena.clone(),
            |mut arena| {
                for i in 0..size {
                    arena.new_node(i);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut arena = Arena::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(arena.new_node(a));
        }
        for a in 0..size {
            keys[a].remove(&mut arena);
        }

        b.iter_batched(
            || arena.clone(),
            |mut arena| {
                for i in 0..size {
                    arena.new_node(i);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut arena = Arena::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(arena.new_node(a));
        }

        b.iter_batched(
            || arena.clone(),
            |mut arena| {
                for i in lookup {
                    keys[*i].remove(&mut arena);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut arena = Arena::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(arena.new_node(a));
        }

        b.iter_batched(
            || arena.clone(),
            |i| {
                for a in 0..size {
                    black_box(i.get(keys[lookup[a]]));
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn iterate(&self, b: &mut Bencher, size: usize) {
        let mut arena = Arena::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(arena.new_node(a));
        }

        b.iter_batched_ref(
            || arena.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reiterate(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut arena = Arena::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(arena.new_node(a));
        }
        for i in lookup {
            keys[*i].remove(&mut arena);
        }

        b.iter_batched_ref(
            || arena.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        );
    }
}
