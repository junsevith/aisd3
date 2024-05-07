use std::fmt::Debug;
use log::trace;
use rand::Rng;
use crate::stats::Stats;

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
    }
    stats.swap();
    array.swap(i, pivot);
    i
}