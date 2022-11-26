mod generic;
mod presets;

use std::error::Error;
use std::fmt::Display;

use super::individual::Chromosome;
use super::operators::selection::SelectionOperator;
use super::population::PopulationGenerator;
use super::{CrossoverOperator, FitnessFn, GAConfig, GAParams, MutationOperator, Probe};

pub use generic::GenericBuilder;
pub use presets::{BitStringBuilder, RealValuedBuilder};

/// Error type for internal use
#[derive(Debug, Clone)]
enum ConfigError {
  MissingParam(String),
  MissingOperator(String),
  MissingPopulationFactory,
  NoProbe,
  NoParams,
}

impl Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::MissingParam(param) => write!(f, "Unspecified parameter: {}", param),
      Self::MissingOperator(op) => write!(f, "Unspecified operator: {}", op),
      Self::MissingPopulationFactory => write!(f, "Unspecified population factory"),
      Self::NoProbe => write!(f, "Unspecified probe"),
      Self::NoParams => write!(f, "No parameters were specified"),
    }
  }
}

impl Error for ConfigError {}

/// This is a mirror struct to `[GAParams]`, except that all fields are wrapped
/// inside `Option` type, so that builders can incrementally fill it up.
// TODO: We should really consider creating a macro here, so that we
// don't have to write it by hand...
pub(self) struct GAParamsOpt {
  pub selection_rate: Option<f64>,
  pub mutation_rate: Option<f64>,
  pub population_size: Option<usize>,
  pub generation_limit: Option<usize>,
  pub max_duration: Option<std::time::Duration>,
}

/// Returns new instance of [GAParamsOpt] struct. All fields are `None` initially.
impl GAParamsOpt {
  pub fn new() -> Self {
    Self {
      selection_rate: None,
      mutation_rate: None,
      population_size: None,
      generation_limit: None,
      max_duration: None,
    }
  }
}

impl TryFrom<GAParamsOpt> for GAParams {
  type Error = ConfigError;

  fn try_from(params_opt: GAParamsOpt) -> Result<Self, Self::Error> {
    let Some(selection_rate) = params_opt.selection_rate else {
			return Err(ConfigError::MissingParam("Unspecified selection rate".to_owned()));
		};

    let Some(mutation_rate) = params_opt.mutation_rate else {
			return Err(ConfigError::MissingParam("Unspecified mutation rate".to_owned()));
		};

    let Some(population_size) = params_opt.population_size else {
			return Err(ConfigError::MissingParam("Unspecified population size".to_owned()));
		};

    let Some(generation_limit) = params_opt.generation_limit else {
			return Err(ConfigError::MissingParam("Unspecified generation_limit".to_owned()));
		};

    let Some(max_duration) = params_opt.max_duration else {
			return Err(ConfigError::MissingParam("Unspecified max duration".to_owned()));
		};

    Ok(GAParams {
      selection_rate,
      mutation_rate,
      population_size,
      generation_limit,
      max_duration,
    })
  }
}

/// This is a mirror struct to `[GAConifg]`, except that all fields are wrapped
/// inside `Option` type, so that builders can incrementally fill it up.
// TODO: We should really consider creating a macro here, so that we
// don't have to write it by hand...
pub(self) struct GAConfigOpt<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  pub params: Option<GAParams>,
  pub fitness_fn: Option<FitnessFn<T>>,
  pub mutation_operator: Option<M>,
  pub crossover_operator: Option<C>,
  pub selection_operator: Option<S>,
  pub population_factory: Option<P>,
  pub probe: Option<Pr>,
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
  /// Returns new instance of [GAConfigOpt] struct. All fields are `None` initially.
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

impl<T, M, C, S, P, Pr> TryFrom<GAConfigOpt<T, M, C, S, P, Pr>> for GAConfig<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  type Error = ConfigError;

  fn try_from(config_opt: GAConfigOpt<T, M, C, S, P, Pr>) -> Result<Self, Self::Error> {
    let Some(params) = config_opt.params else {
			return Err(ConfigError::NoParams);
		};

    let Some(fitness_fn) = config_opt.fitness_fn else {
			return Err(ConfigError::MissingOperator("No fitness function specified".to_owned()));
		};

    let Some(mutation_operator) = config_opt.mutation_operator else {
			return Err(ConfigError::MissingOperator("No mutation operator specified".to_owned()));
		};

    let Some(crossover_operator) = config_opt.crossover_operator else {
			return Err(ConfigError::MissingOperator("No crossover operator specified".to_owned()));
		};

    let Some(selection_operator) = config_opt.selection_operator else {
			return Err(ConfigError::MissingOperator("No selection operator specified".to_owned()));
		};

    let Some(population_factory) = config_opt.population_factory else {
			return Err(ConfigError::MissingPopulationFactory);
		};

    let Some(probe) = config_opt.probe else {
			return Err(ConfigError::NoProbe);
		};

    Ok(GAConfig {
      params,
      fitness_fn,
      mutation_operator,
      crossover_operator,
      selection_operator,
      population_factory,
      probe,
    })
  }
}

pub struct Builder;

impl Builder {
  #[allow(clippy::new_ret_no_self)]
  pub fn new<T, M, C, S, P, Pr>() -> GenericBuilder<T, M, C, S, P, Pr>
  where
    T: Chromosome,
    M: MutationOperator<T>,
    C: CrossoverOperator<T>,
    S: SelectionOperator<T>,
    P: PopulationGenerator<T>,
    Pr: Probe<T>,
  {
    GenericBuilder::<T, M, C, S, P, Pr>::new()
  }

  pub fn with_rvc() -> RealValuedBuilder {
    RealValuedBuilder::new()
  }

  pub fn with_bsc() -> BitStringBuilder {
    BitStringBuilder::new()
  }
}

pub(self) trait BuilderTrait {
  const DEFAULT_PARAMS: GAParams;
}

#[cfg(test)]
mod test {

  #[test]
  fn api_test() {}
}
