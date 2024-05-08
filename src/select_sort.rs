use std::fmt::Debug;
use std::iter;
use itertools::Itertools;
use log::trace;
use crate::quicksort::dual_pivot_partition;

use crate::rand_select::hoare_partition;
use crate::select::{insertion_sort, median_pivot, select};
use crate::stats::Stats;

pub fn select_quicksort<T: Ord + Debug>(array: &mut [T], group: usize, stats: &mut Stats) {
    if array.len() <= group {
        insertion_sort(array, stats);
    } else {
        median_pivot(array, 5, stats);
        let pivot = hoare_partition(array, stats);
        select_quicksort(&mut array[..pivot], group, stats);
        select_quicksort(&mut array[(pivot + 1)..], group, stats);
    }
}

pub fn select_dual_pivot_quicksort<T: Ord + Debug>(array: &mut [T], group: usize, stats: &mut Stats) {
    if array.len() <= group {
        insertion_sort(array, stats);
    } else {
        median_dual_pivot(array, group, stats);
        let (p1, p2) = dual_pivot_partition(array, stats);
        select_dual_pivot_quicksort(&mut array[..p1], group, stats);
        select_dual_pivot_quicksort(&mut array[(p1 + 1)..p2], group, stats);
        select_dual_pivot_quicksort(&mut array[(p2 + 1)..], group, stats);
    }
}

// fn dual_pivot_partition<T: Ord>(array: &mut [T], stats: &mut Stats) -> (usize, usize) {
//     let mut smaller = 0;
//     let mut bigger = 0;
//     let (left_pivot, right_pivot) = (0, array.len() - 1);
//
//     // if array[left_pivot] > array[right_pivot] {
//     //     stats.swap();
//     //     array.swap(left_pivot, right_pivot);
//     // }
//
//     let mut left = left_pivot + 1;
//     let mut right = right_pivot - 1;
//     let mut iter = left;
//
//     while iter <= right {
//         if bigger > smaller {
//             if {
//                 stats.comp();
//                 array[iter] > array[right_pivot]
//             } {
//                 array.swap(iter, right);
//                 stats.swap();
//
//                 right -= 1;
//                 bigger += 1;
//             } else if {
//                 stats.comp();
//                 array[iter] < array[left_pivot]
//             } {
//                 array.swap(iter, left);
//                 stats.swap();
//
//                 left += 1;
//                 iter += 1;
//                 smaller += 1
//             } else {
//                 iter += 1;
//             }
//         } else {
//             if {
//                 stats.comp();
//                 array[iter] < array[left_pivot]
//             } {
//                 array.swap(iter, left);
//                 stats.swap();
//
//                 left += 1;
//                 iter += 1;
//                 smaller += 1;
//             } else if {
//                 stats.comp();
//                 array[iter] > array[right_pivot]
//             } {
//                 array.swap(iter, right);
//                 stats.swap();
//
//                 right -= 1;
//                 bigger += 1;
//             } else {
//                 iter += 1;
//             }
//         }
//     }
//
//     left -= 1;
//     right += 1;
//     stats.swap();
//     array.swap(left_pivot, left);
//     stats.swap();
//     array.swap(right_pivot, right);
//
//     (left, right)
// }

pub fn median_dual_pivot<T: Ord + Debug>(array: &mut [T], group: usize, stats: &mut Stats) {
    let mut index = 0;

    for (start, end) in (0..array.len()).step_by(group).chain(iter::once(array.len())).tuple_windows() {
        insertion_sort(&mut array[start..end], stats);
        stats.swap();
        array.swap(index, (start + end) / 2);
        index += 1;
    }

    let medians = &mut array[..index];
    let len = medians.len();
    trace!("Found medians: {:?}", medians);

    let p1i = len / 3;
    let p2i = 2 * len / 3;

    let p1 = format!("{:?}", select(medians, p1i + 1, group, stats));
    let p2 = format!("{:?}", select(medians, p2i + 1, group, stats));
    trace!("Selected pivots: {:?}, {:?}", p1, p2);

    stats.swap();
    array.swap(p1i, 0);
    stats.swap();
    array.swap(p2i, len - 1);
}