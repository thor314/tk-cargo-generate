//! Benches. To use, import functions of interest, and `cargo bench`.
//!
//! `iai` is an experimental benchmark harness, using Cachegrind to ferform precise single-shot
//! measurements. https://bheisler.github.io/criterion.rs/book/iai/iai.html
#![allow(unused_imports)]
use std::{thread, time};

use iai::black_box as bb;

fn f(millis: u64) {
  let sleep_time = time::Duration::from_millis(millis);
  thread::sleep(sleep_time);
}

fn bench1() { f(bb(1)) }

iai::main!(bench1);

// Iai does not replicate all criterion's features. Using Criterion:
// https://bheisler.github.io/criterion.rs/book/index.html

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// pub fn bench(c: &mut Criterion) {
//     c.bench_function("bench", |b| b.iter(|| f(black_box(1))));
// }

// criterion_group!(benches, bench);
// criterion_main!(benches);
