use criterion::{criterion_group, criterion_main, Criterion};
use genbench::Crate;
use oorandom::Rand32;

pub fn tests() -> [Box<dyn Crate>; 16] {
    [
        Box::new(genbench::bvmap::CrateBvMap()),
        Box::new(genbench::stash::CrateStash()),
        Box::new(genbench::stash::CrateUniqueStash()),
        Box::new(genbench::slotmap::CrateSlotMap()),
        Box::new(genbench::slotmap::CrateHopSlotMap()),
        Box::new(genbench::slotmap::CrateDenseSlotMap()),
        Box::new(genbench::slab::CrateSlab()),
        Box::new(genbench::beach_map::CrateBeachMap()),
        Box::new(genbench::stable_vec::CrateExternStableVec()),
        Box::new(genbench::stable_vec::CrateInlineStableVec()),
        Box::new(genbench::id_vec::CrateIdVec()),
        Box::new(genbench::compactmap::CrateCompactMap()),
        Box::new(genbench::generational_arena::CrateGenerationalArena()),
        Box::new(genbench::moomap::CrateMooSlotMap()),
        Box::new(genbench::thunderdome::CrateThunderdome()),
        Box::new(genbench::pulz_arena::CratePulzArena()),
    ]
}

fn inserts(c: &mut Criterion) {
    let size = 10_000;
    let mut g = c.benchmark_group("Insert");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.insert(b, size);
        });
    }
}

fn reinserts(c: &mut Criterion) {
    let size = 10_000;
    let mut g = c.benchmark_group("ReInsert");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.reinsert(b, size);
        });
    }
}

fn remove(c: &mut Criterion) {
    let size = 10_000;
    let mut g = c.benchmark_group("Remove");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.remove(b, size);
        });
    }
}

fn get(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = Rand32::new(17534350047697527989);
    let mut lookup = Vec::new();
    for _ in 0..size {
        lookup.push(rng.rand_u32() as usize % size);
    }
    let mut g = c.benchmark_group("Get");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.get(b, &lookup, size);
        });
    }
}

fn iterate(c: &mut Criterion) {
    let size = 10_000;
    let mut g = c.benchmark_group("Iter");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.iterate(b, size);
        });
    }
}

fn reiterate(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = Rand32::new(17534350047697527989);
    let mut lookup = Vec::new();
    for i in 0..size {
        if rng.rand_float() < 0.5 {
            lookup.push(i);
        }
    }
    let mut g = c.benchmark_group("IterHalf");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.reiterate(b, &lookup, size);
        });
    }
}

criterion_group!(benches, inserts, reinserts, remove, get, iterate, reiterate);
criterion_main!(benches);
