use crate::aco::colony::Colony;
use crate::aco::fitness::{CanonicalFitness, Fitness};
use crate::aco::pheromone::{Pheromone, PheromoneUpdate};
use crate::aco::probe::{Probe, StdoutProbe};
use crate::aco::termination_condition::{IterationCond, TerminationCondition};
use crate::aco::{AdditionalArgs, AntColonyOptimization, FMatrix};
use std::marker::PhantomData;

pub trait HasAdditionalArgs {}
pub struct Yes;
impl HasAdditionalArgs for Yes {}
pub struct No;
impl HasAdditionalArgs for No {}

/// Builder for [AntColonyOptimization]
///
pub struct Builder<P, C, F, T, Pr, Ph, Args = (), HasArgs = No>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    solution_size: usize,
    pheromone_update: Option<P>,
    fitness: Option<F>,
    colony: Option<C>,
    termination_cond: Option<T>,
    start_pheromone: Option<Ph>,
    probe: Option<Pr>,
    additional_args: Option<Args>,
    _phantom: PhantomData<HasArgs>,
}

impl<P, C, F, T, Pr, Ph, Args, HasArgs> Builder<P, C, F, T, Pr, Ph, Args, HasArgs>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    /// Sets the used [Probe].
    ///
    ///
    /// ## Arguments
    /// * `probe` - [Probe] trait implementation.
    pub fn set_probe(mut self, probe: Pr) -> Self {
        self.probe = Some(probe);
        self
    }

    /// Sets the the way to calculate pheromone update.
    ///
    /// For more info see [aco::pheromone] module.
    ///
    /// ## Arguments
    /// * `pheromone_update` - Implementation of [PheromoneUpdate] trait.
    pub fn set_pheromone_update(mut self, pheromone_update: P) -> Self {
        self.pheromone_update = Some(pheromone_update);
        self
    }

    /// Sets the the way to calculate fitness.
    ///
    /// For more info see [aco::fitness] module.
    ///
    /// ## Arguments
    /// * `fitness` - Implementation of [Fitness] trait.
    pub fn set_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }

    /// Sets the termination condition.
    ///
    /// For more info see [aco::termination_condition] module.
    ///
    /// ## Arguments
    /// * `termination_condition` - Implementation of [TerminationCondition] trait.
    pub fn set_termination_condition(mut self, termination_condition: T) -> Self {
        self.termination_cond = Some(termination_condition);
        self
    }

    /// Sets the initial pheromone.
    /// ## Arguments
    /// * `start_pheromone` - Initial pheromone.
    pub fn set_start_pheromone(mut self, start_pheromone: Ph) -> Self {
        self.start_pheromone = Some(start_pheromone);
        self
    }

    /// Sets ants colony.
    /// ## Arguments
    /// * `colony` - ants colony.
    pub fn set_colony(mut self, colony: C) -> Self {
        self.colony = Some(colony);
        self
    }
}

impl<P, C, F, T, Pr, Ph, Args> Builder<P, C, F, T, Pr, Ph, Args, Yes>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
    Args: AdditionalArgs,
{
    /// Builds [AntColonyOptimization] with provided building blocks.
    ///
    /// * `pheromone_update` needs to be specified, if not program will panic
    /// * `start_pheromone` needs to be specified, if not program will panic
    /// * `ants_behaviour` needs to be specified, if not program will panic
    /// * `fitness` needs to be specified, if not program will panic
    /// * `goodness` needs to be specified, if not program will panic
    /// * `ants` need to be specified, if not program will panic
    pub fn build(self) -> AntColonyOptimization<P, C, F, T, Pr, Ph, Args> {
        AntColonyOptimization {
            colony: self.colony.expect("Colony wasn't set"),
            pheromone: self.start_pheromone.expect("Start pheromone wasn't set"),
            pheromone_update: self.pheromone_update.expect("Pheromone update rule wasn't set"),
            fitness: self.fitness.expect("Fitness operator wasn't set"),
            termination_cond: self.termination_cond.expect("Termination condition wasn't set"),
            probe: self.probe.expect("Probe wasn't set"),
            additional_args: self
                .additional_args
                .expect("AdditionalArgs type has been specified, but no struct was provided"),
        }
    }
}

impl<P, C, F, T, Pr, Ph> Builder<P, C, F, T, Pr, Ph, (), No>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
{
    /// Creates a new instance of Builder.
    pub fn new(solution_size: usize) -> Self {
        Builder {
            solution_size,
            pheromone_update: None,
            fitness: None,
            colony: None,
            termination_cond: None,
            start_pheromone: None,
            probe: None,
            additional_args: None,
            _phantom: PhantomData,
        }
    }
    /// Builds [AntColonyOptimization] with provided building blocks.
    ///
    /// * `pheromone_update` needs to be specified, if not program will panic
    /// * `start_pheromone` needs to be specified, if not program will panic
    /// * `ants_behaviour` needs to be specified, if not program will panic
    /// * `fitness` needs to be specified, if not program will panic
    /// * `goodness` needs to be specified, if not program will panic
    /// * `ants` need to be specified, if not program will panic
    pub fn build(self) -> AntColonyOptimization<P, C, F, T, Pr, Ph, ()> {
        AntColonyOptimization {
            colony: self.colony.expect("Colony wasn't set"),
            pheromone: self.start_pheromone.expect("Start pheromone wasn't set"),
            pheromone_update: self.pheromone_update.expect("Pheromone update rule wasn't set"),
            fitness: self.fitness.expect("Fitness operator wasn't set"),
            termination_cond: self.termination_cond.expect("Termination condition wasn't set"),
            probe: self.probe.expect("Probe wasn't set"),
            additional_args: (),
        }
    }
}

impl<P, C, T, Pr, Ph, Args, HasArgs> Builder<P, C, CanonicalFitness, T, Pr, Ph, Args, HasArgs>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    /// Sets the weighted graph to be searched.
    ///
    /// ## Arguments
    /// * `weights` - Weighted graph in matrix representation.
    pub fn set_weights(mut self, weights: FMatrix) -> Self {
        assert_eq!(
            weights.nrows(),
            weights.nrows(),
            "Weights should be a square matrix"
        );
        assert_eq!(
            weights.nrows(),
            self.solution_size,
            "Weights should be of length equal to solution size"
        );
        if let Some(mut f) = self.fitness {
            f.weights = weights;
            self.fitness = Some(f)
        } else {
            self.fitness = Some(CanonicalFitness::new(weights))
        }
        self
    }
}

impl<P, C, F, Pr, Ph, Args, HasArgs> Builder<P, C, F, IterationCond, Pr, Ph, Args, HasArgs>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    Pr: Probe<Ph>,
    Ph: Pheromone,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    /// Sets iteration termination condition.
    ///
    /// ## Arguments
    /// * `iterations_limit` - maximal number of iterations.
    pub fn with_iteration_termination(mut self, iterations_limit: usize) -> Self {
        self.termination_cond = Some(IterationCond::new(iterations_limit));
        self
    }
}

impl<P, C, F, T, Args, HasArgs> Builder<P, C, F, T, StdoutProbe, FMatrix, Args, HasArgs>
where
    P: PheromoneUpdate<FMatrix>,
    C: Colony<FMatrix>,
    F: Fitness,
    T: TerminationCondition<FMatrix>,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    /// Sets probe to [StdoutProbe].
    pub fn with_stdout_probe(mut self) -> Self {
        self.probe = Some(StdoutProbe::new());
        self
    }
}
