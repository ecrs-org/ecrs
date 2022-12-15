# ECRS - Evolutionary Computation for Rust

**Disclaimer** Please note that this library is in early development phase (latest version is `0.1.0-beta.1`) and breaking changes may occur without any notice.

Evolutionary computation tools & algorithms.

The library provides:

* [Genetic algorithm](src/ga.rs) generic implementation with series of operators
* [Ant System algorithm](src/aco.rs) implementation
* [Firefly algorithm](src/ff.rs) implementation
* [PSO algorithm](src/pso.rs) implementation

For genetic algorithm there are various genetic operators & utility predefined:

* Crossover operators:
	* SinglePoint
	* TwoPoint
	* MultiPoint
	* Uniform
	* Ordered
	* PMX
* Selection operators:
	* RouletteWheel
	* Random
	* Rank
	* RankR
	* Tournament
	* StochasticUniversalSampling
	* Boltzmann
* Mutation operators:
	* Identity
	* FlipBit
	* Interchange
	* Reversing
* Population generatos
	* RandomPoints
	* BitStrings

Each operator can be used in plug-in style to alternate algorithm behaviour.

The library also offers highly customizable logging system based on "probing". You can check out our [examples](example/)

## Get started

### Installation

To add `ecrs` to your project simply make use of `cargo add` command:

```
cargo add ecrs
```

### Usage

Work in progess...

For now the best method to get started is checking out our [examples](example/)

## MSRV

During this stage of development there is not MSRV policy estabilished yet. Currently `MSRV == 1.65.0` as there are some usages of syntax introduced in `1.65.0` in the codebase.
