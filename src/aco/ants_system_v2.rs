mod builder;
mod solution;
pub mod probe;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::zip;
use std::ops::{Add, Mul, Sub};
use std::slice::Iter;
use nalgebra::{Dynamic, OMatrix};
use rand::Rng;

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

pub use builder::AntSystemBuilder;
pub use solution::Solution;
use crate::aco::ants_system_v2::probe::Probe;


pub struct AntSystem {
    weights: FMatrix,
    heuristic: FMatrix,
    pheromone: FMatrix,
    best_sol: Solution,
    alpha: f64,
    beta: f64,
    evaporation_rate: f64,
    ants_num: usize,
    iteration: usize,
    probe: Box<dyn Probe>
}

impl AntSystem {
    pub fn iterate(&mut self) {
        self.probe.on_iteration_start(self.iteration);

        let sols_m = self.run_ants();
        let sols = self.grade(sols_m);

        let best = self.find_best(&sols);
        self.probe.on_current_best(best);
        self.update_best(best);
        let d_pheromone = sols.iter()
            .map(|sol| sol.matrix.scale(1.0/ sol.cost))
            .reduce(|s1,s2| s1.add(s2))
            .expect("d_pheromone creation error");

        let new_pheromone: FMatrix = self.pheromone
            .scale(1.0 - self.evaporation_rate)
            .add(&d_pheromone);


        self.pheromone = new_pheromone;

        self.probe.on_iteration_end(self.iteration);
        self.iteration += 1;
    }

    fn update_best(&mut self, current_best: &Solution) {
        if self.best_sol > *current_best {
            self.probe.on_new_best(current_best);
            self.best_sol = (*current_best).clone();
        }
    }

    fn find_best<'a>(&mut self, sols: &'a Vec<Solution>) -> &'a Solution {
        let best = sols.iter()
            .min_by(|a,b| (*a).partial_cmp(*b).unwrap());

        best.unwrap()
    }

    fn grade(&self, sols_m: Vec<FMatrix>) -> Vec<Solution> {
        let costs: Vec<f64> = Vec::from_iter(sols_m.iter().map(|s| self.grade_one(s)));
        let mut sols: Vec<Solution> = Vec::new();
        for (m,c) in zip(sols_m, costs) {
            sols.push(Solution{
                matrix: m,
                cost: c
            })
        }

        sols
    }

    fn grade_one(&self, s: &FMatrix) -> f64 {
        s.component_mul(&self.weights).sum() / 2.0
    }

    fn run_ants(&self) -> Vec<FMatrix> {
        let prob_iter = self.pheromone.iter()
            .zip(self.heuristic.iter())
            .map(|(p, h)| self.calc_prob(p, h));

        let prob = FMatrix::from_iterator(
            self.pheromone.nrows(),
            self.pheromone.ncols(),
            prob_iter
        );


        let sols: Vec<FMatrix> =  Vec::from_iter(
            (0..self.ants_num)
                .map(|_| run_ant(&prob))
        );

        sols
    }

    fn calc_prob(&self ,pheromone: &f64, heuristic: &f64) -> f64 {
        pheromone.powf(self.alpha) * heuristic.powf(self.beta)
    }

    pub fn end(mut self) {
        self.probe.on_end();
    }
}

fn run_ant(prob: &FMatrix) -> FMatrix {
    let n = prob.nrows();
    let mut sol = FMatrix::zeros(n, n);
    let mut random = rand::thread_rng();
    let mut unvisited: HashSet<usize> = HashSet::from_iter(1..n);

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
            return FMatrix::zeros(n,n);
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

