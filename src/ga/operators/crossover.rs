use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Index;

use crate::ga::individual::{Chromosome, Individual};
use push_trait::{Nothing, Push};
use rand::{rngs::ThreadRng, Rng};

/// # Crossover Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait CrossoverOperator<T: Chromosome> {
  /// Returns a tuple of children
  ///
  /// ## Arguments
  ///
  /// * `parent_1` - First parent to take part in recombination
  /// * `parent_2` - Second parent to take part in recombination
  fn apply(&mut self, parent_1: &Individual<T>, parent_2: &Individual<T>) -> (Individual<T>, Individual<T>);
}

/// # Single point crossover operator
///
/// This struct implements [self::CrossoverOperator] trait and can be used with GA.
///
/// It works by defininig single cutpoint splitting both parent chromosomes in two parts.
/// First child gets `parent_1`'s first part and `parent_2`'s second part.
/// Second child gets `parent_2`'s first part and `parent_1`'s second part.
///
/// Degenerated case when cutpoint is selected at index 0 or last can occur.
pub struct SinglePoint<R: Rng> {
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

impl<GeneT, ChT, R> CrossoverOperator<ChT> for SinglePoint<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
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
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    let chromosome_len = parent_1.chromosome_ref().len();
    let cut_point = self.rng.gen_range(0..chromosome_len);

    let mut child_1: Individual<ChT> = Individual::new();
    let mut child_2: Individual<ChT> = Individual::new();

    for locus in 0..cut_point {
      child_1
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
    }

    for locus in cut_point..chromosome_len {
      child_1
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
    }

    (child_1, child_2)
  }
}

