use criterion::{black_box, criterion_group, criterion_main, Criterion};
use byte_shifter::{shift_bytes, unshift_bytes, generate_long_random_u8};

// Benchmark for shift_bytes with 1 megabyte array
pub fn benchmark_shift_bytes(c: &mut Criterion) {
    let source = generate_long_random_u8(1024 * 1024); // 1 MB of random data
    let mask = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    c.bench_function("shift_bytes 1MB", |b| {
        b.iter(|| shift_bytes(black_box(&source), black_box(&mask)))
    });
}

// Benchmark for unshift_bytes with 1 megabyte array
pub fn benchmark_unshift_bytes(c: &mut Criterion) {
    let source = generate_long_random_u8(1024 * 1024); // 1 MB of random data
    let mask = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let shifted = shift_bytes(&source, &mask);

    c.bench_function("unshift_bytes 1MB", |b| {
        b.iter(|| unshift_bytes(black_box(&shifted), black_box(&mask)))
    });
    println!("Shifted bytes length: {}", shifted.len());
}

// Custom configuration for Criterion
fn configure_criterion() -> Criterion {
  Criterion::default().measurement_time(std::time::Duration::from_secs(15))
    .warm_up_time(std::time::Duration::from_secs(3))
}

// Pass the custom configuration to the criterion_group! macro
criterion_group! {
  name = benches;
  config = configure_criterion();
  targets = benchmark_shift_bytes, benchmark_unshift_bytes
}
criterion_main!(benches);