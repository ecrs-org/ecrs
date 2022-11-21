use std::ops::Index;

use crate::ga::individual::{Chromosome, Individual};
use push_trait::{Nothing, Push};
use rand::Rng;

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
pub struct SinglePoint;

impl SinglePoint {
  /// Creates new [self::SinglePoint] crossover operator
  pub fn new() -> Self {
    SinglePoint {}
  }
}

impl<GeneT, ChT> CrossoverOperator<ChT> for SinglePoint
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
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
    let cut_point = rand::thread_rng().gen_range(0..chromosome_len);

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
pub struct TwoPoint;

impl TwoPoint {
  /// Creates new [self::TwoPoint] crossover operator
  pub fn new() -> Self {
    TwoPoint {}
  }
}

impl<GeneT, ChT> CrossoverOperator<ChT> for TwoPoint
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
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
      rand::thread_rng().gen_range(0..chromosome_len),
      rand::thread_rng().gen_range(0..chromosome_len),
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
pub struct MultiPoint {
  cut_points_no: usize,
}

impl MultiPoint {
  /// Creates new [self::MultiPoint] crossover operator
  ///
  /// ## Arguments
  ///
  /// * `cut_points_no` - Number of cutpoints (crossover points)
  pub fn new(cut_points_no: usize) -> Self {
    assert!(cut_points_no >= 1, "Number of cut points must be >= 1");
    MultiPoint { cut_points_no }
  }
}

impl Default for MultiPoint {
  /// Creates new [self::MultiPoint] crossover operator with 4 cutpoints
  fn default() -> Self {
    MultiPoint { cut_points_no: 4 }
  }
}

impl<GeneT, ChT> CrossoverOperator<ChT> for MultiPoint
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
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
      rand::seq::index::sample(&mut rand::thread_rng(), chromosome_len, self.cut_points_no).into_vec();
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
pub struct Uniform;

impl Uniform {
  /// Creates new [self::Uniform] crossover operator
  pub fn new() -> Self {
    Uniform {}
  }
}

impl<GeneT, ChT> CrossoverOperator<ChT> for Uniform
where
  ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
  GeneT: Copy,
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

    let mask = rand::thread_rng()
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
