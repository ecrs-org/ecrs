use itertools::Itertools;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::ga::individual::IndividualTrait;

use super::{tools, PopulationGenerator};

/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates vector of random points from R^(dim) space within passed domain constraints.
pub struct RandomPoints<R: Rng = ThreadRng> {
    dim: usize,
    constraints: Vec<Range<f64>>,
    rng: R,
}

impl RandomPoints<ThreadRng> {
    /// Returns [RandomPoints] population generator with given constraints and default RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `constraints` -- Ranges for coordinates
    pub fn with_constraints(dim: usize, constraints: Vec<Range<f64>>) -> Self {
        Self::with_constraints_and_rng(dim, constraints, thread_rng())
    }

    /// Returns [RandomPoints] population generator with given constraints and default RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `constraints` -- Ranges for coordinates
    ///
    /// **NOTE**: However type of `constraints` is `Vec<RangeInclusive<f64>>` this factory method
    /// is *IDENTICAL* to `with_constraints`. It is here just to enable inclusive range usage.
    /// Maybe better solution would be to extract this method to separate trait and create some
    /// kind of function overloading.
    pub fn with_constraints_inclusive(dim: usize, constraints: Vec<RangeInclusive<f64>>) -> Self {
        let noninclusive = constraints
            .into_iter()
            .map(|r| *r.start()..*r.end())
            .collect_vec();
        Self::with_constraints_and_rng(dim, noninclusive, thread_rng())
    }

    /// Returns [RandomPoints] population generator with single constraint for all dimensions and
    /// default RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `constraint` -- Range for coordinates
    pub fn with_single_constraint(dim: usize, constraint: Range<f64>) -> Self {
        Self::with_constraints(dim, std::iter::repeat(constraint).take(dim).collect())
    }

    /// Returns [RandomPoints] population generator with no explicit constraints and default RNG.
    /// Points coords will be from range 0.0..1.0.
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    pub fn new(dim: usize) -> Self {
        Self::with_rng(dim, thread_rng())
    }
}

impl<R: Rng> RandomPoints<R> {
    /// Returns [RandomPoints] population generator with given constraints and custom RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `constraints` -- Ranges for coordinates
    /// * `rng` -- Random numbers generator
    pub fn with_constraints_and_rng(dim: usize, constraints: Vec<Range<f64>>, rng: R) -> Self {
        assert!(dim > 0, "Space dimension must be > 0");
        assert_eq!(
            dim,
            constraints.len(),
            "Number of constraints must match dimension of sampled space"
        );

        RandomPoints {
            dim,
            constraints,
            rng,
        }
    }

    /// Returns [RandomPoints] population generator with no explicit constraints and custom RNG.
    /// Points coords will be from range 0.0..1.0.
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `rng` -- Random numbers generator
    pub fn with_rng(dim: usize, rng: R) -> Self {
        assert!(dim > 0, "Space dimension must be > 0");
        RandomPoints {
            dim,
            constraints: Vec::from_iter(std::iter::repeat(0.0..1.0).take(dim)),
            rng,
        }
    }
}

impl<IndividualT: IndividualTrait<ChromosomeT = Vec<f64>>, R: Rng + Clone> PopulationGenerator<IndividualT>
    for RandomPoints<R>
{
    /// Generates vector of `count` random points from R^(dim) space within passed domain constraints.
    /// If there were no constraints passed then the points coords are from range 0.0..1.0.
    ///
    /// ### Arguments
    ///
    /// * `count` -- Number of points to generate
    fn generate(&mut self, count: usize) -> Vec<IndividualT> {
        tools::PointGenerator::with_rng(self.rng.clone())
            .generate_with_constraints(self.dim, count, &self.constraints)
            .into_iter()
            .map(|chromosome| IndividualT::from(chromosome))
            .collect_vec()
    }
}

/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates vector of random bit-strings.
pub struct BitStrings<R: Rng = ThreadRng> {
    dim: usize,
    rng: R,
}

impl BitStrings<ThreadRng> {
    /// Returns [BitString] population generator wit default RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    pub fn new(dim: usize) -> Self {
        Self::with_rng(dim, thread_rng())
    }
}

impl<R: Rng> BitStrings<R> {
    /// Returns [BitString] population generator wit custom RNG
    ///
    /// ### Arguments
    ///
    /// * `dim` -- Dimension of the sampling space
    /// * `rng` -- Random numbers generator
    pub fn with_rng(dim: usize, rng: R) -> Self {
        assert!(dim > 0, "Space dimension must be > 0");
        BitStrings { dim, rng }
    }
}

