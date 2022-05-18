mod probe;

use std::f64;
use rand::{Rng, thread_rng};

//TODO dodać setting wyboru rozkładu epsilona

pub struct FireflyAlgorithmCfg {
    dimensions: u8,
    //Nr of dimensions
    lower_bound: f64,
    //Lower search bound
    upper_bound: f64,
    //Upper search bound
    max_generations: u32,
    //Maximum amount of generations
    population_size: u32,
    //Population size
    alfa0: f64,
    //Initial randomness coefficient
    beta0: f64,
    //Attractiveness coefficient, in most cases leave as 1
    gamma: f64,
    //Light absorption coefficient
    delta: f64,
    //Randomness decrease modifier, 0<delta<1
}

impl Default for FireflyAlgorithmCfg {
    fn default() -> Self {
        FireflyAlgorithmCfg {
            dimensions: 2,
            lower_bound: -5.0,
            upper_bound: 5.0,
            max_generations: 1000,
            population_size: 25,
            alfa0: 1.0,
            beta0: 1.0,
            gamma: 0.01,
            delta: 0.97,
        }
    }
}

pub struct FireflyAlgorithm {
    pub config: FireflyAlgorithmCfg,
    pub brightness_function: fn(&Vec<f64>) -> f64,
}

impl FireflyAlgorithm {
    fn new(config: FireflyAlgorithmCfg, brightness_function: fn(&Vec<f64>) -> f64) -> Self {
        FireflyAlgorithm {
            config,
            brightness_function,
        }
    }

    pub fn execute(&self) {

        let mut population: Vec<Vec<f64>> = Vec::new();
        for _index in 0..self.config.population_size as usize { //Generacja populacji
            let mut temp: Vec<f64> = Vec::new();
            for _dim in 0..self.config.dimensions {
                temp.push(thread_rng().gen_range(self.config.lower_bound as f64..self.config.upper_bound as f64));
            }
            population.push(temp);
        }
        let mut brightness: Vec<f64> = Vec::new();
        let temp = population.clone();
        for point in temp {
            brightness.push(1 as f64 / (self.brightness_function)(&point)); //TODO USUŃ TEMP CLONEA
        }
        let scale = self.config.upper_bound - self.config.lower_bound;
        let mut alfa = self.config.alfa0;
        let mut rng = thread_rng();
        for generation in 0..self.config.max_generations {
            for index in 0 as usize..self.config.population_size as usize {
                for innerindex in 0 as usize..self.config.population_size as usize {
                    if brightness[index] < brightness[innerindex] {
                        let const1 = self.config.beta0 * f64::powf(f64::consts::E, -1 as f64 * self.config.gamma * f64::powi(distance(&population[index], &population[innerindex]), 2));
                        for dimension in 0 as usize..self.config.dimensions as usize {
                            population[index][dimension] += const1 * (population[innerindex][dimension] - population[index][dimension]) + self.config.alfa0 * alfa * (rng.gen_range(0.01..0.99) - 0.5) * scale;
                        }
                        brightness[index] = 1 as f64 / (self.brightness_function)(&population[index]);
                    }
                }
            }
            alfa = alfa * self.config.delta;
            if generation % 25 == 0 {
                //TODO LOG
                let mut maxpos = 0;
                let mut maxbright = 0 as f64;
                for index in 0 as usize..self.config.population_size as usize { //TODO POPRAW ZNAJDOWANIE MAXA
                    if brightness[index] == f64::INFINITY {
                        maxpos = index;
                        break;
                    }
                    if brightness[index] > maxbright {
                        maxbright = brightness[index];
                        maxpos = index;
                    }
                }
                println!("Gen: {}, x: {}, y: {}", generation, population[maxpos][0], population[maxpos][1]);
            }
        }
        println!("END");
    }
}

pub fn distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 { //Distance between two points
    let mut res: f64 = 0 as f64;
    for dimension in 0..a.len() {
        res += f64::powi(a[dimension] - b[dimension], 2)
    }
    f64::sqrt(res)
}