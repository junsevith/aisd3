use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn random_array(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut arr = sorted_array(size);
    arr.shuffle(rng);
    return arr;
}

pub fn sorted_array(size: usize) -> Vec<usize> {
    (1..=size).collect::<Vec<usize>>()
}

pub fn reverse_array(size: usize) -> Vec<usize> {
    (1..=size).rev().collect::<Vec<usize>>()
}


pub fn random_array2(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut range = Uniform::new(0, 2*size-1).unwrap();
    let mut vec = Vec::with_capacity(size);
    for _i in 0..size {
        vec.push(range.sample(rng))
    };
    vec
}
pub fn sorted_array2(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut vec = random_array2(size, rng);
    vec.sort();
    vec
}

pub fn reverse_array2(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut vec = sorted_array2(size, rng);
    vec.reverse();
    vec
}