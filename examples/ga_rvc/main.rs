use ecrs::ga;

mod rastrigin;

fn main() {
  let best_individual = ga::Builder::with_rvc()
    .fitness_fn(rastrigin::rastrigin_fitness)
    .dim(5)
    .build()
    .run()
    .unwrap();

  println!("5D Rastrigin function zero approximation {:#?}", best_individual)
}
