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
        k: usize,
        uvw: [(usize, usize, usize); m],
        xy: [[f64; 2]; n],
    }

    // uvw_map
    const STOP_VALUE: usize = 1_000_000_000;
    let mut uvw_map = vec![vec![STOP_VALUE; n]; n];
    for (u, v, w) in &uvw {
        uvw_map[(u-1) as usize][(v-1) as usize] = *w;
    }

    // now_ans
    let mut now_ans = vec![0; m];
    for i in 0..m {
        now_ans[i] = i % d
    }

    let mut cnt = 0;
    
    // loop
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

        // 合計共分散
        let mut now_sum_covariance = 0.0;
        let mut fut_sum_covariance = 0.0;
        let mut cnt_k = 0;

        for day in [now_day, fut_day] {
            let mut now_coordinate_x = vec![];
            let mut fut_coordinate_x = vec![];
            let mut now_coordinate_y = vec![];
            let mut fut_coordinate_y = vec![];

            // 通行止め
            for i in 0..m {
                if now_ans[i] == day {
                    let now_u = &uvw[i].0;
                    let now_v = &uvw[i].1;

                    let now_ux = xy[(now_u-1) as usize][0];
                    let now_uy = xy[(now_u-1) as usize][1];
                    let now_vx = xy[(now_v-1) as usize][0];
                    let now_vy = xy[(now_v-1) as usize][1];
                    
                    let now_x = (now_ux + now_vx) / 2.0;
                    let now_y = (now_uy + now_vy) / 2.0;
                    
                    now_coordinate_x.push(now_x);
                    now_coordinate_y.push(now_y);
                }

                if fut_ans[i] == day {
                    // 増加量を見る
                    if day == fut_day {
                        cnt_k += 1;
                    }
                    let fut_u = &uvw[i].0;
                    let fut_v = &uvw[i].1;

                    let fut_ux = xy[(fut_u-1) as usize][0];
                    let fut_uy = xy[(fut_u-1) as usize][1];
                    let fut_vx = xy[(fut_v-1) as usize][0];
                    let fut_vy = xy[(fut_v-1) as usize][1];
                    
                    let fut_x = (fut_ux + fut_vx) / 2.0;
                    let fut_y = (fut_uy + fut_vy) / 2.0;
                    
                    fut_coordinate_x.push(fut_x);
                    fut_coordinate_y.push(fut_y);
                }
            }

            // now weight
            let mut now_covariance = 0.0;
            let now_len = now_coordinate_x.len() as usize;
            let avg_now_x = now_coordinate_x.iter().sum::<f64>() / now_coordinate_x.len() as f64;
            let avg_now_y = now_coordinate_y.iter().sum::<f64>() / now_coordinate_y.len() as f64;
            for i in 0..now_len {
                now_covariance += (now_coordinate_x[i] - avg_now_x)*(now_coordinate_y[i] - avg_now_y);
            }
            now_covariance /= now_len as f64;
            now_sum_covariance += now_covariance;

            // fut weight
            let mut fut_covariance = 0.0;
            let fut_len = fut_coordinate_x.len() as usize;
            let avg_fut_x = fut_coordinate_x.iter().sum::<f64>() / fut_coordinate_x.len() as f64;
            let avg_fut_y = fut_coordinate_y.iter().sum::<f64>() / fut_coordinate_y.len() as f64;
            for i in 0..fut_len {
                fut_covariance += (fut_coordinate_x[i] - avg_fut_x)*(fut_coordinate_y[i] - avg_fut_y);
            }
            fut_covariance /= fut_len as f64;
            fut_sum_covariance += fut_covariance;
        }
        if fut_sum_covariance > now_sum_covariance && cnt_k <= k{
            now_ans[random_way] = fut_day;
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
