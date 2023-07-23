use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use proconio::input;
// use rand::Rng;
// 評価関数
// evaluate_socre

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

    // 元のans
    let mut ans = vec![0; m];
    let mut sum_weight: usize = 0;
    for i in 0..m {
        ans[i] = i % d
    }

    // map 作成
    const STOP_VALUE: usize = 1_000_000_000;

    // 元の合計値計算    
    let mut sum_weight = 0;
    let g = UnGraph::<usize, usize, usize>::from_edges(&uvw);

    for i in 0..m {
        let start = uvw[i].0;
        let end = uvw[i].1;
        let res = dijkstra(&g, start.into(), None, |u| *u.weight());
        sum_weight += res.get(&NodeIndex::new(*&end)).unwrap();
    }
    
    println!("{}", sum_weight)
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
