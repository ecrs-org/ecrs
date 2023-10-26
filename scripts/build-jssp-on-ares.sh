#!/usr/bin/bash
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --partition=plgrid
#SBATCH --time=00:10:00
#SBATCH --account=plglscclass23-cpu

# $1: module name
function load_module_if_needed() {
  module_name=$1
  echo "Adding module ${module_name}"
  if [[ $(module is-loaded ${module_name}) -ne 0 ]]; then
    module add ${module_name}
  fi
}

echo "Building jssp [release]"
load_module_if_needed rust/1.65.0-gcccore-12.2.0
cargo build --example jssp --release
mkdir -p bin
mv target/release/examples/jssp bin/

