# Benchmarks: Generational Arenas
This includes benchmarks for crates typically considered to be generational arenas. `idvec` is
omitted from these charts because some of its operations take a log time, blowing up the scale.

## Linear graph - `idvec` removed
![](images/bench_gen_linear.png)

# Benchmarks: All
This includes benchmarks for things that aren't considered generational arenas. A sperate chart is
available with `idvec` removed.

## Log graph
![](images/bench_all_log.png)

## Linear graph - `idvec` removed
![](images/bench_all_linear.png)