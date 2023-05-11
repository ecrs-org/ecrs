use std::{collections::HashSet, fmt::Display};

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
