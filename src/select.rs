use std::fmt::Debug;
use log::trace;

use rand::prelude::IndexedRandom;
use rand::Rng;

use crate::stats::Stats;

// static mut COMPS: usize = 0;
// static mut SWAPS: usize = 0;
// 
// fn stats.comp() {
//     unsafe {
//         COMPS += 1;
//     }
// }
// 
// fn stats.swap() {
//     unsafe {
//         SWAPS += 1;
//     }
// }
// 
// fn reset() {
//     unsafe {
//         COMPS = 0;
//         SWAPS = 0;
//     }
// }
// 
// fn stats() -> (usize, usize) {
//     unsafe {
//         let res = (COMPS, SWAPS);
//         reset();
//         res
//     }
// }
// 



pub fn rand_select<'a, T: Ord + Debug>(array: &'a mut [T], pos: usize, stats: &mut Stats) -> &'a mut T {
    if array.len() == 1 {
        return &mut array[0];
    }
    let r = rand_partition(array, stats);
    trace!("partitioned: {:?} around {:?}", array,  array[r]);

    let k = r + 1;
    return if pos == k {
        &mut array[r]
    } else if pos < k {
        rand_select(&mut array[..r], pos, stats)
    } else {
        rand_select(&mut array[k..], pos - k, stats)
    };
}

fn rand_partition<T: Ord>(array: &mut [T], stats: &mut Stats) -> usize {
    let r = rand::thread_rng().gen_range(0..array.len());
    array.swap(array.len() - 1, r);
    hoare_partition(array, stats)
}

pub fn hoare_partition<T: Ord>(array: &mut [T], stats: &mut Stats) -> usize {
    let last = array.len() - 1;

    let (pivot, sorting) = array.split_last_mut().unwrap();

    let mut i = 0;
    let mut j = last - 1;
    loop {
        while {
            stats.comp();
            i < last - 1 && sorting[i] < *pivot
        } {
            i += 1;
        }
        while {
            stats.comp();
            j > 0 && sorting[j] > *pivot
        } {
            j -= 1;
        }

        if i >= j {
            break;
        } else {
            stats.swap();
            sorting.swap(i, j);

            i += 1;
            j -= 1;
        }
    }


    stats.comp();
    if sorting[j] < *pivot { j += 1; }
    stats.swap();
    array.swap(j, last);
    j
}

pub fn hoare_partition_pivot<T: Ord>(array: &mut [T], pivot: &T, stats: &mut Stats) -> usize {
    let mut i = 0;
    let mut j = array.len() - 1;
    loop {
        while {
            stats.comp();
            &array[i] <= pivot
        } {
            i += 1;
        }
        while {
            stats.comp();
            &array[j] > pivot
        } {
            j -= 1;
        }

        if i >= j {
            break;
        }

        stats.swap();
        array.swap(i, j);

        i += 1;
        j -= 1;
    }
    j
}

pub fn lomuto_partition<T: Ord>(array: &mut [T], stats: &mut Stats) -> usize {
    let pivot = array.len() - 1;
    let mut i = 0;
    for j in 0..array.len() {
        if {
            stats.comp();
            array[j] < array[pivot]
        } {
            stats.swap();
            array.swap(i, j);
            i += 1;
        }
        stats.comp();
    }
    stats.swap();
    array.swap(i, pivot);
    i
}

pub fn select<T: Ord + Clone + Debug>(array: &mut [T], pos: usize, group: usize, stats: &mut Stats) -> T {
    if array.len() <= group {
        insertion_sort(array, stats);
        return array[pos - 1].clone();
    }

    let mut pivot = median_pivot(array, group, stats);

    let r = hoare_partition_pivot(array, &pivot, stats);
    let k = r + 1;
    trace!("partitioned: {:?} around {:?}, r= {:?}", array,  pivot, r);
    return if pos == k {
        pivot
    } else if pos < k {
        select(&mut array[..k], pos, group, stats)
    } else {
        select(&mut array[k..], pos - k, group, stats)
    };
}

fn median_pivot<T: Ord + Clone + Debug>(array: &mut [T], pos: usize, stats: &mut Stats) -> T {
    let mut medians = array.chunks_mut(pos).map(|chunk| median(chunk, stats).clone()).collect::<Vec<_>>();
    trace!("Found medians: {:?}", medians);
    let mid = medians.len() / 2;
    select(&mut medians, mid, pos, stats)
}

fn median<'a, T: Ord + Clone>(array: &'a mut [T], stats: &mut Stats) -> &'a T {
    insertion_sort(array, stats);
    &array[array.len() / 2]
}

fn insertion_sort<T: Ord + Clone>(array: &mut [T], stats: &mut Stats) {
    for i in 0..array.len() {
        let mut j = i;
        while j > 0 && {
            stats.comp();
            array[j] < array[j - 1]
        } {
            stats.swap();
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

// pub fn select_v2<T, G, F>(array: &mut [T], pos: usize, group: usize, value: F) -> &T
//     where F: Fn(&T) -> G,
//           G: Ord {
//     let mut medians = Vec::with_capacity(array.len() / group);
//     array.chunks_mut(group).enumerate().for_each(|(i, chunk)| {
//         chunk.sort_by_key(&value);
//         medians.push(group * i + chunk.len() / 2);
//     });
//
//     let val = |x: &usize| value(&array[*x]);
//     let mid = medians.len() / 2;
//     let pivot = select_v2(&mut medians, mid, group, &val);
//     array.swap(array.len() - 1, *pivot);
//
//     let r = hoare_partition_key(array, &value);
//     let k = r + 1;
//
//     // println!("{:?} {:?}: {:?}", array, r, array[r]);
//     return if pos == k {
//         &array[r]
//     } else if pos < k {
//         select_v2(&mut array[..r], pos, group, value)
//     } else {
//         select_v2(&mut array[k..], pos - k, group, value)
//     };
// }
//
// pub fn hoare_partition_key<T, F, G: Ord>(array: &mut [T], value: F) -> usize
//     where F: Fn(&T) -> G {
//     let last = array.len() - 1;
//
//     let (pivot, sorting) = array.split_last_mut().unwrap();
//     let pivot = value(pivot);
//
//     let mut i = 0;
//     let mut j = last - 1;
//     loop {
//         while {
//             stats.comp();
//             i < last && value(&sorting[i]) < pivot
//         } {
//             i += 1;
//         }
//         while {
//             stats.comp();
//             j > 0 && value(&sorting[j]) > pivot
//         } {
//             j -= 1;
//         }
//
//         if i >= j {
//             break;
//         } else {
//             stats.swap();
//             sorting.swap(i, j);
//
//             i += 1;
//             j -= 1;
//         }
//     }
//
//
//     j += 1;
//     stats.swap();
//     array.swap(j, last);
//     j
// }
//
// pub fn trial<T, G: Ord, F>(val: T, val2: T, fun: F) -> bool
//     where F: Fn(&T) -> G {
//     fun(&val) < fun(&val2)
// }