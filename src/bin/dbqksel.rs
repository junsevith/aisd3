#![feature(iter_next_chunk)]

use std::{env, io};
use std::io::BufRead;
use log::{debug, trace};
use lista3::select::{select};
use lista3::select_sort::{select_dual_pivot_quicksort, select_quicksort};
use lista3::stats::{setup_logger, Stats};

fn main() {
    let group = env::args().skip(1).next().expect("Za mało argumentów").parse::<usize>().expect("Niepoprawna wartość");

    println!("Podaj dane:");
    let mut vals = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<i32>().expect("Niepoprawna wartość")).collect::<Vec<i32>>();

    let mut stats = Stats::new();
    setup_logger(vals.len());

    let before = format!("Before: {:?}", vals);
    trace!("{}", before);

    select_dual_pivot_quicksort(&mut vals, group, &mut stats);

    trace!("{}", before);
    trace!("After: {:?}", vals);
    debug!("Stats: {:?}", stats);
}