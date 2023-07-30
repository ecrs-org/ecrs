use crate::aco::colony::Colony;
use crate::aco::grader::Grader;
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
pub struct Builder<P, C, G, T, Pr, Ph, Args = (), HasArgs = No>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
    Ph: Pheromone,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    pheromone_update: Option<P>,
    grader: Option<G>,
    colony: Option<C>,
    termination_cond: Option<T>,
    start_pheromone: Option<Ph>,
    probe: Option<Pr>,
    additional_args: Option<Args>,
    _phantom: PhantomData<HasArgs>,
}

impl<P, C, G, T, Pr, Ph, Args, HasArgs> Builder<P, C, G, T, Pr, Ph, Args, HasArgs>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
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
    /// For more info see [aco::grader] module.
    ///
    /// ## Arguments
    /// * `grader` - Implementation of [Grader] trait.
    pub fn set_grader(mut self, grader: G) -> Self {
        self.grader = Some(grader);
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

impl<P, C, G, T, Pr, Ph, Args> Builder<P, C, G, T, Pr, Ph, Args, Yes>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
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
    pub fn build(self) -> AntColonyOptimization<P, C, G, T, Pr, Ph, Args> {
        AntColonyOptimization {
            colony: self.colony.expect("Colony wasn't set"),
            pheromone: self.start_pheromone.expect("Start pheromone wasn't set"),
            pheromone_update: self.pheromone_update.expect("Pheromone update rule wasn't set"),
            grader: self.grader.expect("Grader operator wasn't set"),
            termination_cond: self.termination_cond.expect("Termination condition wasn't set"),
            probe: self.probe.expect("Probe wasn't set"),
            additional_args: self
                .additional_args
                .expect("AdditionalArgs type has been specified, but no struct was provided"),
        }
    }
}

impl<P, C, G, T, Pr, Ph, Args: AdditionalArgs> Builder<P, C, G, T, Pr, Ph, Args, No>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
    Ph: Pheromone,
{
    /// Creates a new instance of Builder.
    pub fn new() -> Self {
        Builder {
            pheromone_update: None,
            grader: None,
            colony: None,
            termination_cond: None,
            start_pheromone: None,
            probe: None,
            additional_args: None,
            _phantom: PhantomData,
        }
    }

    pub fn set_additional_args(self, args: Args) -> Builder<P, C, G, T, Pr, Ph, Args, Yes> {
        Builder {
            pheromone_update: self.pheromone_update,
            grader: self.grader,
            colony: self.colony,
            termination_cond: self.termination_cond,
            start_pheromone: self.start_pheromone,
            probe: self.probe,
            additional_args: Some(args),
            _phantom: Default::default(),
        }
    }
}

impl<P, C, G, T, Pr, Ph> Builder<P, C, G, T, Pr, Ph, (), No>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    G: Grader<()>,
    T: TerminationCondition<Ph, ()>,
    Pr: Probe<Ph, ()>,
    Ph: Pheromone,
{
    /// Builds [AntColonyOptimization] with provided building blocks.
    ///
    /// * `pheromone_update` needs to be specified, if not program will panic
    /// * `start_pheromone` needs to be specified, if not program will panic
    /// * `ants_behaviour` needs to be specified, if not program will panic
    /// * `fitness` needs to be specified, if not program will panic
    /// * `goodness` needs to be specified, if not program will panic
    /// * `ants` need to be specified, if not program will panic
    pub fn build(self) -> AntColonyOptimization<P, C, G, T, Pr, Ph> {
        AntColonyOptimization {
            colony: self.colony.expect("Colony wasn't set"),
            pheromone: self.start_pheromone.expect("Start pheromone wasn't set"),
            pheromone_update: self.pheromone_update.expect("Pheromone update rule wasn't set"),
            grader: self.grader.expect("Grader operator wasn't set"),
            termination_cond: self.termination_cond.expect("Termination condition wasn't set"),
            probe: self.probe.expect("Probe wasn't set"),
            additional_args: (),
        }
    }
}

impl<P, C, G, Pr, Ph, Args, HasArgs> Builder<P, C, G, IterationCond, Pr, Ph, Args, HasArgs>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    Pr: Probe<Ph, Args>,
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

impl<P, C, G, T, Args, HasArgs> Builder<P, C, G, T, StdoutProbe, FMatrix, Args, HasArgs>
where
    P: PheromoneUpdate<FMatrix, Args>,
    C: Colony<FMatrix, Args>,
    G: Grader<Args>,
    T: TerminationCondition<FMatrix, Args>,
    Args: AdditionalArgs,
    HasArgs: HasAdditionalArgs,
{
    /// Sets probe to [StdoutProbe].
    pub fn with_stdout_probe(mut self) -> Self {
        self.probe = Some(StdoutProbe::new());
        self
    }
}
