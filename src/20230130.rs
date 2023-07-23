extern crate rand;
use rand::Rng;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use proconio::input;

// 評価関数

fn main() {

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

        // 通行止め
        for i in 0..m {
            if now_ans[i] == now_day {
                now_uvw[i].2 = STOP_VALUE;
            }

            if fut_ans[i] == fut_day {
                fut_uvw[i].2 = STOP_VALUE;
            }
        }
        // ダイクストラ法
        let now_g = UnGraph::<usize, usize, usize>::from_edges(&now_uvw);
        let fut_g = UnGraph::<usize, usize, usize>::from_edges(&fut_uvw);

        for i in 0..m {
            let start = uvw[i].0;
            let end = uvw[i].1;

            let now_res = dijkstra(&now_g, start.into(), None, |now_uvw| *now_uvw.weight());
            let fut_res = dijkstra(&fut_g, start.into(), None, |fut_uvw| *fut_uvw.weight());

            now_sum_weight += now_res.get(&NodeIndex::new(*&end)).unwrap();
            fut_sum_weight += fut_res.get(&NodeIndex::new(*&end)).unwrap();
        }

    }
    println!("{} {}", now_sum_weight, fut_sum_weight);
    // let g = UnGraph::<usize, usize, usize>::from_edges(&uvw);

    // for i in 0..m {
    //     let start = uvw[i].0;
    //     let end = uvw[i].1;
    //     let res = dijkstra(&g, start.into(), None, |uvw| *uvw.weight());
    //     sum_weight += res.get(&NodeIndex::new(*&end)).unwrap();
    // }
    
    // 山登り
    // let mut sum_point: usize = 0; 
    // let mut random_root = rng.gen_range(0, m);
    // let mut random_day  = rng.gen_range(0, d);

    // ans[random_root] = random_day;

    // for day in 0..d {
    //     let mut next_uvw = uvw;
    //     for m_day in 0..m {
    //         if ans[m_day] == day {
    //             let u: usize = uvw[i][0];
    //             let v: usize = uvw[i][1];
    //             next_root_map[(u-1) as usize][(v-1) as usize] = STOP_VALUE;
    //         }
    //     }
    //     // 最短経路
    //     for i in 0..m {
    //         // 始点と終点
    //         let u: usize = uvw[i][0];
    //         let v: usize = uvw[i][1];


    //     }
    // }

    // // 評価してよかったら変更

    
    // // output
    // for i in &ans {
    //     print!("{} ", i+1)
    // }
    
}
