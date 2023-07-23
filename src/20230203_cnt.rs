extern crate rand;
use rand::Rng;
use proconio::input;
// use std::time::{Instant, Duration};

fn main() {
    // get start time
    // 

    // input
    input! {
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        uvw: [(usize, usize, usize); m],
        _xy: [[f64; 2]; n],
    }

    // 初期設定
    let mut root_weight = vec![vec![1_000_000_000; n]; n];
    let mut short_roots = vec![vec![vec![]; n]; n];
    
    // loop内
    const STOP_VALUE: usize = 1;
    let mut daily_uvw_map = vec![vec![vec![0; n]; n]; d];
    let mut conect_roots = vec![];

    for (u, v, w) in &uvw {
        root_weight[(u-1) as usize][(v-1) as usize] = *w;
        root_weight[(v-1) as usize][(u-1) as usize] = *w;
    }

    // day_num
    let mut day_num = vec![0; d];
    // now_ans
    let mut now_ans = vec![0; m];
    for i in 0..m {
        let u = uvw[i].0;
        let v = uvw[i].1;

        let short_root = short_roots[(u-1) as usize][(v-1) as usize].clone();
        let mut random_seed = vec![];
        for i in 0..d {
            random_seed.push(i);
        }
        
        for root in short_root {
            random_seed.retain(|&x| x != root);
        }

        if random_seed.len() == 0 {
            for i in 0..d {
                random_seed.push(i);
            }
        } 

        
        let mut min_v = 100000000;
        for i in &random_seed {
            if day_num[*i] < min_v {
                min_v = day_num[*i];
            }
        }
        let mut day = 1_000_000_000;
        for i in &random_seed {
            if day_num[*i] == min_v {
                day = *i;
            }
        }
        day_num[day] += 1;
        daily_uvw_map[(day) as usize][(u-1) as usize][(v-1) as usize] = STOP_VALUE;
        daily_uvw_map[(day) as usize][(v-1) as usize][(u-1) as usize] = STOP_VALUE;

        let mut stop_days_u = vec![];
        let mut stop_days_v = vec![];
        for j in 0..n {
            if root_weight[(u-1) as usize][j] != 1_000_000_000 {
                stop_days_u.push(j);
            }
            
            if root_weight[(v-1) as usize][j] != 1_000_000_000 {
                stop_days_v.push(j);
            }
        } 

        let mut rw = vec![];
        for u_i in &stop_days_u {
            for v_i in &stop_days_v {
                if u_i == v_i {
                    let sum_w: usize = root_weight[*(u_i)][(u-1) as usize]+root_weight[*(u_i)][(v-1) as usize];
                    rw.push((u_i, sum_w));
                }
            }
        }


        if rw.len() != 0 {
            rw.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
            let halfway_point: usize = *rw[0].0;
            short_roots[halfway_point][(u-1) as usize].push(day);
            short_roots[halfway_point][(v-1) as usize].push(day);
            short_roots[(u-1) as usize][halfway_point].push(day);
            short_roots[(v-1) as usize][halfway_point].push(day);
        } else {
            conect_roots.push((u, v, i));
        }
        now_ans[i] = (day) as usize;
    }
    
    
    // loop
    let mut cnt = 0;
    loop {
        cnt += 1;
        // 乱数
        for (u, v, num) in &conect_roots {
            let mut nc_days = vec![];
            for nc_day in 0..d {
                let mut sum_stop_cnt = 0;
                for i in 0..n {
                    sum_stop_cnt += daily_uvw_map[(nc_day) as usize][(u-1) as usize][(i) as usize];
                    sum_stop_cnt += daily_uvw_map[(nc_day) as usize][(v-1) as usize][(i) as usize];
                }
                if sum_stop_cnt == 0 {
                    nc_days.push(nc_day);
                }
            }

            let short_root = short_roots[(u-1) as usize][(v-1) as usize].clone();
            for day in short_root {
                nc_days.retain(|&x| x != day);
            }
            
            if nc_days.len() != 0 {
                let index = rand::thread_rng().gen_range(0, nc_days.len());
                if day_num[(nc_days[(index) as usize as usize])] < k {
                    day_num[(now_ans[*num]) as usize] -= 1;
                    day_num[(nc_days[(index) as usize]) as usize] += 1;
                    daily_uvw_map[(now_ans[*num]) as usize][(uvw[*num].0-1) as usize][(uvw[*num].1-1) as usize] = 0;
                    daily_uvw_map[(now_ans[*num]) as usize][(uvw[*num].1-1) as usize][(uvw[*num].0-1) as usize] = 0;
                    daily_uvw_map[(nc_days[(index) as usize]) as usize][(uvw[*num].0-1) as usize][(uvw[*num].1-1) as usize] = 1;
                    daily_uvw_map[(nc_days[(index) as usize]) as usize][(uvw[*num].1-1) as usize][(uvw[*num].0-1) as usize] = 1;
                    now_ans[*num] = (nc_days[(index) as usize]) as usize;
                }
                
            }
        }
    
        // get end time
        if cnt > 5 {
            break;
        }
    }
    
    // output
    for i in &now_ans {
        print!("{} ", i+1)
    }
}
