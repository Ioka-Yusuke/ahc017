extern crate rand;
use rand::Rng;
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
        _xy: [[f64; 2]; n],
    }

    // uvw_map
    const STOP_VALUE: usize = 1;
    let mut uvw_map = vec![vec![vec![0; n]; n]; d];

    // now_ans
    let mut now_ans = vec![0; m];
    for i in 0..m {
        now_ans[i] = i % d;
        let u = uvw[i].0;
        let v = uvw[i].1;
        uvw_map[(i%d) as usize][(u-1) as usize][(v-1) as usize] = STOP_VALUE;
        uvw_map[(i%d) as usize][(v-1) as usize][(u-1) as usize] = STOP_VALUE;
    }

    let mut cnt = 0;
    
    // loop
    loop {
        cnt += 1;
        // 乱数
        let random_way: usize = rand::thread_rng().gen_range(0, m);
        let random_u: usize = uvw[random_way].0 - 1;
        let random_v: usize = uvw[random_way].1 - 1;
        let random_day: usize = rand::thread_rng().gen_range(0, d);
        let now_day = now_ans[random_way];
        let fut_day = random_day;

        // fut_ans
        let mut fut_ans = now_ans.clone();
        fut_ans[random_way] = fut_day;

        // 合計StopRoot
        let mut now_stop_root = 0;
        let mut fut_stop_root = 0;
        
        for i in 0..n {
            now_stop_root += uvw_map[now_day][random_u][i] + uvw_map[now_day][random_v][i];
            fut_stop_root += uvw_map[fut_day][random_u][i] + uvw_map[fut_day][random_v][i];
        }
        
        if fut_stop_root < now_stop_root {
            now_ans[random_way] = fut_day;
            uvw_map[now_day][random_u][random_v] = 0;
            uvw_map[fut_day][random_u][random_v] = STOP_VALUE;
        }
    
        // get end time
        let end = start.elapsed();
        let end_sec = end.as_secs_f64();

        if cnt > 10000 {
            println!("{}", end_sec);
            break;
        }
    }
    
    // output
    for i in &now_ans {
        print!("{} ", i+1)
    }
}
