

#[cfg(test)]
pub mod tests;

use std::io;
use crate::aoc_utils;
use crate::day1::utils;
use std::collections::{HashSet};

const MAX_ITERATIONS:i32 = 1_000_000;

pub fn find_first_duplicate_frequency(vector:&Vec<i32>) -> Option<i32> {

    let mut seen_frequencies:HashSet<i32> = HashSet::new();

    let mut current_value = 0;

    seen_frequencies.insert(current_value);

    for _i in 0..MAX_ITERATIONS {

        for freq_change in vector {

            current_value += freq_change;

            if seen_frequencies.contains(&current_value) {
                return Some(current_value);
            } else {
                seen_frequencies.insert(current_value);
            }
        }

    }

    return None;
}


pub fn start(input:&str) -> io::Result<()> {
    println!(" :- Day 2 Part 2");
    println!("    using file: '{}'", input);

    aoc_utils::ensure_file(input);
    let numbers = utils::read_frequencies(input)?;
    let total_freq = find_first_duplicate_frequency(&numbers);

    match total_freq {
        Some(x) => { println!("    result: {}", x); Ok(()) }
        None => Err(io::Error::new(io::ErrorKind::InvalidInput,
                               "could not find a duplicate nr"))
    }
}