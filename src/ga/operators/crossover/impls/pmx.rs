use itertools::enumerate;
use len_trait::Len;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

use crate::ga::individual::{Chromosome, IndividualTrait};
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};

use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

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
    pub(super) fn create_child<GeneT, IndividualT>(
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
    fn apply_legacy(
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

    fn apply(&mut self, metadata: &GAMetadata, selected: &[&IndividualT], output: &mut Vec<IndividualT>) {
        assert!(selected.len() & 1 == 0);
        for parents in selected.chunks(2) {
            let (child_1, child_2) = self.apply_legacy(metadata, parents[0], parents[1]);
            output.push(child_1);
            output.push(child_2);
        }
    }
}
