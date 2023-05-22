use coco_rs::Problem;
use ecrs::ga::{
    individual::{IndividualTrait, RealValueIndividual},
    operators::fitness::Fitness,
};

type RealVector = Vec<f64>;

pub struct CocoFitness<'a, 'suite> {
    coco_problem: &'a mut Problem<'suite>,
    output_cell: [f64; 1],
}

impl<'a, 'suite> CocoFitness<'a, 'suite> {
    pub fn new(problem: &'a mut Problem<'suite>) -> Self {
        Self {
            coco_problem: problem,
            output_cell: [f64::MIN],
        }
    }
}

impl<'a, 'suite> Fitness<RealValueIndividual> for CocoFitness<'a, 'suite> {
    fn apply(&mut self, individual: &mut RealValueIndividual) -> f64 {
        self.coco_problem
            .evaluate_function(individual.chromosome(), &mut self.output_cell);
        self.output_cell[0]
    }
}

// pub struct CocoPopulationGenerator<'a, 'suite> {
// 	coco_problem: &'a Problem<'suite>
// }

// impl<'a, 'suite> CocoPopulationGenerator<'a, 'suite> {
// 	pub fn new(problem: &'a Problem<'suite>) -> Self {
// 		Self {
// 			coco_problem: problem
// 		}
// 	}
// }

// impl<'a, 'suite> PopulationGenerator<RealVector> for CocoPopulationGenerator<'a, 'suite> {
// 	fn generate(&mut self, count: usize) -> Vec<ecrs::ga::Individual<RealVector>> {
// 		unimplemented!()
// 	}
// }
