use itertools::{enumerate, Itertools};
use len_trait::Len;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use crate::ga::individual::{Chromosome, IndividualTrait};
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};
use rand::prelude::SliceRandom;
use rand::{rngs::ThreadRng, Rng};

/// # Crossover Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait CrossoverOperator<IndividualT: IndividualTrait> {
    /// Returns a tuple of children
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT);
}

/// # Single point crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by defininig single cutpoint splitting both parent chromosomes in two parts.
/// First child gets `parent_1`'s first part and `parent_2`'s second part.
/// Second child gets `parent_2`'s first part and `parent_1`'s second part.
///
/// Degenerated case when cutpoint is selected at index 0 or last can occur.
pub struct SinglePoint<R: Rng = ThreadRng> {
    rng: R,
}

impl SinglePoint<ThreadRng> {
    /// Creates new [SinglePoint] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> SinglePoint<R> {
    /// Creates new [SinglePoint] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for SinglePoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works by randomly selecting single cutpoint splitting both parent chromosomes in two parts.
    /// First child gets `parent_1`'s first part and `parent_2`'s second part.
    /// Second child gets `parent_2`'s first part and `parent_1`'s second part.
    ///
    /// Degenerated case when cutpoint is selected at index 0 or last can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        let chromosome_len = parent_1.chromosome().len();
        let cut_point = self.rng.gen_range(0..chromosome_len);

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for locus in 0..cut_point {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
        }

