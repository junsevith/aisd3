extern crate core;

use std::env;
use rand::Rng;

fn main() {
    let args = env::args();
    let size = args.skip(1).next().expect("Za mało argumentów").parse::<usize>().expect("Niepoprawna wartość");
    let mut rng = rand::thread_rng();
    let arr = (0..size).map(|_| rng.gen_range(0..2*size)).collect::<Vec<usize>>();
    let str_arr = arr.iter().fold(String::new(), |a,b| a + &*b.to_string() + " ");
    println!("{}", str_arr)
}