/// # Two point crossover operator
///
/// This struct implements [self::CrossoverOperator] and can be used with GA.
///
/// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
/// Then it creates children by taking parents chromosome parts interchangeably.
/// Its mechanism is analoguous to [self::SinglePoint].
///
/// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
pub struct TwoPoint<R: Rng> {
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

impl<GeneT, ChT, R> CrossoverOperator<ChT> for TwoPoint<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
  R: Rng,
{
  /// Returns a tuple of children
  ///
  /// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
  /// Then it creates children by taking parents chromosome parts interchangeably.
  /// Its mechanism is analoguous to [self::SinglePoint].
  ///
  /// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
  ///
  /// ## Arguments
  ///
  /// * `parent_1` - First parent to take part in recombination
  /// * `parent_2` - Second parent to take part in recombination
  fn apply(
    &mut self,
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    assert_eq!(
      parent_1.chromosome_ref().len(),
      parent_2.chromosome_ref().len(),
      "Parent chromosome length must match"
    );

    let chromosome_len = parent_1.chromosome_ref().len();

    let cut_points = (
      self.rng.gen_range(0..chromosome_len),
      self.rng.gen_range(0..chromosome_len),
    );

    let (cut_point_1, cut_point_2) = if cut_points.0 <= cut_points.1 {
      (cut_points.0, cut_points.1)
    } else {
      (cut_points.1, cut_points.0)
    };

    let mut child_1: Individual<ChT> = Individual::new();
    let mut child_2: Individual<ChT> = Individual::new();

    for locus in 0..cut_point_1 {
      child_1
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
    }

    for locus in cut_point_1..cut_point_2 {
      child_1
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
    }

    for locus in cut_point_2..chromosome_len {
      child_1
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
    }

    (child_1, child_2)
  }
}

/// # Mutli-point crossover operator
///
/// This struct implements [self::CrossoverOperator] and can be used with GA.
///
/// It works analogously to [self::SinglePoint] or [self::TwoPoint]. One important difference is that
/// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
/// can be achieved.
pub struct MultiPoint<R: Rng> {
  cut_points_no: usize,
  rng: R,
}

impl MultiPoint<ThreadRng> {
  /// Creates new [self::MultiPoint] crossover operator with default RNG
  ///
  /// ## Arguments
  ///
  /// * `cut_points_no` - Number of cutpoints (crossover points)
  pub fn new(cut_points_no: usize) -> Self {
    Self::with_rng(cut_points_no, rand::thread_rng())
  }
}

impl Default for MultiPoint<ThreadRng> {
  /// Creates new [self::MultiPoint] crossover operator with 4 cutpoints and default RNG
  fn default() -> Self {
    Self::with_rng(4, rand::thread_rng())
  }
}

impl<R: Rng> MultiPoint<R> {
  /// Creates new [self::MultiPoint] crossover operator with custom RNG
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

impl<GeneT, ChT, R> CrossoverOperator<ChT> for MultiPoint<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
  R: Rng,
{
  /// Returns a tuple of children
  ///
  /// It works analogously to [self::SinglePoint] or [self::TwoPoint]. One important difference is that
  /// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
  /// can be achieved.
  ///
  /// ## Arguments
  ///
  /// * `parent_1` - First parent to take part in recombination
  /// * `parent_2` - Second parent to take part in recombination
  fn apply(
    &mut self,
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    assert_eq!(
      parent_1.chromosome_ref().len(),
      parent_2.chromosome_ref().len(),
      "Parent chromosome length must match"
    );
    assert!(
      self.cut_points_no <= parent_1.chromosome_ref().len(),
      "There can't be more cut points than chromosome length"
    );
    assert!(self.cut_points_no >= 1, "Numver of cut points must be >= 1");

    let chromosome_len = parent_1.chromosome_ref().len();

    let mut cut_points =
      rand::seq::index::sample(&mut self.rng, chromosome_len, self.cut_points_no).into_vec();
    cut_points.sort_unstable();

    let mut child_1: Individual<ChT> = Individual::new();
    let mut child_2: Individual<ChT> = Individual::new();

    let (mut curr_parent_1, mut curr_parent_2) = (&parent_1, &parent_2);

    for locus in 0..cut_points[0] {
      child_1
        .chromosome_ref_mut()
        .push(parent_1.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(parent_2.chromosome_ref()[locus]);
      (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
    }

    for cut_point_idx in 0..self.cut_points_no - 1 {
      for locus in cut_points[cut_point_idx]..cut_points[cut_point_idx + 1] {
        child_1
          .chromosome_ref_mut()
          .push(curr_parent_1.chromosome_ref()[locus]);
        child_2
          .chromosome_ref_mut()
          .push(curr_parent_2.chromosome_ref()[locus]);
      }
      (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
    }

    for locus in cut_points[self.cut_points_no - 1]..chromosome_len {
      child_1
        .chromosome_ref_mut()
        .push(curr_parent_1.chromosome_ref()[locus]);
      child_2
        .chromosome_ref_mut()
        .push(curr_parent_2.chromosome_ref()[locus]);
    }

    (child_1, child_2)
  }
}

/// # Uniform crossover operator
///
/// This struct implements [self::CrossoverOperator] and can be used with GA.
///
/// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
/// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
pub struct Uniform<R: Rng + Clone> {
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

impl<GeneT, ChT, R> CrossoverOperator<ChT> for Uniform<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
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
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    assert_eq!(
      parent_1.chromosome_ref().len(),
      parent_2.chromosome_ref().len(),
      "Parent chromosome length must match"
    );

    let chromosome_len = parent_1.chromosome_ref().len();

    let mut child_1: Individual<ChT> = Individual::new();
    let mut child_2: Individual<ChT> = Individual::new();

    let mask = self
      .rng
      .clone()
      .sample_iter(rand::distributions::Uniform::new(0.0, 1.0))
      .take(chromosome_len);

    for (locus, val) in mask.enumerate() {
      if val >= 0.5 {
        child_1
          .chromosome_ref_mut()
          .push(parent_1.chromosome_ref()[locus]);
        child_2
          .chromosome_ref_mut()
          .push(parent_2.chromosome_ref()[locus]);
      } else {
        child_1
          .chromosome_ref_mut()
          .push(parent_2.chromosome_ref()[locus]);
        child_2
          .chromosome_ref_mut()
          .push(parent_1.chromosome_ref()[locus]);
      }
    }

    (child_1, child_2)
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
pub struct OrderedCrossover<R: Rng> {
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
  fn create_child<GeneT, ChT>(
    &self,
    p1: &Individual<ChT>,
    p2: &Individual<ChT>,
    begin: usize,
    end: usize,
  ) -> Individual<ChT>
  where
    ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
  {
    let chromosome_len = p1.chromosome_ref().len();

    let mut substring_set: HashSet<GeneT> = HashSet::new();

    for i in begin..end {
      substring_set.push(p1.chromosome_ref()[i]);
    }

    let mut child: Individual<ChT> = Individual::new();
    let mut index: usize = 0;

    while child.chromosome_ref().len() < begin {
      let gene = p2.chromosome_ref()[index];
      if !substring_set.contains(&gene) {
        child.chromosome_ref_mut().push(gene);
      }
      index += 1;
    }

    for i in begin..end {
      child.chromosome_ref_mut().push(p1.chromosome_ref()[i]);
    }

    while index < chromosome_len {
      let gene = p2.chromosome_ref()[index];
      if !substring_set.contains(&gene) {
        child.chromosome_ref_mut().push(gene);
      }
      index += 1;
    }
    child
  }
}

impl<GeneT, ChT, R> CrossoverOperator<ChT> for OrderedCrossover<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
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
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    assert_eq!(
      parent_1.chromosome_ref().len(),
      parent_2.chromosome_ref().len(),
      "Parent chromosome length must match"
    );

    let chromosome_len = parent_1.chromosome_ref().len();

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
pub struct PPXCrossover<R: Rng> {
  rng: R,
}

impl PPXCrossover<ThreadRng> {
  /// Creates new [PPXCrossover] crossover operator with default RNG
  pub fn new() -> Self {
    Self::with_rng(rand::thread_rng())
  }
}

impl<R: Rng> PPXCrossover<R> {
  /// Creates new [PPXCrossover] crossover operator with custom RNG
  pub fn with_rng(rng: R) -> Self {
    Self { rng }
  }

  /// Helper function for [PPXCrossover::apply]
  /// ## Arguments
  ///
  /// * `p1` - First parent to take part in crossover
  /// * `p2` - Second parent to take part in crossover
  /// * `take_from_p1` - Which genes should be taken from p1
  fn create_child<GeneT, ChT>(
    &self,
    p1: &Individual<ChT>,
    p2: &Individual<ChT>,
    take_from_p1: &[bool],
  ) -> Individual<ChT>
  where
    ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
  {
    let chromosome_len = p1.chromosome_ref().len();

    let mut already_taken: HashSet<GeneT> = HashSet::new();

    let mut child: Individual<ChT> = Individual::new();
    let mut index_p: [usize; 2] = [0, 0];
    let parents = [p1, p2];

    while child.chromosome_ref().len() < chromosome_len {
      let index_child = child.chromosome_ref().len();
      let parent_i: usize = if take_from_p1[index_child] { 0 } else { 1 };

      while child.chromosome_ref().len() == index_child {
        let gene = parents[parent_i].chromosome_ref()[index_p[parent_i]];
        index_p[parent_i] += 1;

        if !already_taken.contains(&gene) {
          already_taken.push(gene);
          child.chromosome_ref_mut().push(gene);
        }
      }
    }

    child
  }
}

impl<GeneT, ChT, R> CrossoverOperator<ChT> for PPXCrossover<R>
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
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
    parent_1: &Individual<ChT>,
    parent_2: &Individual<ChT>,
  ) -> (Individual<ChT>, Individual<ChT>) {
    assert_eq!(
      parent_1.chromosome_ref().len(),
      parent_2.chromosome_ref().len(),
      "Parent chromosome length must match"
    );

    let chromosome_len = parent_1.chromosome_ref().len();

    let take_from_p1: Vec<bool> = (&mut self.rng)
      .sample_iter(&rand::distributions::Standard)
      .take(chromosome_len)
      .collect_vec();

    let child_1 = self.create_child(parent_1, parent_2, &take_from_p1);
    let child_2 = self.create_child(parent_2, parent_1, &take_from_p1);

    (child_1, child_2)
  }
}
