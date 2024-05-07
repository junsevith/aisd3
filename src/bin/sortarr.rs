use std::env;

fn main() {
    let args = env::args();
    let size = args.skip(1).next().expect("Za mało argumentów").parse::<usize>().expect("Niepoprawna wartość");
    let arr = (0..size).collect::<Vec<usize>>();
    let str_arr = arr.iter().fold(String::new(), |a,b| a + &*b.to_string() + " ");
    println!("{}", str_arr)
}