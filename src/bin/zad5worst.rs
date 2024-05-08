use std::time::Instant;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use lista3::arr_gen::sorted_array;
use lista3::chart::draw_chart;
use lista3::quicksort::{dual_pivot_quicksort, quick_sort};
use lista3::select_sort::{select_dual_pivot_quicksort, select_quicksort};
use lista3::stats::Stats;

fn main() {
    const FUNCTIONS: usize = 4;
    const STATS: usize = 3;
    const REPEATS: usize = 200;

    let n = (100..5_000).step_by(100);
    let size = n.size_hint().0;
    Pcg64Mcg::from_entropy();
    let mut data = vec![vec![Vec::with_capacity(size); FUNCTIONS]; STATS];

    for i in n.clone() {
        let mut cur_data = [[[0usize; REPEATS]; FUNCTIONS]; STATS];
        println!("{}", i);
        for j in 0..REPEATS {
            let mut sample = sorted_array(i);

            {
                let num = 0;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                let before = Instant::now();
                quick_sort(&mut copy, &mut stats);
                cur_data[2][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
                cur_data[1][num][j] = stats.swaps;
            }

            {
                let num = 1;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                let before = Instant::now();
                dual_pivot_quicksort(&mut copy, &mut stats);
                cur_data[2][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
                cur_data[1][num][j] = stats.swaps;
            }

            {
                let num = 2;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                let before = Instant::now();
                select_quicksort(&mut copy, 5, &mut stats);
                cur_data[2][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
                cur_data[1][num][j] = stats.swaps;
            }

            {
                let num = 3;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                let before = Instant::now();
                select_dual_pivot_quicksort(&mut copy, 5, &mut stats);
                cur_data[2][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
                cur_data[1][num][j] = stats.swaps;
            }
        }

        for j in 0..STATS {
            for k in 0..FUNCTIONS {
                data[j][k].push(cur_data[j][k].iter().sum::<usize>() as f64 / REPEATS as f64);
            }
        }
    }
    let stat_names = ["Comparisons", "Swaps", "Time"];
    let fun_names = ["quicksort", "double pivot quicksort", "select quicksort", "select double pivot quicksort"];

    data.clone().into_iter().enumerate().for_each(|(count, data)| {
        let data = fun_names.into_iter().zip(data.into_iter()).collect::<Vec<_>>();
        draw_chart(data, n.clone(), &format!("zad5worst_{}", stat_names[count]), |x, y| y)
    });

    data.into_iter().enumerate().for_each(|(count, data)| {
        let data = fun_names.into_iter().zip(data.into_iter()).collect::<Vec<_>>();
        draw_chart(data, n.clone(), &format!("zad5worst_per_nlogn_{}", stat_names[count]), |x, y| y / (x * x.log2()))
    })
}
