use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use lista3::arr_gen::random_array;
use lista3::chart::draw_chart;
use lista3::rand_select::rand_select;
use lista3::select::{ select};
use lista3::stats::Stats;

fn main() {
    const FUNCTIONS: usize = 2;
    const STATS: usize = 2;
    const REPEATS: usize = 50;

    let n = (100..50_000).step_by(100);
    let size = n.size_hint().0;
    let mut rng = Pcg64Mcg::from_entropy();
    let mut data = vec![vec![Vec::with_capacity(size); FUNCTIONS]; STATS];

    for i in n.clone() {
        let mut cur_data = [[[0usize; REPEATS]; FUNCTIONS]; STATS];
        println!("{}", i);
        for j in 0..REPEATS {
            let sample = random_array(i, &mut rng);

            {
                let num = 0;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                rand_select(&mut copy, REPEATS, &mut stats);
                cur_data[0][num][j] = stats.comps;
                cur_data[1][num][j] = stats.swaps;
            }

            {
                let num = 1;
                let mut copy = sample.clone();
                let mut stats = Stats::new();
                select(&mut copy, REPEATS, 5, &mut stats);
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
    let stat_names = ["Comparisons", "Swaps"];
    let fun_names = ["Randomized Select", "Median Select"];

    data.clone().into_iter().enumerate().for_each(|(count, data)| {
        let data = fun_names.into_iter().zip(data.into_iter()).collect::<Vec<_>>();
        draw_chart(data, n.clone(), &format!("zad2_{}", stat_names[count]), |x, y| y)
    });

    data.into_iter().enumerate().for_each(|(count, data)| {
        let data = fun_names.into_iter().zip(data.into_iter()).collect::<Vec<_>>();
        draw_chart(data, n.clone(), &format!("zad2_per_n_{}", stat_names[count]), |x, y| y / x)
    })
}