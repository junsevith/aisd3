use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use lista3::arr_gen::sorted_array;
use lista3::chart::draw_chart;
use lista3::search::bin_search;
use lista3::stats::Stats;

fn main() {
    const FUNCTIONS: usize = 5;
    const STATS: usize = 2;
    const REPEATS: usize = 200;

    let n = (1000..100_000).step_by(1000);
    let size = n.size_hint().0;
    let mut rng = Pcg64Mcg::from_entropy();
    let mut data = vec![vec![Vec::with_capacity(size); FUNCTIONS]; STATS];

    for i in n.clone() {
        let mut cur_data = [[[0usize; REPEATS]; FUNCTIONS]; STATS];
        println!("{}", i);
        for j in 0..REPEATS {
            let mut sample = sorted_array(i+1);
            let rem = sample.remove(rng.gen_range(0..sample.len()));

            {
                let num = 0;
                let look = rng.gen_range(0..i);
                let mut stats = Stats::new();
                let before = Instant::now();
                bin_search(&sample, &look,&mut stats);
                cur_data[1][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
            }

            {
                let num = 1;
                let look = rng.gen_range(0..(i/9));
                let mut stats = Stats::new();
                let before = Instant::now();
                bin_search(&sample, &look,&mut stats);
                cur_data[1][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
            }

            {
                let num = 2;
                let look = rng.gen_range((i*4/9)..(i*5/9));
                let mut stats = Stats::new();
                let before = Instant::now();
                bin_search(&sample, &look,&mut stats);
                cur_data[1][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
            }

            {
                let num = 3;
                let look = rng.gen_range((i*8/9)..i);
                let mut stats = Stats::new();
                let before = Instant::now();
                bin_search(&sample, &look,&mut stats);
                cur_data[1][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
            }

            {
                let num = 4;
                let look = rem;
                let mut stats = Stats::new();
                let before = Instant::now();
                bin_search(&sample, &look,&mut stats);
                cur_data[1][num][j] = before.elapsed().as_nanos() as usize;
                cur_data[0][num][j] = stats.comps;
            }
        }

        for j in 0..STATS {
            for k in 0..FUNCTIONS {
                data[j][k].push( cur_data[j][k].iter().sum::<usize>() as f64 / REPEATS as f64);
            }
        }
    }
    let stat_names = ["Comparisons", "Time"];
    let fun_names = ["Random element", "From beginning", "From middle", "From end", "Outside element"];

    data.into_iter().enumerate().for_each(|(count, data)| {
        let data = fun_names.into_iter().zip(data.into_iter()).collect::<Vec<_>>();
        draw_chart(data, n.clone(), &format!("zad4_{}", stat_names[count]))
    })
}