# parser

A CSV parser written in Rust for learning and experimentation.

## Benchmark

### Baseline

CSV file size: ~1.3 MB

| Benchmark  |      Time |
| ---------- | --------: |
| CSV parser | ~74.73 ms |

Criterion measured the parser at:

* Lower bound: 74.530 ms
* Estimate: 74.729 ms
* Upper bound: 74.930 ms
* Samples: 100
* Warm-up: 3 seconds
* Outliers: Not reported

Criterion also reported that 100 samples could not be completed within the default 5-second target time. The benchmark itself completed successfully.

This is the current baseline before cleanup and optimization.

Future benchmark results will be compared against this baseline.
