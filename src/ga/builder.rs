//! Builder interfaces & impls for genetic algorithm
//!
//! For usage see docs for particular builders.

mod bitstring;
mod generic;
mod realvalued;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use super::individual::IndividualTrait;
use super::operators::replacement::ReplacementOperator;
use super::operators::selection::SelectionOperator;
use super::population::PopulationGenerator;
use super::{CrossoverOperator, GAConfig, GAParams, MutationOperator, Probe};

use crate::ga::operators::fitness::Fitness;
pub use bitstring::BitStringBuilder;
pub use generic::GenericBuilder;
pub use realvalued::RealValuedBuilder;

/// The trait for fitness was already added. Moreover it rather
/// should not be defined in builder module.
type FitnessFn<S> = fn(&S) -> f64;

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
            Self::MissingParam(param) => write!(f, "Unspecified parameter: {param}"),
            Self::MissingOperator(op) => write!(f, "Unspecified operator: {op}"),
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
#[derive(Debug, Clone)]
pub(self) struct GAParamsOpt {
    pub selection_rate: Option<f64>,
    pub mutation_rate: Option<f64>,
    pub population_size: Option<usize>,
    pub generation_limit: Option<usize>,
    pub max_duration: Option<std::time::Duration>,
}

impl GAParamsOpt {
    /// Returns new instance of [GAParamsOpt] struct. All fields are `None` initially.
    pub fn new() -> Self {
        Self {
            selection_rate: None,
            mutation_rate: None,
            population_size: None,
            generation_limit: None,
            max_duration: None,
        }
    }

    /// Sets all `None` values to values form `other`
    pub fn fill_from(&mut self, other: &GAParams) {
        self.selection_rate.get_or_insert(other.selection_rate);
        self.mutation_rate.get_or_insert(other.mutation_rate);
        self.population_size.get_or_insert(other.population_size);
        self.generation_limit.get_or_insert(other.generation_limit);
        self.max_duration.get_or_insert(other.max_duration);
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
pub(self) struct GAConfigOpt<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT>,
    CrossOpT: CrossoverOperator<IndividualT>,
    SelOpT: SelectionOperator<IndividualT>,
    ReplOpT: ReplacementOperator<IndividualT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
    pub params: GAParamsOpt,
    pub fitness_fn: Option<FitnessT>,
    pub mutation_operator: Option<MutOpT>,
    pub crossover_operator: Option<CrossOpT>,
    pub selection_operator: Option<SelOpT>,
    pub replacement_operator: Option<ReplOpT>,
    pub population_factory: Option<PopGenT>,
    pub probe: Option<ProbeT>,
    _phantom: PhantomData<IndividualT>,
}

impl<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
    GAConfigOpt<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT>,
    CrossOpT: CrossoverOperator<IndividualT>,
    SelOpT: SelectionOperator<IndividualT>,
    ReplOpT: ReplacementOperator<IndividualT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
    /// Returns new instance of [GAConfigOpt] struct. All fields are `None` initially, except params.
    pub fn new() -> Self {
        Self {
            params: GAParamsOpt::new(),
            fitness_fn: None,
            mutation_operator: None,
            crossover_operator: None,
            selection_operator: None,
            replacement_operator: None,
            population_factory: None,
            probe: None,
            _phantom: Default::default(),
        }
    }
}

impl<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
    TryFrom<GAConfigOpt<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>>
    for GAConfig<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT>,
    CrossOpT: CrossoverOperator<IndividualT>,
    SelOpT: SelectionOperator<IndividualT>,
    ReplOpT: ReplacementOperator<IndividualT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
    type Error = ConfigError;

    fn try_from(
        config_opt: GAConfigOpt<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>,
    ) -> Result<Self, Self::Error> {
        let params = GAParams::try_from(config_opt.params)?;

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

        let Some(replacement_operator) = config_opt.replacement_operator else {
			return Err(ConfigError::MissingOperator("No replacement operator specified".to_owned()));
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
            replacement_operator,
            population_factory,
            probe,
            _phantom: PhantomData::default(),
        })
    }
}

/// # Builder
///
/// Use this struct to construct concrete builder of genetic algorithm.
///
/// See public methods descriptions for explaination.
pub struct Builder;

impl Builder {
    /// Returns new instance of [GenericBuilder](self::generic::GenericBuilder)
    ///
    /// Use this function if you want to configure operators && parameters for your optimizer
    #[allow(clippy::new_ret_no_self)]
    pub fn new<IndividualT, MulOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>(
    ) -> GenericBuilder<IndividualT, MulOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
    where
        IndividualT: IndividualTrait,
        MulOpT: MutationOperator<IndividualT>,
        CrossOpT: CrossoverOperator<IndividualT>,
        SelOpT: SelectionOperator<IndividualT>,
        ReplOpT: ReplacementOperator<IndividualT>,
        PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
        FitnessT: Fitness<IndividualT::ChromosomeT>,
        ProbeT: Probe<IndividualT::ChromosomeT>,
    {
        GenericBuilder::<IndividualT, MulOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>::new()
    }

