











use super::CrossoverOperator;

pub mod fixed_point;
pub mod multi_point;
pub mod ordered;
pub mod pmx;
pub mod ppx;
pub mod shuffle;
pub mod single_point;
pub mod two_point;
pub mod uniform;
pub mod uniform_parameterized;

pub use fixed_point::FixedPoint;
pub use multi_point::MultiPoint;
pub use ordered::OrderedCrossover;
pub use pmx::Pmx;
pub use ppx::Ppx;
pub use shuffle::Shuffle;
pub use single_point::SinglePoint;
pub use two_point::TwoPoint;
pub use uniform::Uniform;
pub use uniform_parameterized::UniformParameterized;

#[cfg(test)]
mod test {
    use crate::ga::individual::IndividualTrait;
    use crate::ga::operators::crossover::{CrossoverOperator, FixedPoint, Pmx, Ppx, Shuffle};
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

        let (child_1, child_2) = op.apply_legacy(&GAMetadata::default(), &p1, &p2);
        assert_eq!(child_1.chromosome.len(), 10);
        assert_eq!(child_2.chromosome.len(), 10);
    }

    #[test]
    fn shuffle_fulfills_conditions() {
        let mut op = Shuffle::new();

        let p1 = Individual::from(vec![1, 0, 0, 1, 0, 1, 0, 1, 0, 0]);
        let p2 = Individual::from(vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1]);

        let (c1, c2) = op.apply_legacy(&GAMetadata::default(), &p1, &p2);
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

        let (child_1, child_2) = op.apply_legacy(&GAMetadata::default(), &p1, &p2);

        let child_1_expected_chromosome = vec![8, 4, 7, 3, 4, 5, 6, 7, 8, 9];
        let child_2_expected_chromosome = vec![0, 1, 2, 3, 6, 2, 5, 1, 9, 0];

        assert_eq!(child_1.chromosome(), &child_1_expected_chromosome);
        assert_eq!(child_2.chromosome(), &child_2_expected_chromosome);
    }
}
