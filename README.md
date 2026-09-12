# parser

A CSV parser written in Rust for learning and experimentation.

This project is primarily focused on learning Rust concepts rather than building a production-ready CSV parser. The parser is intentionally simple and somewhat hardcoded because the goal is to understand how Rust handles:

* Ownership and borrowing
* Lifetimes
* Zero-copy parsing
* `&str` slices and references
* String handling
* Error handling
* Iterators
* Performance benchmarking

The parser is being developed incrementally, with benchmark results used to compare different approaches.

## Zero-Copy Parsing

The parser includes a zero-copy version that avoids creating unnecessary `String` allocations when parsing CSV data.

Instead of creating new owned strings for each field, the parser keeps the original CSV contents in memory and stores `&str` slices pointing into that original data.

Zero-copy does not necessarily make the parser faster. Its main purpose is to avoid unnecessary copying and allocations. Performance improvements depend on whether copying and allocation are actually a significant bottleneck.

## Benchmark

CSV file size: ~1.3 MB

### Previous Version

| Benchmark  |      Time |
| ---------- | --------: |
| CSV parser | ~30.34 ms |

Criterion measured the parser at approximately:

* Lower bound: 30.113 ms
* Estimate: 30.341 ms
* Upper bound: 30.601 ms
* Samples: 100
* Outliers: 10 (8 mild, 2 severe)

### Zero-Copy Version

| Benchmark  |      Time |
| ---------- | --------: |
| CSV parser | ~30.10 ms |

Criterion measured the zero-copy parser at:

* Lower bound: 30.003 ms
* Estimate: 30.103 ms
* Upper bound: 30.243 ms
* Samples: 100
* Outliers: 3 (1 mild, 2 severe)

Criterion reported a change of approximately:

`+0.84% +1.29% +1.77%`

The change was within the noise threshold, meaning the zero-copy version did not produce a statistically meaningful runtime improvement in this workload.

The zero-copy implementation was still useful for the project's primary purpose: learning ownership, borrowing, lifetimes, and avoiding unnecessary allocations.

## Project Status

This project is an ongoing Rust learning exercise. Performance optimization and cleanup will be done after the relevant Rust concepts have been explored.
