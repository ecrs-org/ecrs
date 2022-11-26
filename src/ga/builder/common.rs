use crate::ga::{individual::Chromosome, operators::{mutation::MutationOperator, crossover::CrossoverOperator, selection::SelectionOperator}, population::PopulationGenerator, Probe, GAParams, FitnessFn, GAConfig};


/// This is a mirror struct to `[GAConifg]`, except that all fields are wrapped
/// inside `Option` type, so that builders can incrementally fill it up.
// TODO: We should really consider creating a macro here, so that we
// don't have to write it by hand...
struct GAConfigOpt<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  params: Option<GAParams>,
  fitness_fn: Option<FitnessFn<T>>,
  mutation_operator: Option<M>,
  crossover_operator: Option<C>,
  selection_operator: Option<S>,
  population_factory: Option<P>,
  probe: Option<Pr>,
}

impl<T, M, C, S, P, Pr> GAConfigOpt<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
	/// Returns new instance of [GAConfigOpt] struct. All fields are `None`.
  pub fn new() -> Self {
    Self {
      params: None,
      fitness_fn: None,
      mutation_operator: None,
      crossover_operator: None,
      selection_operator: None,
      population_factory: None,
      probe: None,
    }
  }
}

impl<T, M, C, S, P, Pr> From<GAConfigOpt<T, M, C, S, P, Pr>> for GAConfig<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  fn from(config_opt: GAConfigOpt<T, M, C, S, P, Pr>) -> Self {
    GAConfig {
      params: config_opt.params.unwrap(),
      fitness_fn: config_opt.fitness_fn.unwrap(),
      mutation_operator: config_opt.mutation_operator.unwrap(),
      crossover_operator: config_opt.crossover_operator.unwrap(),
      selection_operator: config_opt.selection_operator.unwrap(),
      population_factory: config_opt.population_factory.unwrap(),
      probe: config_opt.probe.unwrap(),
    }
  }
}
