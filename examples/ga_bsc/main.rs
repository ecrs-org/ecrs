use ecrs::ga;

mod wordmax;

fn main() {
  let best_individual = ga::Builder::with_bsc()
    .fitness_fn(wordmax::wordmax_fitness)
    .dim(10)
    .set_population_size(6)
    .build()
    .run()
    .unwrap();

  println!("Bitstring with most ones: {:#?}", best_individual)
}
