# entity-resolution-benchmarks
Testing out various entity resolution patterns using Rust, Python and who knows what else

I'm still checking my code so please don't look at this.


## Rust

Running with ngrams=10 takes approx 3 seconds.  72188 matches found.
```
❯ hyperfine --runs 3 ./target/release/entity-rust
Benchmark 1: ./target/release/entity-rust
  Time (mean ± σ):      3.128 s ±  0.021 s    [User: 3.078 s, System: 0.043 s]
  Range (min … max):    3.104 s …  3.142 s    3 runs
```

## Python

Running with ngrams=10 takes over 40 seconds. 649839 matches found.
hyperfine --runs 3 'python ./entity-py/entity.py'



