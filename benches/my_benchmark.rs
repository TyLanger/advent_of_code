use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // Heap
    // A star                  time:   [199.25 ms 202.79 ms 206.47 ms]
    // Old
    // A star                  time:   [290.74 ms 296.10 ms 301.73 ms]
    // change: [+42.302% +46.016% +49.492%] (p = 0.00 < 0.05)
    // Performance has regressed.
    // Turn heap back on
    // A star                  time:   [190.62 ms 193.56 ms 196.70 ms]
    //                     change: [-36.186% -34.631% -33.009%] (p = 0.00 < 0.05)
    //                     Performance has improved.
    // seems heap is a bit better
    c.bench_function("A star", |b| b.iter(|| advent_of_code::bin::day_12::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
