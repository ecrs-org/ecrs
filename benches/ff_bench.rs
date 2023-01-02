use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecrs::ff::*;

use std::time::Duration;

use ecrs::ff::probe::empty_probe::EmptyProbe;

//TODO PRESET FIREFLIES

pub fn ff_bench_quarter(c: &mut Criterion) {
  println!("A");
  c.bench_function("rastr_quarter", |b| {
    b.iter(|| {
      FireflyAlgorithm {
        probe: Box::new(EmptyProbe {}),
        config: FireflyAlgorithmCfg {
          max_generations: black_box(250),
          ..Default::default()
        },
        ..Default::default()
      }
      .execute()
    })
  });
}

pub fn ff_bench_half(c: &mut Criterion) {
  println!("A");
  c.bench_function("rastr_half", |b| {
    b.iter(|| {
      FireflyAlgorithm {
        probe: Box::new(EmptyProbe {}),
        config: FireflyAlgorithmCfg {
          max_generations: black_box(500),
          ..Default::default()
        },
        ..Default::default()
      }
      .execute()
    })
  });
}

pub fn ff_bench_full(c: &mut Criterion) {
  c.bench_function("rastr_full", |b| {
    b.iter(|| {
      FireflyAlgorithm {
        probe: Box::new(EmptyProbe {}),
        config: FireflyAlgorithmCfg {
          max_generations: black_box(1000),
          ..Default::default()
        },
        ..Default::default()
      }
      .execute()
    })
  });
}
criterion_group! {
  name = ff_fullbench;
  config = Criterion::default().measurement_time(Duration::from_secs(15)).sample_size(80);
  targets = ff_bench_quarter, ff_bench_half, ff_bench_full
}

criterion_main!(ff_fullbench);