        for locus in cut_point..chromosome_len {
            child_1_ch.push(parent_2.chromosome()[locus]);
            child_2_ch.push(parent_1.chromosome()[locus]);
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Two point crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
/// Then it creates children by taking parents chromosome parts interchangeably.
/// Its mechanism is analoguous to [SinglePoint].
///
/// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
pub struct TwoPoint<R: Rng = ThreadRng> {
    rng: R,
}

impl TwoPoint<ThreadRng> {
    /// Creates new [TwoPoint] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> TwoPoint<R> {
    /// Creates new [TwoPoint] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        TwoPoint { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for TwoPoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
    /// Then it creates children by taking parents chromosome parts interchangeably.
    /// Its mechanism is analoguous to [SinglePoint].
    ///
    /// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let cut_points = (
            self.rng.gen_range(0..chromosome_len),
            self.rng.gen_range(0..chromosome_len),
        );

        let (cut_point_1, cut_point_2) = if cut_points.0 <= cut_points.1 {
            (cut_points.0, cut_points.1)
        } else {
            (cut_points.1, cut_points.0)
        };

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for locus in 0..cut_point_1 {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
        }

        for locus in cut_point_1..cut_point_2 {
            child_1_ch.push(parent_2.chromosome()[locus]);
            child_2_ch.push(parent_1.chromosome()[locus]);
        }

        for locus in cut_point_2..chromosome_len {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Mutli-point crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works analogously to [SinglePoint] or [TwoPoint]. One important difference is that
/// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
/// can be achieved.
pub struct MultiPoint<R: Rng = ThreadRng> {
    cut_points_no: usize,
    rng: R,
}

impl MultiPoint<ThreadRng> {
    /// Creates new [MultiPoint] crossover operator with default RNG
    ///
    /// ## Arguments
    ///
    /// * `cut_points_no` - Number of cutpoints (crossover points)
    pub fn new(cut_points_no: usize) -> Self {
        Self::with_rng(cut_points_no, rand::thread_rng())
    }
}

impl Default for MultiPoint<ThreadRng> {
    /// Creates new [MultiPoint] crossover operator with 4 cutpoints and default RNG
    fn default() -> Self {
        Self::with_rng(4, rand::thread_rng())
    }
}

impl<R: Rng> MultiPoint<R> {
    /// Creates new [MultiPoint] crossover operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `cut_points_no` - Number of cutpoints (crossover points)
    /// * `rng` - Custom random number generator
    pub fn with_rng(cut_points_no: usize, rng: R) -> Self {
        assert!(cut_points_no >= 1, "Number of cut points must be >= 1");
        Self { cut_points_no, rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for MultiPoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works analogously to [SinglePoint] or [TwoPoint]. One important difference is that
    /// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
    /// can be achieved.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );
        assert!(
            self.cut_points_no <= parent_1.chromosome().len(),
            "There can't be more cut points than chromosome length"
        );
        assert!(self.cut_points_no >= 1, "Numver of cut points must be >= 1");

        let chromosome_len = parent_1.chromosome().len();

        let mut cut_points =
            rand::seq::index::sample(&mut self.rng, chromosome_len, self.cut_points_no).into_vec();
        cut_points.sort_unstable();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        let (mut curr_parent_1, mut curr_parent_2) = (&parent_1, &parent_2);

        for locus in 0..cut_points[0] {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
            (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
        }

        for cut_point_idx in 0..self.cut_points_no - 1 {
            for locus in cut_points[cut_point_idx]..cut_points[cut_point_idx + 1] {
                child_1_ch.push(curr_parent_1.chromosome()[locus]);
                child_2_ch.push(curr_parent_2.chromosome()[locus]);
            }
            (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
        }

        for locus in cut_points[self.cut_points_no - 1]..chromosome_len {
            child_1_ch.push(curr_parent_1.chromosome()[locus]);
            child_2_ch.push(curr_parent_2.chromosome()[locus]);
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Uniform crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
/// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
pub struct Uniform<R: Rng + Clone = ThreadRng> {
    rng: R,
}

impl Uniform<ThreadRng> {
    /// Creates new [Uniform] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng + Clone> Uniform<R> {
    /// Creates new [Uniform] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Uniform<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng + Clone,
{
    /// Returns a tuple of children
    ///
    /// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
    /// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        let mask = self
            .rng
            .clone()
            .sample_iter(rand::distributions::Uniform::new(0.0, 1.0))
            .take(chromosome_len);

        for (locus, val) in mask.enumerate() {
            if val >= 0.5 {
                child_1_ch.push(parent_1.chromosome()[locus]);
                child_2_ch.push(parent_2.chromosome()[locus]);
            } else {
                child_1_ch.push(parent_2.chromosome()[locus]);
                child_2_ch.push(parent_1.chromosome()[locus]);
            }
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Parameterized Uniform  crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
/// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
///
/// Bias is a probability of drawing a 1 in the bit-mask.
pub struct UniformParameterized<R: Rng = ThreadRng> {
    rng: R,
    distr: rand::distributions::Uniform<f64>,
    bias: f64,
}

impl UniformParameterized<ThreadRng> {
    pub fn new(bias: f64) -> Self {
        Self::with_rng(rand::thread_rng(), bias)
    }
}

impl<R: Rng> UniformParameterized<R> {
    pub fn with_rng(rng: R, bias: f64) -> Self {
        Self {
            rng,
            distr: rand::distributions::Uniform::new(0.0, 1.0),
            bias,
        }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for UniformParameterized<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng + Clone,
{
    /// Returns a tuple of children
    ///
    /// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
    /// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        let mask = self.rng.clone().sample_iter(self.distr).take(chromosome_len);

        for (locus, val) in mask.enumerate() {
            if val <= self.bias {
                child_1_ch.push(parent_1.chromosome()[locus]);
                child_2_ch.push(parent_2.chromosome()[locus]);
            } else {
                child_1_ch.push(parent_2.chromosome()[locus]);
                child_2_ch.push(parent_1.chromosome()[locus]);
            }
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Ordered crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by taking a substring from one parent, and filling the missing places with alleles from
/// second parent in the order they appear in.
///
/// P1 : 2 <b>4 1 3</b> 5 <br>
/// P2 : 5 2 1 4 3 <br>
/// Ch : 5 <b>4 1 3</b> 2
///
/// Degenerated case when substring has length equal to genome length can occur.
pub struct OrderedCrossover<R: Rng = ThreadRng> {
    rng: R,
}

impl OrderedCrossover<ThreadRng> {
    /// Creates new [OrderedCrossover] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> OrderedCrossover<R> {
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    /// Helper function for [OrderedCrossover::apply]
    /// ## Arguments
    ///
    /// * `p1` - First parent to take part in crossover
    /// * `p2` - Second parent to take part in crossover
    /// * `begin` - Start (inclusive) of substring to transplant
    /// * `end` - End (exclusive) of substring to transplant
    fn create_child<GeneT, IndividualT>(
        &self,
        p1: &IndividualT,
        p2: &IndividualT,
        begin: usize,
        end: usize,
    ) -> IndividualT
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
        let chromosome_len = p1.chromosome().len();

        let mut substring_set: HashSet<GeneT> = HashSet::new();

        for i in begin..end {
            substring_set.push(p1.chromosome()[i]);
        }

        let mut child_ch = IndividualT::ChromosomeT::default();
        let mut index: usize = 0;

        while child_ch.len() < begin {
            let gene = p2.chromosome()[index];
            if !substring_set.contains(&gene) {
                child_ch.push(gene);
            }
            index += 1;
        }

        for i in begin..end {
            child_ch.push(p1.chromosome()[i]);
        }

        while index < chromosome_len {
            let gene = p2.chromosome()[index];
            if !substring_set.contains(&gene) {
                child_ch.push(gene);
            }
            index += 1;
        }
        IndividualT::from(child_ch)
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for OrderedCrossover<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
    R: Rng,
{
    /// Returns a tuple of children, first child is created by taking a substring from parent_1,
    /// second child is created by using a substring from parent_2
    ///
    /// It works by taking a substring from one parent, and filling the missing places with alleles from
    /// second parent in the order they appear in.
    ///
    /// P1 : 2 <b>4 1 3</b> 5 <br>
    /// P2 : 5 2 1 4 3 <br>
    /// Ch : 5 <b>4 1 3</b> 2
    ///
    /// Degenerated case when substring has length equal to genome length can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in crossover
    /// * `parent_2` - Second parent to take part in crossover
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let begin: usize = self.rng.gen_range(0..chromosome_len);
        let end: usize = self.rng.gen_range(begin..=chromosome_len);

        let child_1 = self.create_child(parent_1, parent_2, begin, end);
        let child_2 = self.create_child(parent_2, parent_1, begin, end);

        (child_1, child_2)
    }
}

/// # PPX crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// PPX (Precedence Preservative Crossover), genes are taken in order they appear in parent,
/// parent is chosen at random, if gene was already taken from other parent then the next un-taken gene
/// is chosen
///
/// P1         : <i>2 4 1 3 5</i> <br>
/// P2         : <b>5 2 1 4 3</b> <br>
/// Gene source: 1 1 2 1 2 <br>
/// Ch         : <i> 2 4 </i> <b> 5 </b> <i> 1<i/> <b> 3</b>
///
/// Degenerated case when all genes are taken from the same parent.
pub struct Ppx<R: Rng = ThreadRng> {
    rng: R,
    distribution: rand::distributions::Standard,
}

impl Ppx<ThreadRng> {
    /// Creates new [Ppx] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Ppx<R> {
    /// Creates new [PPXCrossover] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self {
            rng,
            distribution: rand::distributions::Standard,
        }
    }

    /// Helper function for [Ppx::apply]
    /// ## Arguments
    ///
    /// * `p1` - First parent to take part in crossover
    /// * `p2` - Second parent to take part in crossover
    /// * `take_from_p1` - Which genes should be taken from p1
    fn create_child<GeneT, IndividualT>(
        &self,
        p1: &IndividualT,
        p2: &IndividualT,
        take_from_p1: &[bool],
    ) -> IndividualT
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
        let chromosome_len = p1.chromosome().len();

        let mut already_taken: HashSet<GeneT> = HashSet::new();

        let mut child_ch = IndividualT::ChromosomeT::default();
        let mut index_p: [usize; 2] = [0, 0];
        let parents = [p1, p2];

        while child_ch.len() < chromosome_len {
            let index_child = child_ch.len();
            let parent_i = usize::from(!take_from_p1[index_child]);

            while child_ch.len() == index_child {
                let gene = parents[parent_i].chromosome()[index_p[parent_i]];
                index_p[parent_i] += 1;

                if !already_taken.contains(&gene) {
                    already_taken.push(gene);
                    child_ch.push(gene);
                }
            }
        }
        IndividualT::from(child_ch)
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Ppx<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
    R: Rng,
{
    /// Returns a tuple of children, first child is created by using parent_1 as first parent,
    /// second child is created by using a parent_1 as the second parent.
    ///
    /// PPX (Precedence Preservative Crossover), genes are taken in order they appear in parent,
    /// parent is chosen at random, if gene was already taken from other parent then the next un-taken gene
    /// is chosen
    ///
    /// P1         : <i>2 4 1 3 5</i> <br>
    /// P2         : <b>5 2 1 4 3</b> <br>
    /// Gene source: 1 1 2 1 2 <br>
    /// Ch         : <i> 2 4 </i> <b> 5 </b> <i> 1<i/> <b> 3</b>
    ///
    /// Degenerated case when all genes are taken from the same parent can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - one of the parents to take part in crossover
    /// * `parent_2` - one of the parents to take part in crossover
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let take_from_p1: Vec<bool> = (&mut self.rng)
            .sample_iter(self.distribution)
            .take(chromosome_len)
            .collect_vec();

        let child_1 = self.create_child(parent_1, parent_2, &take_from_p1);
        let child_2 = self.create_child(parent_2, parent_1, &take_from_p1);

        (child_1, child_2)
    }
}

/// # PMX crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// Returns a tuple of children, first child is created by taking a substring from parent_1,
/// second child is created by using a substring from parent_2
///
/// It works by taking a substring from one parent, then in second parent we look at genes one by one
/// that would be transplanted if we were transplanting from second parent, if the gene (gene_1) appares in transplanted string from parent one
/// then we ignore it, else:
/// * I. We remember the gene place index (index_1)
/// * II. We look what gene (gene_2) is at this place (index_1) in first parent
/// * III. We look for gene (gene_3) place (index_2) in second parent
/// * IV. If this gene (gene_3) can be found in transplanted genes then we place gene_1 in index_2 place,
/// else we go to step I. with gene_1 = gene_3
///
/// P1 : 8 4 7 <b>3 6 2 5 1</b> 9 0 <br>
/// P2 : 0 1 2 3 4 5 6 7 8 9 <br>
/// Ch : 0 7 4 3 6 2 5 1 8 9
///
/// Degenerated case when substring has length equal to genome length can occur.
///
pub struct Pmx<R: Rng = ThreadRng> {
    rng: R,
}

impl Pmx<ThreadRng> {
    /// Creates new [Pmx] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Pmx<R> {
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    fn to_val_index_map<GeneT, ChT>(&self, chromosome: &ChT) -> HashMap<GeneT, usize>
    where
        ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
        let chromosome_len = chromosome.len();
        let mut val_index_map: HashMap<GeneT, usize> = HashMap::new();

        for i in 0..chromosome_len {
            val_index_map.push((chromosome[i], i));
        }

        val_index_map
    }

    /// Helper function for [Pmx::apply]
    /// ## Arguments
    ///
    /// * `p1` - First parent to take part in crossover
    /// * `p2` - Second parent to take part in crossover
    /// * `begin` - Start (inclusive) of substring to transplant
    /// * `end` - End (exclusive) of substring to transplant
    fn create_child<GeneT, IndividualT>(
        &self,
        p1: &IndividualT,
        p2: &IndividualT,
        begin: usize,
        end: usize,
    ) -> IndividualT
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
        let chromosome_len = p1.chromosome().len();

        let mut substring_set: HashSet<GeneT> = HashSet::new();
        let mut new_chromosome: Vec<Option<GeneT>> = vec![None; chromosome_len];
        let val_to_i_p2 = self.to_val_index_map(p2.chromosome());

        #[allow(clippy::needless_range_loop)]
        for i in begin..end {
            substring_set.push(p1.chromosome()[i]);
            new_chromosome[i] = Some(p1.chromosome()[i])
        }

        for i in begin..end {
            let gene = p2.chromosome()[i];
            if substring_set.contains(&gene) {
                continue;
            }

            let mut j = i;
            loop {
                let val = &p1.chromosome()[j];
                let gene_place_candidate = val_to_i_p2.get(val).unwrap();
                if !(begin..end).contains(gene_place_candidate) {
                    new_chromosome[*gene_place_candidate] = Some(gene);
                    break;
                }
                j = *gene_place_candidate;
            }
        }

        let mut child_ch = IndividualT::ChromosomeT::default();
        for (index, gene_opt) in enumerate(new_chromosome) {
            match gene_opt {
                Some(gene) => child_ch.push(gene),
                None => child_ch.push(p2.chromosome()[index]),
            };
        }
        IndividualT::from(child_ch)
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Pmx<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
    R: Rng,
{
    /// Returns a tuple of children, first child is created by taking a substring from parent_1,
    /// second child is created by using a substring from parent_2
    ///
    /// It works by taking a substring from one parent, then in second parent we look at genes one by one
    /// that would be transplanted if we were transplanting from second parent, if the gene (gene_1) appares in transplanted string from parent one
    /// then we ignore it, else:
    /// * I. We remember the gene place index (index_1)
    /// * II. We look what gene (gene_2) is at this place (index_1) in first parent
    /// * III. We look for gene (gene_3) place (index_2) in second parent
    /// * IV. If this gene (gene_3) can be found in transplanted genes then we place gene_1 in index_2 place,
    /// else we go to step I. with gene_1 = gene_3
    ///
    /// P1 : 8 4 7 <b>3 6 2 5 1</b> 9 0 <br>
    /// P2 : 0 1 2 3 4 5 6 7 8 9 <br>
    /// Ch : 0 7 4 3 6 2 5 1 8 9
    ///
    /// Degenerated case when substring has length equal to genome length can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in crossover
    /// * `parent_2` - Second parent to take part in crossover
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let begin: usize = self.rng.gen_range(0..chromosome_len);
        let end: usize = self.rng.gen_range(begin..=chromosome_len);

        let child_1 = self.create_child(parent_1, parent_2, begin, end);
        let child_2 = self.create_child(parent_2, parent_1, begin, end);

        (child_1, child_2)
    }
}

/// # Shuffle crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by randomly shuffling both parents chromosome the same way then
/// selecting single cutpoint splitting both parents shuffled chromosomes in two parts.
/// First child gets `parent_1`'s first part and `parent_2`'s second part.
/// Second child gets `parent_2`'s first part and `parent_1`'s second part.
/// Lastly childs chromosomes are de-shuffled.
///
/// Degenerated case when cutpoint is selected at index 0 or last can occur.
pub struct Shuffle<R: Rng = ThreadRng> {
    rng: R,
}

impl Shuffle<ThreadRng> {
    /// Creates new [Shuffle] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Shuffle<R> {
    /// Creates new [Shuffle] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Shuffle<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works by randomly shuffling both parents chromosome the same way then
    /// selecting single cutpoint splitting both parents shuffled chromosomes in two parts.
    /// First child gets `parent_1`'s first part and `parent_2`'s second part.
    /// Second child gets `parent_2`'s first part and `parent_1`'s second part.
    /// Lastly childs chromosomes are de-shuffled.
    ///
    /// Degenerated case when cutpoint is selected at index 0 or last can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        let chromosome_len = parent_1.chromosome().len();
        let cut_point = self.rng.gen_range(0..chromosome_len);

        let mut shuffled = Vec::from_iter(0..chromosome_len);
        shuffled.shuffle(&mut self.rng);
        let mask = shuffled.iter().map(|x| x < &cut_point).collect_vec();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for (i, fist_parent) in enumerate(mask) {
            if fist_parent {
                child_1_ch.push(parent_1.chromosome()[i]);
                child_2_ch.push(parent_2.chromosome()[i]);
            } else {
                child_1_ch.push(parent_2.chromosome()[i]);
                child_2_ch.push(parent_1.chromosome()[i]);
            }
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

/// # Fixed point crossover operator
///
/// Works just like `SinglePoint`, however the cut point is fixed and chosen apriori instead of
/// being random.
struct FixedPoint {
    pub cut_point: usize,
}

impl FixedPoint {
    /// Returns new instance of the `FixedPoint` operator.
    ///
    /// # Arguments
    ///
    /// * `cut_point` - index of first gene that will be taken from second parent to first child
    pub fn new(cut_point: usize) -> Self {
        Self { cut_point }
    }
}

impl<GeneT, IndividualT> CrossoverOperator<IndividualT> for FixedPoint
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: IndexMut<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
{
    /// Returns a tuple of children
    ///
    /// It works by cutting parent chromosomes in single, fixed point and the acting like a single
    /// point crossover.
    ///
    /// # Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        let mut child_1 = parent_1.clone();
        let mut child_2 = parent_2.clone();

        for i in self.cut_point..parent_1.chromosome().len() {
            child_1.chromosome_mut()[i] = parent_2.chromosome()[i];
            child_2.chromosome_mut()[i] = parent_1.chromosome()[i];
        }

        (child_1, child_2)
    }
}

#[cfg(test)]
mod test {
    use crate::ga::individual::IndividualTrait;
    use crate::ga::operators::crossover::Ppx;
    use crate::ga::operators::crossover::{CrossoverOperator, FixedPoint, Pmx, Shuffle};
    use crate::ga::{GAMetadata, Individual};
    use std::iter::zip;

    #[test]
    fn check_ppx_example() {
        let op = Ppx::new();
        let p1 = Individual::from(vec![1, 2, 3, 4, 5, 6]);
        let p2 = Individual::from(vec![3, 1, 2, 6, 4, 5]);
        let take_from_p1 = [true, false, true, true, false, false];

        let child = op.create_child(&p1, &p2, &take_from_p1);

        child
            .chromosome()
            .iter()
            .zip([1, 3, 2, 4, 6, 5].iter())
            .for_each(|(x, x_expected)| assert_eq!(x, x_expected))
    }

    #[test]
    fn check_pmx_example() {
        // https://www.rubicite.com/Tutorials/GeneticAlgorithms/CrossoverOperators/PMXCrossoverOperator.aspx/
        let op = Pmx::new();

        let p1 = Individual::from(vec![8, 4, 7, 3, 6, 2, 5, 1, 9, 0]);
        let p2 = Individual::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let child = op.create_child(&p1, &p2, 3, 8);
        for (i, j) in zip(child.chromosome, vec![0, 7, 4, 3, 6, 2, 5, 1, 8, 9]) {
            assert_eq!(i, j);
        }
    }

    #[test]
    fn shuffle_gives_appropriate_len() {
        let mut op = Shuffle::new();

        let p1 = Individual::from(vec![8, 4, 7, 3, 6, 2, 5, 1, 9, 0]);
        let p2 = Individual::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let (child_1, child_2) = op.apply(&GAMetadata::default(), &p1, &p2);
        assert_eq!(child_1.chromosome.len(), 10);
        assert_eq!(child_2.chromosome.len(), 10);
    }

    #[test]
    fn shuffle_fulfills_conditions() {
        let mut op = Shuffle::new();

        let p1 = Individual::from(vec![1, 0, 0, 1, 0, 1, 0, 1, 0, 0]);
        let p2 = Individual::from(vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1]);

        let (c1, c2) = op.apply(&GAMetadata::default(), &p1, &p2);
        for (g1, g2) in c1.chromosome.iter().zip(c2.chromosome.iter()) {
            assert_eq!(g1 * g2, 0);
            assert_eq!(g1 + g2, 1);
        }
    }

    #[test]
    fn fixed_point_works_as_expected() {
        let mut op = FixedPoint::new(4);

        let parent_1_chromosome = vec![8, 4, 7, 3, 6, 2, 5, 1, 9, 0];
        let parent_2_chromosome = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let p1 = Individual::from(parent_1_chromosome.clone());
        let p2 = Individual::from(parent_2_chromosome.clone());

        let (child_1, child_2) = op.apply(&GAMetadata::default(), &p1, &p2);

        let child_1_expected_chromosome = vec![8, 4, 7, 3, 4, 5, 6, 7, 8, 9];
        let child_2_expected_chromosome = vec![0, 1, 2, 3, 6, 2, 5, 1, 9, 0];

        assert_eq!(child_1.chromosome(), &child_1_expected_chromosome);
        assert_eq!(child_2.chromosome(), &child_2_expected_chromosome);
    }
}