    /// Returns new instance of [RealValuedBuilder](self::realvalued::RealValuedBuilder)
    ///
    /// Use this function if your problem uses real valued chromosome and you don't want
    /// to specify all the operators manually but rely on provided defaults.
    ///
    /// Please note that sticking to defaults rarely results in great performance.
    ///
    pub fn with_rvc<F: Fitness<realvalued::Rvc>>() -> RealValuedBuilder<F> {
        RealValuedBuilder::new()
    }

    /// Returns new instance of [BitStringBuilder](self::bitstring::BitStringBuilder)
    ///
    /// Use this function if your problem uses bit string chromosome and you don't want
    /// to specify all the operators manually but rely on provided defaults.
    ///
    /// Please note that sticking to defaults rarely results in great performance.
    ///
    pub fn with_bsc<F: Fitness<bitstring::Bsc>>() -> BitStringBuilder<F> {
        BitStringBuilder::new()
    }
}

/// Trait with default parameters definitions. Can be used as a bound to override
/// some of the values.
pub(crate) trait DefaultParams {
    const DEFAULT_PARAMS: GAParams = GAParams {
        selection_rate: 1.0,
        mutation_rate: 0.05,
        population_size: 100,
        generation_limit: usize::MAX,
        max_duration: std::time::Duration::MAX,
    };

    // fn set_selection_rate(self, selection_rate: f64) -> Self;
    // fn set_mutation_rate(self, mutation_rate: f64) -> Self;
    // fn set_max_duration(self, max_duration: std::time::Duration) -> Self;
    // fn set_max_generation_count(self, max_gen_count: usize) -> Self;
    // fn set_population_size(self, size: usize) -> Self;
    // fn build(self) -> GeneticAlgorithm<T, M, C, S, P, Pr>;
}

#[cfg(test)]
mod test {
    use super::GAParamsOpt;
    use crate::ga::{builder::ConfigError, GAParams};

    fn convert_gaparamsopt_to_ga_params(params_opt: GAParamsOpt) -> Result<GAParams, ConfigError> {
        params_opt.try_into()
    }

    #[test]
    fn new_param_opt_is_empty() {
        let params = GAParamsOpt::new();
        assert!(params.selection_rate.is_none());
        assert!(params.mutation_rate.is_none());
        assert!(params.population_size.is_none());
        assert!(params.generation_limit.is_none());
        assert!(params.max_duration.is_none());
    }

    #[test]
    fn param_opt_fills_correctly() {
        let mut params_opt = GAParamsOpt::new();
        params_opt.selection_rate = Some(0.5);
        params_opt.generation_limit = Some(100);

        let params = GAParams {
            selection_rate: 1.0,
            mutation_rate: 1.0,
            population_size: 100,
            generation_limit: 200,
            max_duration: std::time::Duration::from_secs(1),
        };

        params_opt.fill_from(&params);

        assert!(params_opt.selection_rate.is_some() && params_opt.selection_rate.unwrap() == 0.5);
        assert!(params_opt.mutation_rate.is_some() && params_opt.mutation_rate.unwrap() == 1.0);
        assert!(params_opt.population_size.is_some() && params_opt.population_size.unwrap() == 100);
        assert!(params_opt.generation_limit.is_some() && params_opt.generation_limit.unwrap() == 100);
        assert!(
            params_opt.max_duration.is_some()
                && params_opt.max_duration.unwrap() == std::time::Duration::from_secs(1)
        );
    }

    #[test]
    fn conversion_works_as_expected() {
        let mut params_opt = GAParamsOpt::new();

        params_opt.selection_rate = Some(1.0);
        assert!(convert_gaparamsopt_to_ga_params(params_opt.clone()).is_err());

        params_opt.mutation_rate = Some(0.0);
        assert!(convert_gaparamsopt_to_ga_params(params_opt.clone()).is_err());

        params_opt.population_size = Some(200);
        assert!(convert_gaparamsopt_to_ga_params(params_opt.clone()).is_err());

        params_opt.generation_limit = Some(200);
        assert!(convert_gaparamsopt_to_ga_params(params_opt.clone()).is_err());

        params_opt.max_duration = Some(std::time::Duration::from_micros(10));
        assert!(convert_gaparamsopt_to_ga_params(params_opt).is_ok());
    }
}
