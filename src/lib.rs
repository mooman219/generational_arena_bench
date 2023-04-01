pub mod extra;

pub mod beach_map;
pub mod bvmap;
pub mod compactmap;
pub mod generational_arena;
pub mod generational_indextree;
pub mod id_vec;
pub mod indextree;
pub mod naive;
pub mod pulz_arena;
pub mod slab;
pub mod slotmap;
pub mod stable_vec;
pub mod stash;
pub mod thunderdome;

use criterion::Bencher;

pub trait Crate {
    fn name(&self) -> &'static str;

    /// Setup: None.
    /// Bench: Insert size elements.
    fn insert(&self, b: &mut Bencher, size: usize);

    /// Setup: Insert size elements, remove size elements.
    /// Bench: Insert size elements.
    fn reinsert(&self, b: &mut Bencher, size: usize);

    /// Setup: Insert size elements.
    /// Bench: Remove size elements.
    fn remove(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize);

    /// Setup: Insert size elements.
    /// Bench: Get all element indicies from lookup.
    fn get(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize);

    /// Setup: Insert size elements.
    /// Bench: Iterate over size elements.
    fn iterate(&self, b: &mut Bencher, size: usize);

    /// Setup: Insert size elements. Remove half size elements.
    /// Bench: Iterate over remaining elements.
    fn reiterate(&self, b: &mut Bencher, lookup: &Vec<usize>, size: usize);
}
