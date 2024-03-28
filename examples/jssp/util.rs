#![allow(dead_code)]

use std::{
    collections::{hash_map, HashMap, HashSet},
    fmt::Display,
    path::{Path, PathBuf},
};

pub fn print_hash_set<T: Display>(set: &HashSet<T>) {
    for elem in set {
        print!("{elem}, ");
    }
    println!();
}

pub fn print_slice<T: Display>(slc: &[T]) {
    for elem in slc {
        print!("{elem}, ");
    }
    println!();
}

#[inline]
pub fn create_event_map(base_dir: &Path) -> HashMap<String, PathBuf> {
    HashMap::from([
        ("popmetrics".to_owned(), base_dir.join("event_popmetrics.csv")),
        ("newbest".to_owned(), base_dir.join("event_newbest.csv")),
        ("bestingen".to_owned(), base_dir.join("event_bestingen.csv")),
        ("popgentime".to_owned(), base_dir.join("event_popgentime.csv")),
        ("iterinfo".to_owned(), base_dir.join("event_iterinfo.csv")),
    ])
}

pub fn assert_dir_exists(dir: &Path) {
    if dir.is_dir() {
        return;
    }

    match std::fs::create_dir_all(dir) {
        Ok(()) => (),
        Err(err) => panic!("Failed to create output directory with error {err}"),
    };
}

// Hey this surely can be done better (optimise it a bit)
#[inline]
pub fn euclidean_distance(vec_1: &[f64], vec_2: &[f64]) -> f64 {
    vec_1
        .iter()
        .zip(vec_2.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .reduce(|a, b| a + b)
        .unwrap()
        .sqrt()
}
