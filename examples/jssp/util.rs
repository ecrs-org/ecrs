#![allow(dead_code)]

use std::{collections::{HashSet, HashMap, hash_map}, fmt::Display, path::{PathBuf, Path}};

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
        ("diversity".to_owned(), base_dir.join("event_diversity.csv")),
        ("newbest".to_owned(), base_dir.join("event_newbest.csv")),
        ("bestingen".to_owned(), base_dir.join("event_bestingen.csv")),
        ("popgentime".to_owned(), base_dir.join("event_popgentime.csv")),
        ("iterinfo".to_owned(), base_dir.join("event_iterinfo.csv"))
    ])
}

pub fn assert_dir_exists(dir: &Path) {
    if dir.is_dir() {
        return;
    }

    match std::fs::create_dir_all(dir) {
        Ok(()) => return,
        Err(err) => panic!("Failed to create outuput directory with error {err}"),
    };
}
