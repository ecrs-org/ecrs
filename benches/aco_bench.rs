use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecrs::aco;
use ecrs::aco::ants_behaviour::AntSystemAB;
use ecrs::aco::pheromone::AntSystemPU;
use ecrs::aco::{util, FMatrix};
use std::time::Duration;

pub fn bench_aco_small(c: &mut Criterion) {
  let dist = calc_dist(&CITIES_5);
  let heuristic = util::create_heuristic_from_weights(&dist);

  c.bench_function("aco small", |b| {
    b.iter(|| {
      aco::Builder::new(5)
        .set_weights(black_box(dist.clone()))
        .set_heuristic(black_box(heuristic.clone()))
        .with_standard_ants(black_box(5))
        .set_ants_behaviour(AntSystemAB)
        .set_pheromone_update(AntSystemPU)
        .set_probe(Box::new(EmptyProbe))
        .with_iteration_termination(black_box(20))
        .build()
        .run()
    })
  });
}

pub fn bench_aco_medium(c: &mut Criterion) {
  let dist = calc_dist(&CITIES_15);
  let heuristic = util::create_heuristic_from_weights(&dist);

  c.bench_function("aco medium", |b| {
    b.iter(|| {
      aco::Builder::new(15)
        .set_weights(black_box(dist.clone()))
        .set_heuristic(black_box(heuristic.clone()))
        .set_pheromone_update(AntSystemPU)
        .with_standard_ants(black_box(5))
        .set_ants_behaviour(AntSystemAB)
        .set_probe(Box::new(EmptyProbe))
        .with_iteration_termination(black_box(20))
        .build()
        .run()
    })
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(15)).sample_size(200);
  targets = bench_aco_small, bench_aco_medium
}

criterion_main!(benches);

struct EmptyProbe;
impl aco::probe::Probe for EmptyProbe {}

fn calc_dist(cities: &[(f64, f64)]) -> FMatrix {
  let sol_size = cities.len();
  let mut distance: FMatrix = FMatrix::zeros(sol_size, sol_size);
  for i in 0..sol_size {
    for j in i..sol_size {
      let (x1, y1) = cities[i];
      let (x2, y2) = cities[j];
      let x = x1 - x2;
      let y = y1 - y2;

      let dist = f64::sqrt(x * x + y * y);

      distance[(i, j)] = dist;
      distance[(j, i)] = dist;
    }
  }

  distance
}

const CITIES_5: [(f64, f64); 5] = [
  (35.940008207230115, 74.07029520995907),
  (62.94267436518637, 72.40259837871687),
  (61.183807718335046, 70.20878271639623),
  (24.031479914616625, 24.53073532750132),
  (86.12206741911965, 66.20895811496419),
];

const CITIES_15: [(f64, f64); 15] = [
  (18.065450068857935, 88.71382504540846),
  (39.23620808476569, 81.44965158009457),
  (76.75290449417722, 11.061355377746839),
  (18.514245047506538, 50.4579016607233),
  (67.62017702874621, 60.273441999250025),
  (90.54341436801035, 26.047316687118503),
  (74.55222239828714, 91.10875833150453),
  (24.449575356592447, 69.18499769705073),
  (15.060399162180271, 13.977951744369754),
  (83.69681323875658, 25.683306252468196),
  (59.033928897278656, 52.920644309526075),
  (72.85313559195792, 41.02287752019873),
  (25.70405119619188, 6.984200996335677),
  (74.79935959893629, 89.80620989883252),
  (3.3953339453155706, 11.464492662696578),
];
