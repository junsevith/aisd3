#![feature(iter_next_chunk)]

use std::{env, io};
use std::io::BufRead;
use std::iter::successors;
use log::{debug, trace};
use lista3::select::{rand_select, select};
use lista3::stats::{setup_logger, Stats};

fn main() {
    let input = env::args().skip(1).next_chunk::<2>().expect("Za mało argumentów");
    let look = input[0].parse::<usize>().expect("Niepoprawna wartość");
    let group = input[1].parse::<usize>().expect("Niepoprawna wartość");



    println!("Podaj dane:");
    let mut vals = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<i32>().expect("Niepoprawna wartość")).collect::<Vec<i32>>();

    let mut stats = Stats::new();
    setup_logger(vals.len());

    let before = format!("Before: {:?}", vals);
    trace!("{}", before);

    let found = select(&mut vals, look, group, &mut stats);

    trace!("{}", before);
    trace!("After: {:?}", vals);

    debug!("For: {} found: {} {}", look, found, if found == *vals.clone().select_nth_unstable(look-1).1 {"Correct!"} else {"Incorrect!"});

    debug!("Stats: {:?}", stats);

    vals.sort();
    trace!("Sorted: {:?}", vals)
}