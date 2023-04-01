# Crates
| Implementation    | Link                                                  |
|-------------------|-------------------------------------------------------|
| BvMap             | https://github.com/spersson/bvmap/                    |
| Stash             | https://github.com/Stebalien/stash-rs                 |
| UniqueStash       | https://github.com/Stebalien/stash-rs                 |
| SlotMap           | https://github.com/orlp/slotmap                       |
| HopSlotMap        | https://github.com/orlp/slotmap                       |
| DenseSlotMap      | https://github.com/orlp/slotmap                       |
| Slab              | https://github.com/tokio-rs/slab                      |
| BeachMap          | https://github.com/leudz/beach_map                    |
| ExternStableVec   | https://github.com/LukasKalbertodt/stable-vec         |
| InlineStableVec   | https://github.com/LukasKalbertodt/stable-vec         |
| IdVec             | https://github.com/nical/vodk.rs                      |
| CompactMap        | https://github.com/vi/compactmap                      |
| GenerationalArena | https://github.com/fitzgen/generational-arena         |
| NaiveSlotMap      | https://github.com/mooman219/generational_arena_bench |
| Thunderdome       | https://github.com/LPGhatguy/thunderdome              |
| pulz-arena        | https://github.com/HellButcher/pulz                   |
| indextree         | https://github.com/saschagrunert/indextree            |
| generational-indextree | https://gitlab.com/barry.van.acker/generational-indextree |

# Procedure
  
| Test       | Setup                                                                                        | Benchmark                            |
|------------|----------------------------------------------------------------------------------------------|--------------------------------------|
| Get        | - Create an empty arena.<br>- Insert 10,000 entities.                                        | Get 10,000 entities randomly.        |
| Insert     | - Create an empty arena.                                                                     | Insert 10,000 entities.              |
| InsertUsed | - Create an empty arena.<br>- Insert 10,000 entities.<br>- Remove all entities sequentially. | Insert 10,000 entities.              |
| Iter       | - Create an empty arena.<br>- Insert 10,000 entities.                                        | Iterate over the arena sequentially. |
| IterHalf   | - Create an empty arena.<br>- Insert 10,000 entities.<br>- Remove 5,000 entities randomly.   | Iterate over the arena sequentially. |
| Remove     | - Create an empty arena.<br>- Insert 10,000 entities.                                        | Remove all entities randomly.        |

The lower bound of the 95% confidence interval is used to reduce run to run noise. This likely under-penalizes indirection overhead, so take measurements with a grain of salt.  

# Benchmarks: Generational Arenas
This includes benchmarks for crates typically considered to be generational arenas. `idvec` is
omitted from these charts because some of its operations take a long time, blowing up the scale.

This was run on an [AMD 3950x](https://www.amd.com/en/products/cpu/amd-ryzen-9-3950x).

## Linear graph - `idvec` removed
![](images/bench_gen_linear.png)

# Benchmarks: All
This includes benchmarks for things that aren't considered generational arenas. A sperate chart is
available with `idvec` removed.

## Log graph
![](images/bench_all_log.png)

## Linear graph - `idvec` removed
![](images/bench_all_linear.png)
