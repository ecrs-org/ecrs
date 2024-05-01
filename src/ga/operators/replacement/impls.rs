use crate::ga::{individual::IndividualTrait, Metrics};

use super::ReplacementOperator;

/// # BothParents replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It works simply by replacing parents with their children. In effect, each individual
/// only gets to breed once.
///
/// **NOTE**: In current implementation, all library-implemented operators assume that
/// at indices i, i+1 in `population` collection there are parents of children i, i+1
/// from `children` collection. Any violation of this invariant may lead to bugs - it can
/// be considered an undefined behaviour. We'll work towards improving this case in the future.
pub struct BothParents;

impl BothParents {
    /// Returns new instance of [BothParents] replacement operator.
    pub fn new() -> Self {
        Self
    }
}

impl<IndividualT: IndividualTrait> ReplacementOperator<IndividualT> for BothParents {
    /// Works simply by replacing parents with their children
    ///
    /// **NOTE**: In current implementation, all library-implemented operators assume that
    /// at indices i, i+1 in `population` collection there are parents of children i, i+1
    /// from `children` collection. Any violation of this invariant may lead to bugs - it can
    /// be considered an undefined behaviour. We'll work towards improving this case in the future.
    ///
    /// ### Arguments
    ///
    /// * `population` - Original population, input to the crossover phase.
    /// This collection should be modified in place by the operator.
    /// * `children` - Result of the crossover phase
    #[inline(always)]
    fn apply(
        &mut self,
        _metrics: &Metrics,
        _population: Vec<IndividualT>,
        children: Vec<IndividualT>,
    ) -> Vec<IndividualT> {
        children
    }

    /// Returns `true` when the operator requires children to possess valid fitness values.
    ///
    /// This implementation returns `false`.
    #[inline(always)]
    fn requires_children_fitness(&self) -> bool {
        false
    }
}

/// # Noop replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It does nothing. Implementation is a noop.
pub struct Noop;

impl Noop {
    /// Returns new instance of [Noop] replacement operator
    pub fn new() -> Self {
        Self
    }
}

impl<IndividualT: IndividualTrait> ReplacementOperator<IndividualT> for Noop {
    /// Returns input `population`.
    #[inline(always)]
    fn apply(
        &mut self,
        _metrics: &Metrics,
        population: Vec<IndividualT>,
        _children: Vec<IndividualT>,
    ) -> Vec<IndividualT> {
        population
    }

    /// Returns `true` when the operator requires children to possess valid fitness values.
    ///
    /// This implementation returns `false`.
    #[inline(always)]
    fn requires_children_fitness(&self) -> bool {
        false
    }
}

/// # WeakParent replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// Works by taking two out of four individuals (two parents and two children) with the largest fitness.
///
/// **NOTE**: In current implementation, all library-implemented operators assume that
/// at indices i, i+1 in `population` collection there are parents of children i, i+1
/// from `children` collection. Any violation of this invariant may lead to bugs - it can
/// be considered an undefined behaviour. We'll work towards improving this case in the future.
///
/// **NOTE**: This operator assumes that the size of `population` and `children` are of same size.
/// Assertion is performed only in debug build. Breaking this condition may lead to bugs and can be thought
/// of as undefined behaviour.
///
/// **NOTE**: This operator assumes that the size of `population` and `children` is a even number.
/// Assertion is performed only in debug build. Breaking this condition may lead to bugs and can be thought
/// of as undefined behaviour. This restriction will be removed in future versions
/// of the library.
pub struct WeakParent;

impl WeakParent {
    /// Returns new instance of [WeakParent] replacement operator.
    pub fn new() -> Self {
        Self
    }
}

