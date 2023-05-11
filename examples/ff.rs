#![cfg(feature = "ff")]

use ecrs::ff::FireflyAlgorithm;

fn main() {
    let mut alg = FireflyAlgorithm::default();

    alg.run();
}

#[cfg(not(feature = "ff"))]
compile_error!("Required feature \"ff\" is not enabled");
