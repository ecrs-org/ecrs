pub mod probe;
mod solution;

use rand::Rng;
use std::collections::HashSet;
use std::iter::zip;
use std::ops::Add;

pub use solution::Solution;

use crate::aco::AntSystemCfg;
use crate::aco::FMatrix;
/// Wrapper class for AntSystem algorithm.
///
/// To extract data use a [probe](probe)
pub struct AntSystem {
    cfg: AntSystemCfg,
    pheromone: FMatrix,
    best_sol: Solution,
}

impl AntSystem {
    /// Creates a new instance of AntSystem using config
    pub fn new(cfg: AntSystemCfg) -> AntSystem {
        let pheromone = FMatrix::repeat(cfg.weights.nrows(), cfg.weights.ncols(), 0.5f64);
        AntSystem {
            cfg,
            pheromone,
            best_sol: Solution::default(),
        }
    }
    /// Executes the algorithm
    pub fn execute(mut self) {
        for i in 0..self.cfg.iteration {
            self.cfg.probe.on_iteration_start(i);
            self.iterate();
            self.cfg.probe.on_iteration_end(i);
        }

        self.end()
    }
    #[doc(hidden)]
    fn iterate(&mut self) {
        let sols_m = self.run_ants();
        let sols = self.grade(sols_m);

        let best = self.find_best(&sols);
        self.cfg.probe.on_current_best(best);
        self.update_best(best);
        let d_pheromone = sols
            .iter()
            .map(|sol| sol.matrix.scale(1.0 / sol.cost))
            .reduce(|s1, s2| s1.add(s2))
            .expect("d_pheromone creation error");

        let new_pheromone: FMatrix = self
            .pheromone
            .scale(1.0 - self.cfg.evaporation_rate)
            .add(&d_pheromone);

        self.cfg
            .probe
            .on_pheromone_update(&self.pheromone, &new_pheromone);
        self.pheromone = new_pheromone;
    }
    #[doc(hidden)]
    fn update_best(&mut self, current_best: &Solution) {
        if self.best_sol > *current_best {
            self.cfg.probe.on_new_best(current_best);
            self.best_sol = (*current_best).clone();
        }
    }
    #[doc(hidden)]
    fn find_best<'a>(&mut self, sols: &'a [Solution]) -> &'a Solution {
        let best = sols.iter().min_by(|a, b| (*a).partial_cmp(*b).unwrap());

        best.unwrap()
    }
    #[doc(hidden)]
    fn grade(&self, sols_m: Vec<FMatrix>) -> Vec<Solution> {
        let costs: Vec<f64> = Vec::from_iter(sols_m.iter().map(|s| self.grade_one(s)));
        let mut sols: Vec<Solution> = Vec::new();
        for (m, c) in zip(sols_m, costs) {
            sols.push(Solution { matrix: m, cost: c })
        }

        sols
    }
    #[doc(hidden)]
    fn grade_one(&self, s: &FMatrix) -> f64 {
        s.component_mul(&self.cfg.weights).sum() / 2.0
    }
    #[doc(hidden)]
    fn run_ants(&self) -> Vec<FMatrix> {
        let prob_iter = self
            .pheromone
            .iter()
            .zip(self.cfg.heuristic.iter())
            .map(|(p, h)| self.calc_prob(p, h));

        let prob =
            FMatrix::from_iterator(self.pheromone.nrows(), self.pheromone.ncols(), prob_iter);

        let sols: Vec<FMatrix> = Vec::from_iter((0..self.cfg.ants_num).map(|_| run_ant(&prob)));

        sols
    }

    #[doc(hidden)]
    fn calc_prob(&self, p: &f64, h: &f64) -> f64 {
        p.powf(self.cfg.alpha) * h.powf(self.cfg.beta)
    }
    #[doc(hidden)]
    fn end(mut self) {
        self.cfg.probe.on_end();
    }
}
#[doc(hidden)]
fn run_ant(prob: &FMatrix) -> FMatrix {
    let n = prob.nrows();
    let mut sol = FMatrix::zeros(n, n);
    let mut random = rand::thread_rng();
    let mut unvisited: HashSet<usize> = HashSet::from_iter(0..n);

    let first: usize = random.gen_range(0..n);
    unvisited.remove(&first);
    let mut last: usize = first;

    while !unvisited.is_empty() {
        let mut sum = 0.0_f64;
        let row = prob.row(last);
        for v in unvisited.iter() {
            sum += row[*v];
        }

        let r_range = 0.0..sum;
        if r_range.is_empty() {
            println!("Could not find a solution");
            return FMatrix::zeros(n, n);
        }

        let mut r = random.gen_range(r_range);
        let mut next = last; // maybe 0
        for v in unvisited.iter() {
            r -= row[*v];
            if r <= 0.0 {
                next = *v;
                break;
            }
        }

        sol[(last, next)] = 1.0;
        sol[(next, last)] = 1.0;
        unvisited.remove(&next);
        last = next;
    }

    sol[(last, first)] = 1.0;
    sol[(first, last)] = 1.0;

    sol
}