impl<IndividualT: IndividualTrait> ReplacementOperator<IndividualT> for WeakParent {
    /// Works by taking two out of four individuals (two parents and two children) with the largest fitness.
    ///
    /// **NOTE**: In current implementation, all library-implemented operators assume that
    /// at indices i, i+1 in `population` collection there are parents of children i, i+1
    /// from `children` collection. Any violation of this invariant may lead to bugs - it can
    /// be considered an undefined behaviour. We'll work towards improving this case in the future.
    ///
    /// **NOTE**: This operator assumes that the size of `population` and `children` are of same size.
    /// Assertion is performed only in debug build. Breaking this condition may lead to bugs and can be thought
    /// of as undefined behaviour.
    ///
    /// **NOTE**: This operator assumes that the size of `population` and `children` is a even number.
    /// Assertion is performed only in debug build. Breaking this condition may lead to bugs and can be thought
    /// of as undefined behaviour. This restriction will be removed in future versions
    /// of the library.
    ///
    /// ### Arguments
    ///
    /// * `population` - Original population, input to the crossover phase.
    /// This collection should be modified in place by the operator.
    /// * `children` - Result of the crossover phase
    fn apply(
        &mut self,
        _metrics: &Metrics,
        mut population: Vec<IndividualT>,
        mut children: Vec<IndividualT>,
    ) -> Vec<IndividualT> {
        debug_assert_eq!(
            population.len(),
            children.len(),
            "Population and children must be of the same size"
        );
        debug_assert!(population.len() % 2 == 0, "Population size must be even");

        // Unfortunately array windowing is not in stable Rust yet, I believe
        // https://doc.rust-lang.org/std/slice/struct.ArrayChunks.html

        // There is for sure a nicer way to implement this ;D
        for i in (0..(population.len() - 1)).step_by(2) {
            if population[i] < population[i + 1] {
                population.swap(i, i + 1);
            }

            if children[i] < children[i + 1] {
                children.swap(i, i + 1);
            }

            if children[i] > population[i] {
                population.swap(i, i + 1);
                std::mem::swap(&mut children[i], &mut population[i]);

                if children[i + 1] > population[i + 1] {
                    std::mem::swap(&mut children[i + 1], &mut population[i + 1]);
                }
            } else if children[i] > population[i + 1] {
                std::mem::swap(&mut children[i], &mut population[i + 1]);
            }
        }
        population
    }
}

#[cfg(test)]
mod tests {
    use crate::ga::{Individual, Metrics};

    use super::{BothParents, Noop, ReplacementOperator, WeakParent};

    #[test]
    fn noop_has_new_method() {
        let _ = Noop::new();
    }

    #[test]
    fn both_parents_has_new_method() {
        let _ = BothParents::new();
    }

    #[test]
    fn weak_parent_swaps_when_children_are_stronger() {
        let parents = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 40.0,
            },
        ];

        let children = vec![
            Individual {
                chromosome: 0.0,
                fitness: 120.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 100.0,
            },
        ];

        let children_clone = children.clone();

        let result = WeakParent::new().apply(&Metrics::default(), parents, children);

        assert_eq!(result, children_clone);
    }

    #[test]
    fn weak_parent_does_not_swap_when_parents_are_stronger() {
        let parents = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 40.0,
            },
        ];

        let children = vec![
            Individual {
                chromosome: 0.0,
                fitness: 10.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 12.0,
            },
        ];

        let parents_clone = parents.clone();

        let result = WeakParent::new().apply(&Metrics::default(), parents, children);

        assert_eq!(result, parents_clone);
    }

    #[test]
    fn weak_parent_cross_swaps_child_1() {
        let parents = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 40.0,
            },
        ];

        let children = vec![
            Individual {
                chromosome: 0.0,
                fitness: 50.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 30.0,
            },
        ];

        let expected_result = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 50.0,
            },
        ];

        let result = WeakParent::new().apply(&Metrics::default(), parents, children);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn weak_parent_cross_swaps_child_2() {
        let parents = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 40.0,
            },
        ];

        let children = vec![
            Individual {
                chromosome: 0.0,
                fitness: 30.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 50.0,
            },
        ];

        let expected_result = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 50.0,
            },
        ];

        let result = WeakParent::new().apply(&Metrics::default(), parents, children);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn weak_parent_takes_two_best() {
        let parents = vec![
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 40.0,
            },
        ];

        let children = vec![
            Individual {
                chromosome: 0.0,
                fitness: 70.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 50.0,
            },
        ];

        let expected_result = vec![
            Individual {
                chromosome: 0.0,
                fitness: 70.0,
            },
            Individual {
                chromosome: 0.0,
                fitness: 60.0,
            },
        ];

        let result = WeakParent::new().apply(&Metrics::default(), parents, children);

        assert_eq!(result, expected_result);
    }
}
