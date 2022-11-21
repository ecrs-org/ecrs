use ecrs::ga::{population::{RandomPoints, PopulationGenerator}, operators::crossover::{SinglePoint, CrossoverOperator, TwoPoint, MultiPoint, Uniform}};

fn operator_takes_values_from_parents<T: CrossoverOperator<Vec<f64>>>(mut operator: T) {
	let parents = RandomPoints::new(30).generate(2);
	assert_eq!(parents.len(), 2, "Expected population of size 2");

	let (child_1, child_2) = operator.apply(&parents[0], &parents[1]);
	for (i, (gene_1, gene_2)) in std::iter::zip(child_1.chromosome_ref(), child_2.chromosome_ref()).enumerate() {
		assert!(parents[0].chromosome_ref()[i] == *gene_1 || parents[1].chromosome_ref()[i] == *gene_1);
		assert!(parents[0].chromosome_ref()[i] == *gene_2 || parents[1].chromosome_ref()[i] == *gene_2);
	}
}

#[test]
fn single_point_takes_values_from_parents() {
	operator_takes_values_from_parents(SinglePoint::new());
}

#[test]
fn two_point_takes_values_from_parents() {
	operator_takes_values_from_parents(TwoPoint::new());
}

#[test]
fn multi_point_takes_values_from_parents() {
	for n in 4..=8 {
		operator_takes_values_from_parents(MultiPoint::new(n));
	}
}

#[test]
fn uniform_takes_values_from_parents() {
	operator_takes_values_from_parents(Uniform::new());
}
