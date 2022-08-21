use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
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
        Box::new(genbench::naive::CrateMooSlotMap()),
        Box::new(genbench::thunderdome::CrateThunderdome()),
        Box::new(genbench::pulz_arena::CratePulzArena()),
    ]
}

fn configure<'a>(c: &'a mut Criterion, name: &'a str) -> BenchmarkGroup<'a, WallTime> {
    let mut g = c.benchmark_group(name);
    g.sample_size(200);
    g
}

fn inserts(c: &mut Criterion) {
    let size = 10_000;
    let mut g = configure(c, "Insert");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.insert(b, size);
        });
    }
}

fn reinserts(c: &mut Criterion) {
    let size = 10_000;
    let mut g = configure(c, "InsertUsed");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.reinsert(b, size);
        });
    }
}

fn remove(c: &mut Criterion) {
    let size = 10_000;
    let mut g = configure(c, "Remove");

    // Lookup is being populated with all 10,000 indicies with a non-linear distribution.
    let mut rng = Rand32::new(17534350047697527989);
    let mut lookup = (0..size).collect::<Vec<usize>>();
    for i in 0..size {
        let t = rng.rand_u32() as usize % size;
        lookup.swap(i, t);
    }

    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.remove(b, &lookup, size);
        });
    }
}

fn get(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = Rand32::new(17534350047697527989);
    let mut lookup = Vec::with_capacity(size);
    for _ in 0..size {
        lookup.push(rng.rand_u32() as usize % size);
    }
    let mut g = configure(c, "Get");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.get(b, &lookup, size);
        });
    }
}

fn iterate(c: &mut Criterion) {
    let size = 10_000;
    let mut g = configure(c, "Iter");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.iterate(b, size);
        });
    }
}

fn reiterate(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = Rand32::new(17534350047697527989);
    let mut lookup = Vec::with_capacity(size);
    for i in 0..size {
        if rng.rand_float() < 0.5 {
            lookup.push(i);
        }
    }
    let mut g = configure(c, "IterHalf");
    for test in self::tests() {
        g.bench_function(test.name(), |b| {
            test.reiterate(b, &lookup, size);
        });
    }
}

criterion_group!(benches, inserts, reinserts, remove, get, iterate, reiterate);
criterion_main!(benches);
