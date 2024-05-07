use std::cmp::Ordering;
use std::fmt::Debug;
use log::trace;
use crate::stats::Stats;

pub fn bin_search<T: Ord + Debug>(array: &[T], element: &T, stats: &mut Stats) -> bool {
    let len = array.len();
    return if len == 0 { false } else {
        trace!("Searching in: {:?}", array);
        let mid = len / 2;
        stats.comp();
        return match element.cmp(&array[mid]) {
            Ordering::Equal => true,
            Ordering::Less => bin_search(&array[..mid], element, stats),
            Ordering::Greater => bin_search(&array[(mid + 1)..], element, stats)
        };
    };
}