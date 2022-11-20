use ecrs::ga::{
  operators::selection::{
    Boltzmann, Random, Rank, RankR, RouletteWheel, SelectionOperator, StochasticUniversalSampling, Tournament,
  },
  population::{BitStrings, PopulationGenerator, RandomPoints},
  GAMetadata,
};

#[test]
fn random_selection_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = Random::new().apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn roulette_whell_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = RouletteWheel::new().apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn rank_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = Rank::new().apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn rankr_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = RankR::new(0.5).apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn tournament_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = Tournament::new(0.2).apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn sus_returns_demanded_size() {
  let expected_population_size: usize = 42;
  let population = BitStrings::new(21).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = StochasticUniversalSampling::new().apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}

#[test]
fn boltzmann_returns_demanded_size() {
  let expected_population_size: usize = 42;

  let mut constraints: Vec<std::ops::Range<f64>> = Vec::with_capacity(expected_population_size);
  for _ in 0..expected_population_size {
    constraints.push(-1.0..1.0);
  }

  let population = RandomPoints::new(21, constraints).generate(expected_population_size);

  assert_eq!(
    expected_population_size,
    population.len(),
    "Population generator must return population of expected size"
  );

  // FIXME: We must add mocking!
  let metadata = GAMetadata::default();

  let expected_selection_size = expected_population_size / 2;

  let selected = Boltzmann::new(0.2, 6.0, 300, true).apply(&metadata, &population, expected_selection_size);

  assert_eq!(
    expected_selection_size,
    selected.len(),
    "Selection operator must return population of expected size"
  );
}
