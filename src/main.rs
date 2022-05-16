mod city;

use rand::{Rng, thread_rng};
use std::fmt;
use std::fmt::Formatter;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use std::f64::consts;
use city::City;
use log::{warn, info};

// struct Config {
//     elite_size: u32,
//     num_of_cites: u32,
//     pop_size: u32,
//     mutation_rate: f64,
//     generations: u32,
// }
//
// struct GeneticAlgorithm {
//     config: Config,
// }
//
// impl GeneticAlgorithm {
//   fn run(&self) {
//     let mut rng = rand::thread_rng();
//
//     let mut cities: Vec<City> = Vec::new();
//
//     while cities.len() < self.config.num_of_cites as usize {
//       let tempcity: City = rng.gen();
//       if !cities.contains(&tempcity){
//         cities.push(tempcity);
//       }
//     }
//
//     println!("Miasta:");
//     for city in &cities {
//       println!("{}", city);
//     }
//
//     let mut population: Vec<Vec<City>> = Vec::new(); //Populacja jako wektor wektorów miast, naszą drogą są połączenia w kolejności wektora
//
//     while population.len() < self.config.pop_size as usize { //Populacja początkowa
//       cities.shuffle(&mut thread_rng());
//       population.push(cities.clone());
//     }
//     for index in 1..=self.config.generations {
//       population = self.selection(&population); //selekcja
//       while population.len() < self.config.pop_size as usize { //rozmnażanie selekcji do wymaganej wielkości populacji
//         population.push(self.breed(&population[rng.gen_range(0..population.len())],
//                               &population[rng.gen_range(0..population.len())]))
//       }
//       for index in 0..population.len() as u32 {
//         if (rng.gen_range(0..=100)) as f64 >= 10000 as f64 * self.config.mutation_rate { //losowanie, czy mutacja nastąpi
//           population[index as usize] = self.mutate(&population[index as usize]); //mutacja
//         }
//       }
//       if index % 10 == 0{
//         println!("Pokolenie {}: top fitness {}, dystans: {}", index, self.fitness(&population[0]), (100000 as f64)/ self.fitness(&population[0]));
//       }
//     }
//   }
//
//   fn distance(&self, city_a: &City, city_b: &City) -> f64 {
//     f64::sqrt(f64::powi((city_a.x - city_b.x) as f64, 2) + f64::powi((city_a.y - city_b.y) as f64, 2))
//   }
//
//   fn fitness(&self, cities: &Vec<City>) -> f64 {
//     let mut  fit: f64 = 0 as f64;
//     for index in 0..cities.len()-1{
//       fit += self.distance(&cities[index], &cities[index+1]);
//     }
//     (100000 as f64)/fit
//   }
//
//   fn selection(&self, cities: &Vec<Vec<City>>) -> Vec<Vec<City>>{ //Algo selekcji
//     let mut temp: Vec<Vec<City>> = cities.clone();
//     temp.sort_by(|a,b| self.fitness(a).partial_cmp(&self.fitness(b)).unwrap());
//     temp.reverse();
//     let mut result: Vec<Vec<City>> = Vec::new();
//     for index in 0..self.config.elite_size { //Zachowujemy ELITESIZE najlepszych
//       result.push(temp[index as usize].clone())
//     }
//     result
//   }
//
//   fn breed(&self, route_a :&Vec<City>, route_b: &Vec<City>) -> Vec<City>{
//     let routea = route_a.clone();
//     let routeb = route_b.clone();
//
//     let mut result: Vec<City> = Vec::new();
//     let gene_a: u32 = thread_rng().gen_range(0..route_a.len()-2) as u32;
//     let gene_b: u32 = thread_rng().gen_range((gene_a+1) as usize..route_a.len()-1) as u32;
//
//     let transplant_me:Vec<City>;
//     transplant_me = routea[gene_a as usize..gene_b as usize].to_owned();
//     let mut index = 0;
//     while result.len() <= gene_a as usize{
//       if !transplant_me.contains(&route_b[index]){
//         result.push(routeb[index]);
//       }
//       index +=1;
//     }
//     result.append(& mut transplant_me.clone());
//     for index in gene_a as usize..route_a.len() as usize {
//       if !result.contains(&routeb[index]){
//         result.push(routeb[index])
//       }
//     }
//     result
//   }
//
//   fn mutate(&self, route:&Vec<City>) -> Vec<City> {
//     let gene_a = thread_rng().gen_range(0..route.len());
//     let gene_b = thread_rng().gen_range(0..route.len());
//     let mut result = route.clone();
//     result.swap(gene_a as usize, gene_b as usize);
//     result
//   }
// }

