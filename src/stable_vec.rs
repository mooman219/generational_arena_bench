use crate::Crate;
use criterion::{black_box, BatchSize, Bencher};
use stable_vec::{ExternStableVec, InlineStableVec};

pub struct CrateExternStableVec();
impl Crate for CrateExternStableVec {
    fn name(&self) -> &'static str {
        "ExternStableVec"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let map: ExternStableVec<usize> = ExternStableVec::new();
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.push(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut map: ExternStableVec<usize> = ExternStableVec::new();
        for a in 0..size {
            map.push(a);
        }
        for a in 0..size {
            map.remove(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.push(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: ExternStableVec<usize> = ExternStableVec::new();
        for a in 0..size {
            map.push(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in lookup {
                    i.remove(*a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: ExternStableVec<usize> = ExternStableVec::new();
        for a in 0..size {
            map.push(a);
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
        let mut map: ExternStableVec<usize> = ExternStableVec::new();
        for a in 0..size {
            map.push(a);
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
        let mut map: ExternStableVec<usize> = ExternStableVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.push(a));
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

pub struct CrateInlineStableVec();
impl Crate for CrateInlineStableVec {
    fn name(&self) -> &'static str {
        "InlineStableVec"
    }

    fn insert(&self, b: &mut Bencher, size: usize) {
        let map: InlineStableVec<usize> = InlineStableVec::new();
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.push(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn reinsert(&self, b: &mut Bencher, size: usize) {
        let mut map: InlineStableVec<usize> = InlineStableVec::new();
        for a in 0..size {
            map.push(a);
        }
        for a in 0..size {
            map.remove(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in 0..size {
                    i.push(a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn remove(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: InlineStableVec<usize> = InlineStableVec::new();
        for a in 0..size {
            map.push(a);
        }
        b.iter_batched_ref(
            || map.clone(),
            |i| {
                for a in lookup {
                    i.remove(*a);
                }
            },
            BatchSize::SmallInput,
        );
    }

    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize) {
        let mut map: InlineStableVec<usize> = InlineStableVec::new();
        for a in 0..size {
            map.push(a);
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
        let mut map: InlineStableVec<usize> = InlineStableVec::new();
        for a in 0..size {
            map.push(a);
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
        let mut map: InlineStableVec<usize> = InlineStableVec::new();
        let mut keys = Vec::new();
        for a in 0..size {
            keys.push(map.push(a));
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