impl<IndividualT: IndividualTrait<ChromosomeT = Vec<bool>>, R: Rng> PopulationGenerator<IndividualT>
    for BitStrings<R>
{
    /// Generates vector of `count` random bitstrings
    ///
    /// ### Arguments
    ///
    /// * `count` -- Number of bitstrings to generate
    fn generate(&mut self, count: usize) -> Vec<IndividualT> {
        let mut population: Vec<IndividualT> = Vec::with_capacity(count);

        let distr = rand::distributions::Standard;

        for _ in 0..count {
            population.push(IndividualT::from(
                (&mut self.rng).sample_iter(distr).take(self.dim).collect_vec(),
            ));
        }

        population
    }
}

/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates random permutations of given vector.
/// Permutations can be repeated
pub struct RandomPermutations<GeneT: Copy, R: Rng> {
    genes: Vec<GeneT>,
    rng: R,
}

impl<GeneT: Copy> RandomPermutations<GeneT, ThreadRng> {
    /// Returns [RandomPermutations] population generator with default rng.
    ///
    /// ### Arguments
    ///
    /// * `genes` - Vector which will be permuted
    pub fn new(genes: Vec<GeneT>) -> Self {
        Self::with_rng(genes, thread_rng())
    }
}

impl<GeneT: Copy, R: Rng> RandomPermutations<GeneT, R> {
    /// Returns [RandomPermutations] population generator with custom rng.
    ///
    /// ### Arguments
    ///
    /// * `genes` - Vector which will be permuted
    /// * `rng` - Random numbers generator
    pub fn with_rng(genes: Vec<GeneT>, rng: R) -> Self {
        RandomPermutations { genes, rng }
    }
}

impl<IndividualT: IndividualTrait<ChromosomeT = Vec<GeneT>>, GeneT, R> PopulationGenerator<IndividualT>
    for RandomPermutations<GeneT, R>
where
    GeneT: Copy + Debug + Sync + Send,
    R: Rng,
{
    /// Generates vector of `count` random permutations from stored genes.
    /// Repeated individual are possible.
    ///
    /// ### Arguments
    ///
    /// * `count` -- Number of random permutations to generate
    fn generate(&mut self, count: usize) -> Vec<IndividualT> {
        let mut population: Vec<IndividualT> = Vec::with_capacity(count);

        for _ in 0..count {
            let mut genome = self.genes.clone();
            genome.shuffle(&mut self.rng);
            population.push(IndividualT::from(genome))
        }

        population
    }
}

#[cfg(test)]
mod tests {
    use super::{BitStrings, PopulationGenerator, RandomPoints};
    use crate::ga::{individual::IndividualTrait, population::RandomPermutations, Individual};
    use itertools::Itertools;

    #[test]
    fn points_have_appropriate_len() {
        let dim = 4;
        let mut gen =
            RandomPoints::with_constraints(dim, vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)]);
        let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

        for p in points {
            assert_eq!(p.chromosome.len(), dim)
        }
    }

    #[test]
    fn points_follow_explicit_constraints() {
        let dim = 4;
        let constraints = vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)];
        let mut gen = RandomPoints::with_constraints(dim, constraints.clone());
        let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

        for p in points {
            for (v, res) in std::iter::zip(p.chromosome(), &constraints) {
                assert!(res.contains(v));
            }
        }
    }

    #[test]
    fn points_follow_implicit_constraints() {
        let dim = 30;
        let count = 100;

        let mut gen = RandomPoints::new(dim);
        let points: Vec<Individual<Vec<f64>>> = gen.generate(count);

        for p in points {
            for v in p.chromosome() {
                assert!((0.0..1.0).contains(v));
            }
        }
    }

    #[test]
    fn bistrings_have_appropriate_len() {
        let dim = 30;
        let mut gen = BitStrings::new(dim);
        let points: Vec<crate::ga::Individual<Vec<bool>>> = gen.generate(30);

        for p in points {
            assert_eq!(p.chromosome().len(), dim)
        }
    }

    #[test]
    fn permutations_have_appropriate_len() {
        let dim = 30;
        let mut gen = RandomPermutations::new((0..dim).collect_vec());
        let points: Vec<Individual<Vec<usize>>> = gen.generate(30);

        for p in points {
            assert_eq!(p.chromosome().len(), dim)
        }
    }

    #[test]
    fn permutations_have_every_gene() {
        let dim: usize = 30;
        let mut gen = RandomPermutations::new((1..=dim).collect_vec());
        let points: Vec<Individual<Vec<usize>>> = gen.generate(10);

        for p in points {
            let sum: usize = p.chromosome().iter().sum();
            assert_eq!(sum, ((dim + 1) * dim) / 2)
        }
    }
}
