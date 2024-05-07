use std::{env, io};
use std::io::BufRead;
use log::{debug, trace};
use lista3::rand_select::rand_select;
use lista3::stats::{setup_logger, Stats};

fn main() {
    let look = env::args().skip(1).next().expect("Za mało argumentów")
        .parse::<usize>().expect("Niepoprawna wartość");

    println!("Podaj dane:");
    let mut vals = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<i32>().expect("Niepoprawna wartość")).collect::<Vec<i32>>();

    let mut stats = Stats::new();
    setup_logger(vals.len());

    let before = format!("Before: {:?}", vals);
    trace!("{}", before);

    let found = *rand_select(&mut vals, look, &mut stats);

    trace!("{}", before);
    trace!("After: {:?}", vals);

    debug!("For: {} found: {} {}", look, found, if found == vals[look - 1] {"Correct!"} else {"Incorrect!"});

    debug!("Stats: {:?}", stats);

    vals.sort();
    trace!("Sorted: {:?}", vals)
}