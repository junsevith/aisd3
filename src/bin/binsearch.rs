use std::{env, io};
use std::io::BufRead;
use std::iter::successors;
use log::{debug, trace};
use lista3::search::bin_search;
use lista3::select::rand_select;
use lista3::stats::{setup_logger, Stats};

fn main() {
    let look = env::args().skip(1).next().expect("Za mało argumentów")
        .parse::<usize>().expect("Niepoprawna wartość");

    println!("Podaj dane:");
    let mut vals = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<usize>().expect("Niepoprawna wartość")).collect::<Vec<_>>();

    let mut stats = Stats::new();
    setup_logger(vals.len());

    let res = bin_search(&mut vals, &look, &mut stats);
    debug!("For: {} found: {} {}", look, res, if vals.contains(&look) {"Correct!"} else {"Incorrect!"});
}