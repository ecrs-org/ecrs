#[allow(clippy:ptr_arg)]
pub fn wordmax_fitness(chromosome: &Vec<bool>) -> f64 {
  chromosome.iter().filter(|gene| **gene).count() as f64
}
