#![allow(dead_code)]

use ga::pipeline::pipeline_test;

mod aco;
mod ga;
mod pso;
mod ff;
mod examples;
mod test_functions;

fn main() {
	// examples::ga_example()
	pipeline_test()
}