struct GeneticAlgorithmCfg<T> {
  mutation_rate: f32,
  selection_rate: f32,
  generation_upper_bound: i32,
  population_size: i32,
  eps: f32,
  fitness_fn: fn(&T) -> f32,
  mutation_operator: fn(T) -> T,
  crossover_operator: fn(&T, &T) -> T,
  selection_operator: fn(&[T], usize)-> T,
  population_factory: fn(i32) -> Vec<T>,
}

// impl<T> Default for GeneticAlgorithmCfg<T> {
//   fn default() -> Self {
//     GeneticAlgorithmCfg {
//       mutation_rate: 0.08,
//       selection_rate: 0.5,
//       generation_upper_bound: 200,
//       population_size: 100,
//
//     }
//   }
// }

fn rastrigin_fitness_function(chromosome: &[f64]) -> f64 {
  return 10_f64 * chromosome.len() as f64 + chromosome.iter().fold(0_f64, |sum, x| {
    sum + x.powi(2) - 10_f64 * (2_f64 * consts::PI * x).cos()
  })
}

fn custom_selection_operator<'a>(population: &[&'a[f64]], population_size: usize) -> &'a[f64] {
  // if let Some(choosen_one) = population.iter().map(|x| rastrigin_fitness_function(x) ).min() {
  //   return choosen_one
  // }
  if let Some(choosen_one) = population.get(thread_rng().gen_range(0..population.len())) {
    return choosen_one
  }
  unimplemented!()
}

fn custom_mutation_operator(chromosome: Vec<f64>) -> Vec<f64> {
  let idx = thread_rng().gen_range(0..chromosome.len());
  let mut new_chromosome = chromosome.clone();

  let mut rng = thread_rng();
  let mut distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let random_value = distribution.sample(&mut rng) as f64;
  new_chromosome[idx] = random_value;
  new_chromosome
}

fn custom_population_factory(population_size: i32) -> Vec<f64> {
  let mut rng = thread_rng();
  let mut distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let mut population = Vec::with_capacity(population_size as usize);
  for _ in 0..population_size {
    population.push(distribution.sample(&mut rng) as f64);
  }
  population
}

struct GeneticAlgorithm<T: Ord + Clone> {
  config: GeneticAlgorithmCfg<T>,
  rng: ThreadRng,
}

impl<T: Ord + Clone> GeneticAlgorithm<T> {
  fn new(config: GeneticAlgorithmCfg<T>) -> Self {
    GeneticAlgorithm {
      config,
      rng: thread_rng()
    }
  }

  fn maybe_apply_mutation_operator(&self, target: T) -> T {
    // TODO
    (self.config.mutation_operator)(target)
  }

  fn run(&mut self) -> Option<T> {
    let mut population = (self.config.population_factory)(self.config.population_size);

    for generation_idx in 0..self.config.generation_upper_bound {
      let mut new_generation: Vec<T> = Vec::with_capacity(self.config.population_size as usize);

      // TODO: consider to parametrize this
      for _ in 0..(self.config.population_size / 2) {
        let (mut father, mut mother) = (
          (self.config.selection_operator)(&population, self.config.population_size as usize),
          (self.config.selection_operator)(&population, self.config.population_size as usize)
        );

        // TODO: possible optimization
        father = self.maybe_apply_mutation_operator(father);
        mother = self.maybe_apply_mutation_operator(mother);

        let mut child = (self.config.crossover_operator)(&father, &mother);
        new_generation.push(child);
        if self.rng.gen_bool(self.config.selection_rate as f64) {
          new_generation.push(father);
        } else {
          new_generation.push(mother);
        }
      }

      population = new_generation;

      if let Some(chromosome) = population.iter().min() {
        if (self.config.fitness_fn)(chromosome) < self.config.eps {
          return Option::Some((*chromosome).clone());
        }
        println!("Generation: {}, best fit: {}", generation_idx + 1, (self.config.fitness_fn)(chromosome));
      }
    }
    if let Some(chromosome) = population.iter().min() {
      if (self.config.fitness_fn)(chromosome) < self.config.eps {
        return Option::Some((*chromosome).clone());
      } else {
        None
      }
    } else {
      None
    }
  }
}

fn main() {
  // let alg = GeneticAlgorithm {
  //   config: Config {
  //     elite_size: 20,
  //     num_of_cites: 25,
  //     pop_size: 100,
  //     mutation_rate: 0.01,
  //     generations: 500,
  //   }
  // };
  // alg.run();
  println!("Hello world");
}
