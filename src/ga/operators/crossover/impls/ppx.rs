use itertools::Itertools;
use len_trait::Len;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Index;

use crate::ga::individual::IndividualTrait;
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};

use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

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
    pub(super) fn create_child<GeneT, IndividualT>(
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
    fn apply_single<GeneT, IndividualT>(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT)
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
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

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Ppx<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
    R: Rng,
{
    /// Returns vector of owned individuals which were created in result of applying crossover
    /// operator.
    ///
    /// First child is created by using parent_1 as first parent,
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
    /// * `metadata` - algorithm state metadata, see the structure details for more info,
    /// * `selected` - references to individuals selected during selection step.
    fn apply(&mut self, metadata: &GAMetadata, selected: &[&IndividualT]) -> Vec<IndividualT> {
        assert!(selected.len() & 1 == 0);

        let mut output = Vec::with_capacity(selected.len());

        for parents in selected.chunks(2) {
            let (child_1, child_2) = self.apply_single(metadata, parents[0], parents[1]);
            output.push(child_1);
            output.push(child_2);
        }

        output
    }
}
