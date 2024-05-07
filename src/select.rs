use std::fmt::Debug;
use std::iter;
use itertools::Itertools;
use log::trace;
use rand::Rng;
use crate::rand_select::hoare_partition;
use crate::stats::Stats;

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

pub fn select<'a, T: Ord + Debug>(array: &'a mut [T], pos: usize, group: usize, stats: &mut Stats) -> &'a T {
    trace!("Selecting {:?} from {:?}", pos, array);
    if array.len() <= group {
        insertion_sort(array, stats);
        return &array[pos - 1];
    }

    median_pivot(array, group, stats);

    let index = hoare_partition(array, stats);
    let stat = index + 1;
    trace!("using {:?} partitioned: {:?}", array[index], array );
    return if pos == stat {
        &array[index]
    } else if pos < stat {
        select(&mut array[..index], pos, group, stats)
    } else {
        select(&mut array[stat..], pos - stat, group, stats)
    };
}

///This function finds best pivot and places it at the end of the array
pub fn median_pivot<T: Ord + Debug>(array: &mut [T], group: usize, stats: &mut Stats) {
    let mut index = 0;

    for (start, end) in (0..array.len()).step_by(group).chain(iter::once(array.len())).tuple_windows() {
        insertion_sort(&mut array[start..end], stats);
        stats.swap();
        array.swap(index, (start + end) / 2);
        index += 1;
    }

    // for chunk in (0..array.len()).chunks(group) {
    //     let vals = chunk.collect::<Vec<_>>();
    //     let first = *vals.first().unwrap();
    //     let last = *vals.last().unwrap();
    //     let mid = (last + first) / 2;
    //     insertion_sort(&mut array[first..=last], stats);
    //     stats.swap();
    //     array.swap(index, mid);
    //     index += 1;
    // }

    let medians = &mut array[..index];
    trace!("Found medians: {:?}", medians);

    let mid = medians.len() / 2;
    let a = select(medians, mid + 1, group, stats);
    trace!("Selected pivot: {:?}", a);

    stats.swap();
    array.swap(mid, array.len() - 1);
}

pub(crate) fn median<'a, T: Ord >(array: &'a mut [T], stats: &mut Stats) -> &'a T {
    insertion_sort(array, stats);
    &array[array.len() / 2]
}

pub(crate) fn insertion_sort<T: Ord >(array: &mut [T], stats: &mut Stats) {
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