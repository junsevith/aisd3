pub mod select;
pub mod stats;
pub mod arr_gen;
pub mod chart;
pub mod search;

mod tests {
    use log::{debug, LevelFilter, trace};
    use rand::{Rng, thread_rng};
    use crate::arr_gen::random_array;
    use crate::chart::draw_chart;
    use crate::search::bin_search;
    use crate::select::{rand_select, select};
    use crate::stats::{setup_logger, Stats};
    use super::*;

    #[test]
    fn test() {
        let look = 12;
        let mut stats = Stats::new();
        let mut vals = random_array(40, &mut rand::thread_rng());

        setup_logger(vals.len());

        let before = format!("Before: {:?}", vals);
        trace!("{}", before);

        let found = *rand_select(&mut vals, look, &mut stats);

        trace!("{}", before);
        trace!("After: {:?}", vals);

        debug!("For: {} found: {} {}", look, found, if found == vals[look - 1] {"Correct!"} else {"Incorrect!"});

        debug!("Stats: {:?}", stats);

        vals.sort();
        trace!("Sorted: {:?}", vals)
    }

    #[test]
    fn test2() {
        let size = 39;
        setup_logger(size);
        // for _i in 0..100 {
            let look = 12;
            let group = 5;
            let mut stats = Stats::new();
            let mut vals = random_array(size, &mut rand::thread_rng());


            let before = format!("Before: {:?}", vals);
            trace!("{}", before);

            let found = select(&mut vals, look, group, &mut stats);

            trace!("{}", before);
            trace!("After: {:?}", vals);

            debug!("For: {} found: {} {}", look, found, if found == *vals.clone().select_nth_unstable(look-1).1 {"Correct!"} else {"Incorrect!"});

            debug!("Stats: {:?}", stats);

            vals.sort();
            trace!("Sorted: {:?}", vals)
        // }
    }

    #[test]
    fn test3() {
        let data = vec![
            ("Numero uno", vec![3, 6, 9, 11, 14, 5, 7, 8, 9, 10]),
            ("Numero dos", vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]),
            ("Numero tres", vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            ("Numero cuatro", vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        ];
        draw_chart(data, 0..10, "dupa")
    }

    #[test]
    fn test4() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        setup_logger(vec.len());
        let mut stats = Stats::new();
        let res = bin_search(&vec, &11, &mut stats);
        println!("{}", res);
        println!("{:?}", stats);

    }
}

