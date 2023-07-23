extern crate rand;
use rand::Rng;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use proconio::input;
use std::time::Instant;

// 評価関数

fn main() {
    // get start time
    let start = Instant::now();

    // input
    input! {
        n: usize,
        m: usize,
        d: usize,
        _k: usize,
        uvw: [(usize, usize, usize); m],
        _xy: [[usize; 2]; n],
    }
    const STOP_VALUE: usize = 1_000_000_000;

    // now_ans
    let mut now_ans = vec![0; m];
    for i in 0..m {
        now_ans[i] = i % d
    }
    let mut cnt = 0;

    loop {
        cnt += 1;
        // 乱数
        let random_way: usize = rand::thread_rng().gen_range(0, m);
        let random_day: usize = rand::thread_rng().gen_range(0, d);
        let now_day = now_ans[random_way];
        let fut_day = random_day;

        // fut_ans
        let mut fut_ans = now_ans.clone();
        fut_ans[random_way] = fut_day;


        // 合計値計算
        let mut now_sum_weight = 0;
        let mut fut_sum_weight = 0;

        for day in [now_day, fut_day] {
            let mut now_uvw = uvw.clone();
            let mut fut_uvw = uvw.clone();
            let mut now_stop_root = vec![];
            let mut fut_stop_root = vec![];

            // 通行止め
            for i in 0..m {
                if now_ans[i] == day {
                    now_uvw[i].2 = STOP_VALUE;
                    now_stop_root.push(i);
                }

                if fut_ans[i] == day {
                    fut_uvw[i].2 = STOP_VALUE;
                    fut_stop_root.push(i);
                }
            }
            // ダイクストラ法
            let now_g = UnGraph::<usize, usize, usize>::from_edges(&now_uvw);
            let fut_g = UnGraph::<usize, usize, usize>::from_edges(&fut_uvw);

            // now weight
            for i in now_stop_root {
                let start = uvw[i].0;
                let end = uvw[i].1;
                let weight = uvw[i].2;

                let now_res = dijkstra(&now_g, start.into(), None, |now_uvw| *now_uvw.weight());
                let now_weight =  now_res.get(&NodeIndex::new(*&end)).unwrap();
                let dif_weight = now_weight - weight;

                now_sum_weight += dif_weight;
            }

            // now weight
            for i in fut_stop_root {
                let start = uvw[i].0;
                let end = uvw[i].1;
                let weight = uvw[i].2;

                let fut_res = dijkstra(&fut_g, start.into(), None, |fut_uvw| *fut_uvw.weight());
                let fut_weight =  fut_res.get(&NodeIndex::new(*&end)).unwrap();
                let dif_weight = fut_weight - weight;

                fut_sum_weight += dif_weight;
            }

        }

        if fut_sum_weight < now_sum_weight {
            println!("{}", "change");
            now_ans[random_way] = fut_day;
        }

        // get end time
        let end = start.elapsed();
        let end_sec = end.as_secs_f64();
        println!("{}", cnt);

        if cnt > 5 {
            println!("{}", end_sec);
            break;
        }
    }

    
    // output
    println!("{}", cnt);
    for i in &now_ans {
        print!("{} ", i+1)
    }
}
