use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecrs::ff::{FireflyAlgorithm, FireflyAlgorithmCfg};

use ecrs::ff::population::Population;
use std::time::Duration;

use ecrs::ff::probe::empty_probe::EmptyProbe;

pub fn ff_bench_preset_small(c: &mut Criterion) {
  let small_flies: Vec<Vec<f64>> = vec![
    vec![-2.474902112144155, -2.1536362875740656],
    vec![3.8653951722920947, -2.2201668468609492],
    vec![1.0164102881219117, 2.4781280669119106],
    vec![-1.8121615279105852, 4.230286526263365],
    vec![-1.998471672021862, 4.761704776832321],
    vec![-1.1717511819365445, 0.7299195422915394],
    vec![4.613365867671519, -4.793012151227323],
    vec![0.5309418654536424, -4.905729640670433],
    vec![-0.4076064672067625, 2.83010735845669],
    vec![-2.2161403148671166, 2.3316005653614074],
  ];
  c.bench_function("rastr_quarter", |b| {
    b.iter(|| {
      FireflyAlgorithm {
        probe: Box::new(EmptyProbe {}),
        config: FireflyAlgorithmCfg {
          max_generations: black_box(500),
          ..Default::default()
        },
        population: Population::from_vector(&small_flies),
        ..Default::default()
      }
      .run()
    })
  });
}

pub fn preset_large(c: &mut Criterion) {
  let large_flies: Vec<Vec<f64>> = vec![
    vec![-1.096708672217388, -2.4925081992981424],
    vec![-2.217337470948857, 3.6870274697138132],
    vec![-0.5852573654914313, 2.07858190306019],
    vec![0.8350311263010237, 4.421696428877633],
    vec![1.2307497294588217, -1.2977275040319713],
    vec![-3.5081747701421584, -0.4808682266114639],
    vec![1.4807090950339656, -4.788982665002752],
    vec![3.347601243782389, 3.0122589325288303],
    vec![3.4764538073705094, -3.825105981058119],
    vec![2.8574385462155742, -4.5813285576664935],
    vec![-1.0634170839022516, -1.5550446535187246],
    vec![4.368547372209592, -1.844392848116263],
    vec![4.591698838864467, -0.24383904791121225],
    vec![1.0546699882061494, -4.931426872497509],
    vec![3.0790434931189523, -4.032090346414607],
    vec![3.2499260908143235, -4.836103255306432],
    vec![1.2360537193135706, -2.345959946755356],
    vec![-4.43928824672243, 3.5531369052833455],
    vec![0.42897629844825147, -1.4553283223745361],
    vec![-4.70002080799667, 4.109159099580671],
  ];

  c.bench_function("rastr_half", |b| {
    b.iter(|| {
      FireflyAlgorithm {
        probe: Box::new(EmptyProbe {}),
        config: FireflyAlgorithmCfg {
          population_size: black_box(20),
          ..Default::default()
        },
        population: Population::from_vector(&large_flies),
        ..Default::default()
      }
      .run()
    })
  });
}

pub fn ff_bench_quarter(c: &mut Criterion) {
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
      .run()
    })
  });
}

pub fn ff_bench_half(c: &mut Criterion) {
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
      .run()
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
      .run()
    })
  });
}

criterion_group! {
  name = ff_fullbench;
  config = Criterion::default().measurement_time(Duration::from_secs(15)).sample_size(80);
  targets = ff_bench_quarter, ff_bench_half, ff_bench_full
}

criterion_main!(ff_fullbench);
