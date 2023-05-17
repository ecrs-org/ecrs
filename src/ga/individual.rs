//! Tratis, structs & methods for representing individual in genetic algorithm

use serde::Serialize;
use std::{fmt::Debug, ops::Deref};

/// Purpose of this trait is to aggregate minimal trait bounds that are requried
/// for the chromosome representation to work with genetic algorithm.
///
/// This is not stable yet, as traits such as Clone / Debug might not be required in the future.
pub trait Chromosome: Sized + Sync + Send + Clone + Default + Debug {}

/// Blanket implementation of Chromosome trait for any type that satisfies the bounds
impl<T: Sized + Sync + Send + Clone + Default + Debug> Chromosome for T {}

/// Representation of an individual for a genetic algorithm.
///
/// This struct has two fileds:
///
/// * `chromosome` - representation of a solution
/// * `fitness` - fitness function value of given individual
///
/// As of latest version this struct is a part of public API,
/// however it will most likely be private in future versions,
/// as there in reason for the end user to interact with it directly.
#[derive(Clone, Debug, Serialize)]
pub struct ConcreteIndividual<T: Chromosome> {
    pub chromosome: T,
    pub fitness: f64,
}

impl<T: Chromosome> ConcreteIndividual<T> {
    /// Returns new instance of individual with default values for its fields.
    pub fn new() -> Self {
        ConcreteIndividual {
            chromosome: T::default(),
            fitness: f64::default(),
        }
    }

    /// Returns reference to chromosome
    #[inline]
    pub fn chromosome_ref(&self) -> &T {
        &self.chromosome
    }

    /// Returns mutable reference to chromosome
    #[inline]
    pub fn chromosome_ref_mut(&mut self) -> &mut T {
        &mut self.chromosome
    }
}

/// This trait is implemented so some usecases are simpler & we
/// can avoid some refactorings. This needs to be checked whether
/// it is a bad practice or not to implement `Deref` for other
/// types than smart pointers.
impl<T: Chromosome> Deref for ConcreteIndividual<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.chromosome
    }
}

impl<T: Chromosome> From<T> for ConcreteIndividual<T> {
    fn from(chromosome: T) -> Self {
        ConcreteIndividual {
            chromosome,
            fitness: f64::MIN,
        }
    }
}

// Traits required for more ergonomic sorting

impl<T: Chromosome> PartialEq<Self> for ConcreteIndividual<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl<T: Chromosome> Eq for ConcreteIndividual<T> {}

impl<T: Chromosome> PartialOrd<Self> for ConcreteIndividual<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

/// Implementation of `Ord` is important for many operators, so that the solutions
/// can be sorted.
impl<T: Chromosome> Ord for ConcreteIndividual<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            return ord;
        }
        unimplemented!();
    }
}

/// Type alias for real valued individual (gene is a f64)
pub type RealValueIndividual = ConcreteIndividual<Vec<f64>>;

/// Type alias for bit string individual (gene is a bool)
pub type BitStringIndividual = ConcreteIndividual<Vec<bool>>;

pub trait IntoIndividual: Sized {
    type ChromosomeType: Chromosome;

    fn into_individual(self) -> ConcreteIndividual<Self::ChromosomeType>;
}
