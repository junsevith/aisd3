use std::fmt::Debug;
use crate::rand_select::hoare_partition;
use crate::stats::Stats;

pub fn quick_sort<T: Ord + Clone + Debug>(array: &mut [T], stats: &mut Stats) {
    let pivot = hoare_partition(array, stats);
    quick_sort(&mut array[..pivot], stats);
    quick_sort(&mut array[(pivot + 1)..], stats);
}

pub fn dual_pivot_quicksort<T: Ord>(array: &mut [T], stats: &mut Stats) {

    let (p1, p2) = dual_pivot_partition(array, stats);

    dual_pivot_quicksort(&mut array[..p1], stats);

    dual_pivot_quicksort(&mut array[p1 + 1..p2], stats);

    dual_pivot_quicksort(&mut array[p2 + 1..], stats);
}

fn dual_pivot_partition<T: Ord>(array: &mut [T], stats: &mut Stats) -> (usize, usize) {
    let mut smaller = 0;
    let mut bigger = 0;
    let (left_pivot, right_pivot) = (0, array.len() - 1);

    if array[left_pivot] > array[right_pivot] {
        stats.swap();
        array.swap(left_pivot, right_pivot);
    }

    let mut left = left_pivot + 1;
    let mut right = right_pivot - 1;
    let mut iter = left;

    while iter <= right {
        if bigger > smaller {
            if {
                stats.comp();
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                stats.swap();

                right -= 1;
                bigger += 1;
            } else if {
                stats.comp();
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                stats.swap();

                left += 1;
                iter += 1;
                smaller += 1
            } else {
                iter += 1;
            }
        } else {
            if {
                stats.comp();
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                stats.swap();

                left += 1;
                iter += 1;
                smaller += 1;
            } else if {
                stats.comp();
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                stats.swap();

                right -= 1;
                bigger += 1;
            } else {
                iter += 1;
            }
        }
    }

    left -= 1;
    right += 1;
    stats.swap();
    array.swap(left_pivot, left);
    stats.swap();
    array.swap(right_pivot, right);

    (left, right)
}